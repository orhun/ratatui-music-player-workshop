use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    time::Duration,
};

use color_eyre::eyre::{Result, WrapErr};
use lofty::{
    file::{AudioFile, TaggedFileExt},
    read_from_path,
    tag::Accessor,
};
use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player as RodioPlayer, Source};

/// One local MP3 file and the metadata used by Ratatune's playlist.
#[derive(Clone, Debug)]
pub struct Track {
    pub path: PathBuf,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: Duration,
}

/// Read MP3 files from a directory and use their tags when available.
pub fn scan_tracks(directory: impl AsRef<Path>) -> Result<Vec<Track>> {
    let directory = directory.as_ref();
    let entries = fs::read_dir(directory)
        .wrap_err_with(|| format!("Could not read music folder {}", directory.display()))?;
    let mut tracks = Vec::new();

    for entry in entries {
        let path = entry?.path();
        if !path.is_file()
            || !path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("mp3"))
        {
            continue;
        }

        let tagged_file = read_from_path(&path).ok();
        let tag = tagged_file
            .as_ref()
            .and_then(|file| file.primary_tag().or_else(|| file.first_tag()));
        let fallback_title = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("Unknown track")
            .to_string();
        let title = tag
            .and_then(|tag| tag.title())
            .map(|title| title.into_owned())
            .unwrap_or(fallback_title);
        let artist = tag
            .and_then(|tag| tag.artist())
            .map(|artist| artist.into_owned());
        let album = tag
            .and_then(|tag| tag.album())
            .map(|album| album.into_owned());
        let duration = tagged_file
            .as_ref()
            .map(|file| file.properties().duration())
            .unwrap_or_default();

        tracks.push(Track {
            path,
            title,
            artist,
            album,
            duration,
        });
    }

    tracks.sort_by(|left, right| left.title.to_lowercase().cmp(&right.title.to_lowercase()));
    Ok(tracks)
}

/// A small wrapper around Rodio so the UI does not need to manage audio devices.
pub struct Player {
    // Keep the output stream alive for as long as the player is in use.
    _stream: MixerDeviceSink,
    player: RodioPlayer,
    duration: Duration,
}

impl Player {
    pub fn new() -> Result<Self> {
        let stream = open_audio_output()?;
        let player = RodioPlayer::connect_new(stream.mixer());
        Ok(Self {
            _stream: stream,
            player,
            duration: Duration::ZERO,
        })
    }

    /// Start playing one track, replacing the current track.
    pub fn play(&mut self, track: &Track) -> Result<()> {
        self.player.stop();
        let file = File::open(&track.path)
            .wrap_err_with(|| format!("Could not open {}", track.path.display()))?;
        let decoder = Decoder::try_from(file)
            .wrap_err_with(|| format!("Could not decode {}", track.path.display()))?;
        self.duration = decoder.total_duration().unwrap_or(track.duration);
        self.player.append(decoder);
        self.player.play();
        Ok(())
    }

    pub fn pause(&self) {
        self.player.pause();
    }

    pub fn resume(&self) {
        self.player.play();
    }

    pub fn is_paused(&self) -> bool {
        self.player.is_paused()
    }

    pub fn position(&self) -> Duration {
        self.player.get_pos()
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }

    pub fn is_empty(&self) -> bool {
        self.player.empty()
    }

    pub fn set_volume(&self, volume: f32) {
        self.player.set_volume(volume.clamp(0.0, 1.0));
    }
}

fn open_audio_output() -> Result<MixerDeviceSink> {
    #[cfg(target_os = "linux")]
    {
        use cpal::traits::HostTrait;

        // Prefer PulseAudio (also supported by PipeWire) so audio follows the
        // user's selected output device instead of opening raw ALSA hardware.
        if let Ok(host) = cpal::host_from_id(cpal::HostId::PulseAudio) {
            if let Some(device) = host.default_output_device() {
                if let Ok(builder) = DeviceSinkBuilder::from_device(device) {
                    if let Ok(stream) = builder.open_sink_or_fallback() {
                        return Ok(stream);
                    }
                }
            }
        }
    }

    DeviceSinkBuilder::open_default_sink()
        .wrap_err("Could not open the default audio output device")
}

/// Format a playback position as minutes and seconds.
pub fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    format!("{}:{:02}", seconds / 60, seconds % 60)
}
