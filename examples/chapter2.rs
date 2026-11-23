#[path = "../src/music.rs"]
#[allow(dead_code)] // Later chapters connect the prepared playback controls.
mod music;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::Alignment,
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new()?.run(terminal);
    ratatui::restore();
    result
}

/// The main application which holds the playlist and UI state.
pub struct App {
    running: bool,
    tracks: Vec<music::Track>,
}

impl App {
    pub fn new() -> color_eyre::Result<Self> {
        Ok(Self {
            running: true,
            tracks: music::scan_tracks("music")?,
        })
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Render the tracks discovered in the local music folder.
    fn render(&mut self, frame: &mut Frame) {
        let tracks = self
            .tracks
            .iter()
            .enumerate()
            .map(|(index, track)| {
                format!(
                    "{:>2}.  {:<32} {:<24} {}",
                    index + 1,
                    track.title,
                    track.artist.as_deref().unwrap_or("Unknown artist"),
                    music::format_duration(track.duration)
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        let text = if tracks.is_empty() {
            "No MP3 files found. Add files to music/".to_string()
        } else {
            tracks
        };
        frame.render_widget(
            Paragraph::new(text)
                .block(Block::bordered().title(Line::from("Ratatune · Playlist").bold()))
                .alignment(Alignment::Left),
            frame.area(),
        );
    }

    fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                self.running = false;
            }
            _ => {}
        }
    }
}
