mod music;

use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use music::{Player, Track, format_duration, scan_tracks};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint::*, Layout, Rect},
    style::{Style, Stylize, palette::tailwind},
    text::{Line, Span},
    widgets::{Block, BorderType, Cell, Gauge, Paragraph, Row, Table, TableState, Wrap},
};
use ratatui_textarea::TextArea;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().and_then(|app| app.run(terminal));
    ratatui::restore();
    result
}

/// The main application which holds the state and logic of the application.
pub struct App {
    /// Is the application running?
    running: bool,
    /// The music files discovered in the local music folder.
    tracks: Vec<Track>,
    /// The current state of the playlist table.
    table_state: TableState,
    /// The audio output and playback controls.
    player: Option<Player>,
    /// Index of the track currently being played.
    playing: Option<usize>,
    /// Playback volume from 0.0 (muted) to 1.0 (full volume).
    volume: f32,
    /// Is the user searching the playlist?
    searching: bool,
    /// Single-line search editor.
    search_input: TextArea<'static>,
    /// Search text used to filter the playlist.
    query: String,
    /// A short message shown in the footer.
    message: String,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> color_eyre::Result<Self> {
        let tracks = scan_tracks("music")?;
        let player = match Player::new() {
            Ok(player) => {
                player.set_volume(0.5);
                Some(player)
            }
            Err(error) => {
                // Keep the playlist usable even if this computer has no audio output.
                eprintln!("Could not open an audio device: {error}");
                None
            }
        };
        let message = if tracks.is_empty() {
            "Drop some MP3 files into music/ to get started".to_string()
        } else if player.is_none() {
            "Audio output unavailable; playlist browsing still works".to_string()
        } else {
            "Ready to play".to_string()
        };

        let mut table_state = TableState::default();
        if !tracks.is_empty() {
            table_state.select(Some(0));
        }

        Ok(Self {
            running: true,
            tracks,
            table_state,
            player,
            playing: None,
            volume: 0.5,
            searching: false,
            search_input: TextArea::default(),
            query: String::new(),
            message,
        })
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
            self.tick();
        }
        Ok(())
    }

    /// Clear the current track from the UI after playback finishes.
    fn tick(&mut self) {
        if self.player.as_ref().is_some_and(Player::is_empty) && self.playing.is_some() {
            self.playing = None;
            self.message = "Track finished".to_string();
        }
    }

    /// Renders the user interface.
    fn render(&mut self, frame: &mut Frame) {
        let [header_area, body_area, footer_area] =
            Layout::vertical([Length(3), Min(0), Length(3)]).areas(frame.area());
        let [playlist_area, details_area] =
            Layout::horizontal([Percentage(62), Fill(1)]).areas(body_area);
        let [now_playing_area, progress_area] =
            Layout::vertical([Fill(1), Length(5)]).areas(details_area);

        self.render_header(frame, header_area);
        self.render_playlist(frame, playlist_area);
        self.render_now_playing(frame, now_playing_area);
        self.render_progress(frame, progress_area);
        self.render_footer(frame, footer_area);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let title = Line::from("♫ Ratatune ♫")
            .bold()
            .fg(tailwind::BLUE.c200)
            .centered();
        frame.render_widget(
            Paragraph::new("An MP3 player built with Ratatui")
                .block(Self::create_pane("Ratatune").title(title))
                .alignment(Alignment::Center),
            area,
        );
    }

    fn render_playlist(&mut self, frame: &mut Frame, area: Rect) {
        let visible = self.visible_tracks();
        let rows = visible.iter().map(|&index| {
            let track = &self.tracks[index];
            let playing = self.playing == Some(index);
            let icon = if playing { "♫" } else { " " };
            Row::new(vec![
                Cell::from(icon).fg(tailwind::GREEN.c300),
                Cell::from(track.title.as_str()),
                Cell::from(track.artist.as_deref().unwrap_or("Unknown artist")),
                Cell::from(format_duration(track.duration)),
            ])
        });
        let header = Row::new(vec!["", "Title", "Artist", "Length"])
            .style(Style::new().fg(tailwind::YELLOW.c200).bold());
        let widths = [Length(2), Percentage(42), Percentage(36), Length(8)];
        let table = Table::new(rows, widths)
            .header(header)
            .row_highlight_style(Style::new().bg(tailwind::GRAY.c700).fg(tailwind::GRAY.c100));
        frame.render_stateful_widget(
            table.block(Self::create_pane("Playlist")),
            area,
            &mut self.table_state,
        );
        if visible.is_empty() {
            frame.render_widget(
                Paragraph::new("No MP3 files found in music/")
                    .centered()
                    .dim(),
                area,
            );
        }
    }

    fn render_now_playing(&self, frame: &mut Frame, area: Rect) {
        let content = self
            .playing
            .and_then(|index| self.tracks.get(index))
            .map_or_else(
                || "Nothing is playing yet\n\nSelect a track and press Enter".to_string(),
                |track| {
                    format!(
                        "♫  {}\n\n{}\n\n{}",
                        track.title,
                        track.artist.as_deref().unwrap_or("Unknown artist"),
                        track.album.as_deref().unwrap_or("Unknown album")
                    )
                },
            );
        frame.render_widget(
            Paragraph::new(content)
                .block(Self::create_pane("Now Playing"))
                .wrap(Wrap { trim: true }),
            area,
        );
    }

    fn render_progress(&self, frame: &mut Frame, area: Rect) {
        let position = if self.playing.is_some() {
            self.player
                .as_ref()
                .map_or(Duration::ZERO, Player::position)
        } else {
            Duration::ZERO
        };
        let duration = if self.playing.is_some() {
            self.player
                .as_ref()
                .map_or(Duration::ZERO, Player::duration)
        } else {
            Duration::ZERO
        };
        let ratio = if duration.is_zero() {
            0.0
        } else {
            (position.as_secs_f64() / duration.as_secs_f64()).clamp(0.0, 1.0)
        };
        let paused = self.playing.is_none() || self.player.as_ref().is_none_or(Player::is_paused);
        let icon = if paused { "▶" } else { "⏸" };
        let label = format!(
            "{icon}  {} / {}",
            format_duration(position),
            format_duration(duration)
        );
        frame.render_widget(
            Gauge::default()
                .block(Self::create_pane("Progress"))
                .gauge_style(Style::new().fg(tailwind::GREEN.c400))
                .ratio(ratio)
                .label(label),
            area,
        );
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        if self.searching {
            let block = Self::create_pane("Search (Enter to finish, Esc to clear)");
            let input_area = block.inner(area);
            frame.render_widget(block, area);
            frame.render_widget(&self.search_input, input_area);
            return;
        }

        let message = format!(
            "{}  |  ↑/↓ select  Enter play  Space pause  n next  +/- volume  / search  q quit",
            self.message
        );
        frame.render_widget(
            Paragraph::new(Line::from(vec![Span::styled(
                message,
                Style::new().fg(tailwind::GRAY.c200),
            )]))
            .block(Self::create_pane("Controls"))
            .alignment(Alignment::Center),
            area,
        );
    }

    /// Return indices of tracks matching the current search query.
    fn visible_tracks(&self) -> Vec<usize> {
        let query = self.query.to_lowercase();
        self.tracks
            .iter()
            .enumerate()
            .filter(|(_, track)| {
                query.is_empty()
                    || track.title.to_lowercase().contains(&query)
                    || track
                        .artist
                        .as_deref()
                        .is_some_and(|artist| artist.to_lowercase().contains(&query))
            })
            .map(|(index, _)| index)
            .collect()
    }

    /// Creates a bordered block with a title.
    fn create_pane(title: &str) -> Block<'_> {
        let title = Line::from_iter([
            "┤ ".fg(tailwind::GRAY.c700),
            title.fg(tailwind::BLUE.c200),
            " ├".fg(tailwind::GRAY.c700),
        ]);
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title)
            .border_style(tailwind::GRAY.c700)
            .style(Style::new().bg(tailwind::GRAY.c900))
    }

    /// Reads Crossterm events and updates the application state.
    fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
                Event::Mouse(_) | Event::Resize(_, _) => {}
                _ => {}
            }
        }
        Ok(())
    }

    /// Handles keyboard input and updates the app or player state.
    fn on_key_event(&mut self, key: KeyEvent) {
        if self.searching {
            self.on_search_key(key);
            return;
        }

        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c' | 'C')) => self.quit(),
            (_, KeyCode::Down | KeyCode::Char('j')) => self.select_next(),
            (_, KeyCode::Up | KeyCode::Char('k')) => self.select_previous(),
            (_, KeyCode::Enter) => self.play_selected(),
            (_, KeyCode::Char(' ')) => self.toggle_playback(),
            (_, KeyCode::Char('n')) => self.play_next(),
            (_, KeyCode::Char('+') | KeyCode::Char('=')) => self.adjust_volume(0.1),
            (_, KeyCode::Char('-')) => self.adjust_volume(-0.1),
            (_, KeyCode::Char('/')) => {
                self.searching = true;
                self.search_input = TextArea::default();
                self.query.clear();
                self.table_state
                    .select((!self.tracks.is_empty()).then_some(0));
            }
            _ => {}
        }
    }

    fn on_search_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.query.clear();
                self.searching = false;
                self.search_input = TextArea::default();
                self.table_state
                    .select((!self.tracks.is_empty()).then_some(0));
            }
            KeyCode::Enter => self.searching = false,
            _ => {
                self.search_input.input(key);
                self.query = self
                    .search_input
                    .lines()
                    .first()
                    .cloned()
                    .unwrap_or_default();
                self.table_state
                    .select((!self.visible_tracks().is_empty()).then_some(0));
            }
        }
    }

    fn select_next(&mut self) {
        let count = self.visible_tracks().len();
        if count > 0 {
            let selected = self.table_state.selected().unwrap_or(0);
            self.table_state.select(Some((selected + 1) % count));
        }
    }

    fn select_previous(&mut self) {
        let count = self.visible_tracks().len();
        if count > 0 {
            let selected = self.table_state.selected().unwrap_or(0);
            self.table_state
                .select(Some((selected + count - 1) % count));
        }
    }

    fn play_selected(&mut self) {
        let Some(index) = self
            .table_state
            .selected()
            .and_then(|selected| self.visible_tracks().get(selected).copied())
        else {
            self.message = "No track selected".to_string();
            return;
        };
        self.play_track(index);
    }

    fn play_track(&mut self, index: usize) {
        self.playing = None;
        let Some(player) = &mut self.player else {
            self.message = "Could not open an audio output device".to_string();
            return;
        };
        match player.play(&self.tracks[index]) {
            Ok(()) => {
                self.playing = Some(index);
                self.message = format!("Playing {}", self.tracks[index].title);
            }
            Err(error) => self.message = format!("Could not play track: {error}"),
        }
    }

    fn toggle_playback(&mut self) {
        if self.playing.is_none() {
            self.play_selected();
            return;
        }
        if let Some(player) = &self.player {
            if player.is_paused() {
                player.resume();
                self.message = "Playback resumed".to_string();
            } else {
                player.pause();
                self.message = "Playback paused".to_string();
            }
        }
    }

    fn play_next(&mut self) {
        let visible = self.visible_tracks();
        if visible.is_empty() {
            return;
        }
        let selected = self.table_state.selected().unwrap_or(0);
        let next_position = self
            .playing
            .and_then(|playing| visible.iter().position(|&index| index == playing))
            .map_or((selected + 1) % visible.len(), |position| {
                (position + 1) % visible.len()
            });
        self.table_state.select(Some(next_position));
        self.play_track(visible[next_position]);
    }

    fn adjust_volume(&mut self, change: f32) {
        self.volume = (self.volume + change).clamp(0.0, 1.0);
        if let Some(player) = &self.player {
            player.set_volume(self.volume);
        }
        self.message = format!("Volume: {:.0}%", self.volume * 100.0);
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
