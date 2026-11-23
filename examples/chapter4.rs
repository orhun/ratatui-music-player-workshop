#![allow(unused_imports)] // Keep later chapter imports ready from the start.

#[path = "../src/music.rs"]
#[allow(dead_code)] // Playback controls are added in the next chapter.
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
    tracks: Vec<Track>,
    table_state: TableState,
}

impl App {
    pub fn new() -> color_eyre::Result<Self> {
        let tracks = scan_tracks("music")?;
        let mut table_state = TableState::default();
        if !tracks.is_empty() {
            table_state.select(Some(0));
        }
        Ok(Self {
            running: true,
            tracks,
            table_state,
        })
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Split the area into a header, playlist and details, progress gauge and footer.
    fn render(&mut self, frame: &mut Frame) {
        let [header_area, content_area, progress_area, footer_area] =
            Layout::vertical([Length(3), Min(0), Length(3), Length(3)]).areas(frame.area());
        let [playlist_area, details_area] =
            Layout::horizontal([Percentage(65), Fill(1)]).areas(content_area);

        frame.render_widget(
            Paragraph::new("♫ Ratatune ♫")
                .block(
                    Block::bordered()
                        .border_type(BorderType::Rounded)
                        .border_style(tailwind::GRAY.c700)
                        .style(Style::new().bg(tailwind::GRAY.c900)),
                )
                .alignment(Alignment::Center),
            header_area,
        );

        let rows = self.tracks.iter().map(|track| {
            Row::new(vec![
                Cell::from(track.title.as_str()),
                Cell::from(track.artist.as_deref().unwrap_or("Unknown artist")),
                Cell::from(format_duration(track.duration)),
            ])
        });
        let header = Row::new(vec!["Title", "Artist", "Length"])
            .style(Style::new().fg(tailwind::YELLOW.c200).bold());
        let table = Table::new(rows, [Percentage(48), Percentage(38), Length(8)])
            .header(header)
            .row_highlight_style(Style::new().bg(tailwind::GRAY.c700).fg(tailwind::GRAY.c100));
        frame.render_stateful_widget(
            table.block(Self::create_pane("Playlist")),
            playlist_area,
            &mut self.table_state,
        );
        if self.tracks.is_empty() {
            frame.render_widget(
                Paragraph::new("Add MP3 files to music/").centered().dim(),
                playlist_area,
            );
        }

        let details = self
            .table_state
            .selected()
            .and_then(|index| self.tracks.get(index))
            .map_or_else(
                || "Select a track to see its details".to_string(),
                |track| {
                    format!(
                        "{}\n\nArtist: {}\nAlbum: {}\nLength: {}",
                        track.title,
                        track.artist.as_deref().unwrap_or("Unknown artist"),
                        track.album.as_deref().unwrap_or("Unknown album"),
                        format_duration(track.duration)
                    )
                },
            );
        frame.render_widget(
            Paragraph::new(details)
                .block(Self::create_pane("Track Details"))
                .wrap(Wrap { trim: true }),
            details_area,
        );

        // Use sample values here; Chapter 5 connects the gauge to live playback.
        let position = Duration::from_secs(42);
        let duration = Duration::from_secs(195);
        let ratio = position.as_secs_f64() / duration.as_secs_f64();
        frame.render_widget(
            Gauge::default()
                .block(Self::create_pane("Progress"))
                .gauge_style(Style::new().fg(tailwind::GREEN.c400))
                .ratio(ratio)
                .label(format!(
                    "{} / {}",
                    format_duration(position),
                    format_duration(duration)
                )),
            progress_area,
        );

        frame.render_widget(
            Paragraph::new("↑/↓ or j/k select  q quit")
                .block(Self::create_pane("Controls"))
                .alignment(Alignment::Center),
            footer_area,
        );
    }

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
            Event::Mouse(_) | Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c' | 'C')) => self.running = false,
            (_, KeyCode::Down | KeyCode::Char('j')) => self.select_next(),
            (_, KeyCode::Up | KeyCode::Char('k')) => self.select_previous(),
            _ => {}
        }
    }

    fn select_next(&mut self) {
        if !self.tracks.is_empty() {
            let selected = self.table_state.selected().unwrap_or(0);
            self.table_state
                .select(Some((selected + 1) % self.tracks.len()));
        }
    }

    fn select_previous(&mut self) {
        if !self.tracks.is_empty() {
            let selected = self.table_state.selected().unwrap_or(0);
            self.table_state
                .select(Some((selected + self.tracks.len() - 1) % self.tracks.len()));
        }
    }
}
