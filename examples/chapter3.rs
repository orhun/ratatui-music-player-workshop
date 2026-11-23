#[path = "../src/music.rs"]
#[allow(dead_code)] // Later chapters connect the prepared playback controls.
mod music;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint::*, Layout},
    style::{Style, Stylize, palette::tailwind},
    text::Line,
    widgets::{Block, BorderType, Paragraph},
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

    /// Split the terminal into a header, playlist, player panel, and footer.
    fn render(&mut self, frame: &mut Frame) {
        let [header_area, content_area, footer_area] =
            Layout::vertical([Length(3), Min(0), Length(3)]).areas(frame.area());
        let [playlist_area, player_area] =
            Layout::horizontal([Percentage(62), Fill(1)]).areas(content_area);
        let [now_playing_area, progress_area] =
            Layout::vertical([Fill(1), Length(5)]).areas(player_area);

        frame.render_widget(
            Paragraph::new("♫  MP3 player built with Ratatui")
                .block(Self::create_pane("Ratatune"))
                .alignment(Alignment::Center),
            header_area,
        );
        frame.render_widget(
            Paragraph::new(format!("{} tracks found in music/", self.tracks.len()))
                .block(Self::create_pane("Playlist")),
            playlist_area,
        );
        frame.render_widget(
            Paragraph::new("Select a track to start listening")
                .block(Self::create_pane("Now Playing")),
            now_playing_area,
        );
        frame.render_widget(Self::create_pane("Progress"), progress_area);
        frame.render_widget(
            Paragraph::new("↑/↓ select  Enter play  Space pause  q quit")
                .block(Self::create_pane("Controls"))
                .alignment(Alignment::Center),
            footer_area,
        );
    }

    /// Creates a bordered block with a title.
    fn create_pane(title: &str) -> Block<'_> {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title(Line::from(title).fg(tailwind::BLUE.c200).bold())
            .border_style(tailwind::GRAY.c700)
            .style(Style::new().bg(tailwind::GRAY.c900))
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
