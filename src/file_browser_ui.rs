use crate::config::{ColorMode, RenderMode, Config, QualityPreset};
use crate::file_browser::FileBrowser;
use crate::playlist::{Playlist, PlaylistEntry, RepeatMode};
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Gauge, Wrap},
    Terminal,
};
use std::io;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuMode {
    FileBrowser,
    Playlist,
    Settings,
    Help,
}

pub struct FileBrowserUI {
    browser: FileBrowser,
    playlist: Playlist,
    config: Config,
    menu_mode: MenuMode,
    message: String,
    message_timer: u32,
    selected_file: Option<PathBuf>,
}

impl FileBrowserUI {
    pub fn new(config: Config) -> Result<Self> {
        let browser = FileBrowser::new(None)?;
        Ok(FileBrowserUI {
            browser,
            playlist: Playlist::new(),
            config,
            menu_mode: MenuMode::FileBrowser,
            message: String::new(),
            message_timer: 0,
            selected_file: None,
        })
    }

    pub fn run(&mut self) -> Result<Option<PathBuf>> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        let result = self.event_loop(&mut terminal);

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        result
    }

    fn event_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<Option<PathBuf>> {
        loop {
            terminal.draw(|f| self.render(f))?;

            if event::poll(Duration::from_millis(250))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            return Ok(self.selected_file.clone());
                        }
                        KeyCode::Char('h') | KeyCode::F(1) => {
                            self.menu_mode = MenuMode::Help;
                        }
                        KeyCode::Tab => {
                            self.menu_mode = match self.menu_mode {
                                MenuMode::FileBrowser => MenuMode::Playlist,
                                MenuMode::Playlist => MenuMode::Settings,
                                MenuMode::Settings => MenuMode::FileBrowser,
                                MenuMode::Help => MenuMode::FileBrowser,
                            };
                        }
                        _ if self.menu_mode == MenuMode::Help => {
                            self.menu_mode = MenuMode::FileBrowser;
                        }
                        _ if self.menu_mode == MenuMode::FileBrowser => {
                            self.handle_browser_input(key.code)?;
                        }
                        _ if self.menu_mode == MenuMode::Playlist => {
                            self.handle_playlist_input(key.code)?;
                        }
                        _ if self.menu_mode == MenuMode::Settings => {
                            self.handle_settings_input(key.code);
                        }
                        _ => {}
                    }
                }
            }

            if self.message_timer > 0 {
                self.message_timer = self.message_timer.saturating_sub(1);
            }
        }
    }

    fn handle_browser_input(&mut self, code: KeyCode) -> Result<()> {
        match code {
            KeyCode::Down => {
                self.browser.move_down();
            }
            KeyCode::Up => {
                self.browser.move_up();
            }
            KeyCode::Enter => {
                if let Some(file) = self.browser.get_selected() {
                    if file.name.contains("Parent Directory") {
                        if let Some(parent) = self.browser.current_path().parent() {
                            self.browser.navigate_to(parent.to_path_buf())?;
                        }
                    } else if file.path.is_dir() {
                        self.browser.navigate_to(file.path.clone())?;
                    } else if file.is_video() {
                        self.selected_file = Some(file.path.clone());
                        return Ok(());
                    }
                }
            }
            KeyCode::Char('a') => {
                if let Some(file) = self.browser.get_selected() {
                    if file.is_video() {
                        let entry = PlaylistEntry {
                            path: file.path.clone(),
                            title: file.name.clone(),
                            duration: 0.0,
                            position: 0.0,
                        };
                        self.playlist.add_entry(entry);
                        self.message = format!("✓ Added: {}", file.name);
                        self.message_timer = 100;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_playlist_input(&mut self, code: KeyCode) -> Result<()> {
        match code {
            KeyCode::Down => {
                let len = self.playlist.entries().len();
                if len > 0 {
                    let next_idx = (self.playlist.current_index() + 1) % len;
                    self.playlist.set_current_index(next_idx);
                }
            }
            KeyCode::Up => {
                let len = self.playlist.entries().len();
                if len > 0 {
                    let prev_idx = if self.playlist.current_index() == 0 {
                        len - 1
                    } else {
                        self.playlist.current_index() - 1
                    };
                    self.playlist.set_current_index(prev_idx);
                }
            }
            KeyCode::Delete => {
                if !self.playlist.entries().is_empty() {
                    self.playlist.entries_mut().remove(self.playlist.current_index());
                    self.message = "✓ Removed from playlist".to_string();
                    self.message_timer = 100;
                }
            }
            KeyCode::Char('c') => {
                self.playlist.clear();
                self.message = "✓ Playlist cleared".to_string();
                self.message_timer = 100;
            }
            KeyCode::Enter => {
                if let Some(entry) = self.playlist.current() {
                    if entry.path.exists() {
                        self.selected_file = Some(entry.path.clone());
                        return Ok(());
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_settings_input(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('r') => {
                self.config.render_mode = self.config.render_mode.next();
                self.message = format!("▶ Render: {:?}", self.config.render_mode);
                self.message_timer = 100;
            }
            KeyCode::Char('c') => {
                self.config.color_mode = self.config.color_mode.next();
                self.message = format!("▶ Color: {:?}", self.config.color_mode);
                self.message_timer = 100;
            }
            KeyCode::Char('q') => {
                self.config.quality_preset = self.config.quality_preset.next();
                self.message = format!("▶ Quality: {:?}", self.config.quality_preset);
                self.message_timer = 100;
            }
            KeyCode::Char('d') => {
                self.config.dithering = !self.config.dithering;
                self.message = format!("▶ Dithering: {}", if self.config.dithering { "On" } else { "Off" });
                self.message_timer = 100;
            }
            KeyCode::Char('h') => {
                self.config.hardware_decode = !self.config.hardware_decode;
                self.message = format!("▶ HW Decode: {}", if self.config.hardware_decode { "On" } else { "Off" });
                self.message_timer = 100;
            }
            _ => {}
        }
    }

    fn render(&self, f: &mut ratatui::Frame<CrosstermBackend<io::Stdout>>) {
        match self.menu_mode {
            MenuMode::Help => self.render_help(f),
            MenuMode::FileBrowser | MenuMode::Playlist => self.render_browser_and_playlist(f),
            MenuMode::Settings => self.render_settings(f),
        }

        self.render_status_bar(f);
    }

    fn render_browser_and_playlist(&self, f: &mut ratatui::Frame<CrosstermBackend<io::Stdout>>) {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(4),
            ])
            .split(size);

        self.render_title(f, chunks[0]);

        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        if self.menu_mode == MenuMode::FileBrowser {
            self.render_file_browser(f, content_chunks[0]);
            self.render_playlist_panel(f, content_chunks[1]);
        } else {
            self.render_file_browser(f, content_chunks[0]);
            self.render_playlist_focused(f, content_chunks[1]);
        }
    }

    fn render_file_browser(&self, f: &mut ratatui::Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
        let items: Vec<ListItem> = self
            .browser
            .files()
            .iter()
            .enumerate()
            .map(|(idx, file)| {
                let prefix = if idx == self.browser.selected_index() {
                    "➤ "
                } else {
                    "  "
                };
                let icon = if file.path.is_dir() {
                    "📁"
                } else {
                    "🎬"
                };
                let text = format!("{}{} {} ({})", prefix, icon, file.name, file.size_string());
                ListItem::new(text).style(
                    if idx == self.browser.selected_index() {
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    },
                )
            })
            .collect();

        let list = List::new(items)
            .block(Block::default()
                .borders(Borders::ALL)
                .title(format!(" 📂 {} ", self.browser.current_path().display()))
                .border_style(Style::default().fg(Color::Green)));

        f.render_widget(list, area);
    }

    fn render_playlist_panel(&self, f: &mut ratatui::Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
        let items: Vec<ListItem> = self
            .playlist
            .entries()
            .iter()
            .enumerate()
            .map(|(idx, entry)| {
                let prefix = if idx == self.playlist.current_index() {
                    "▶ "
                } else {
                    "  "
                };
                ListItem::new(format!("{}{}", prefix, entry.title)).style(
                    if idx == self.playlist.current_index() {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Gray)
                    },
                )
            })
            .collect();

        let list = List::new(items)
            .block(Block::default()
                .borders(Borders::ALL)
                .title(format!(" ▶ Playlist ({}) ", self.playlist.entries().len()))
                .border_style(Style::default().fg(Color::Magenta)));

        f.render_widget(list, area);
    }

    fn render_playlist_focused(&self, f: &mut ratatui::Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
        let items: Vec<ListItem> = self
            .playlist
            .entries()
            .iter()
            .enumerate()
            .map(|(idx, entry)| {
                let prefix = if idx == self.playlist.current_index() {
                    "▶ "
                } else {
                    "  "
                };
                ListItem::new(format!("{}{}", prefix, entry.title)).style(
                    if idx == self.playlist.current_index() {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Gray)
                    },
                )
            })
            .collect();

        let list = List::new(items)
            .block(Block::default()
                .borders(Borders::ALL)
                .title(format!(" ▶ Playlist ({}) - FOCUSED ", self.playlist.entries().len()))
                .border_style(Style::default().fg(Color::Yellow)));

        f.render_widget(list, area);
    }

    fn render_settings(&self, f: &mut ratatui::Frame<CrosstermBackend<io::Stdout>>) {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(4)])
            .split(size);

        self.render_title(f, chunks[0]);

        let settings_text = vec![
            Line::from(""),
            Line::from("  ⚙️  VIDEO PLAYER SETTINGS"),
            Line::from(""),
            Line::from(format!("  Render Mode: {:?}", self.config.render_mode)),
            Line::from(format!("  Color Mode: {:?}", self.config.color_mode)),
            Line::from(format!("  Quality: {:?}", self.config.quality_preset)),
            Line::from(format!("  Dithering: {}", if self.config.dithering { "✓ Enabled" } else { "✗ Disabled" })),
            Line::from(format!("  Hardware Decode: {}", if self.config.hardware_decode { "✓ Enabled" } else { "✗ Disabled" })),
            Line::from(""),
            Line::from("  Press r/c/q/d/h to toggle settings"),
            Line::from(""),
        ];

        let paragraph = Paragraph::new(settings_text)
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Left)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)));

        f.render_widget(paragraph, chunks[1]);
    }

    fn render_help(&self, f: &mut ratatui::Frame<CrosstermBackend<io::Stdout>>) {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(2)])
            .split(size);

        self.render_title(f, chunks[0]);

        let help_text = vec![
            Line::from(""),
            Line::from("🎯 NAVIGATION & PLAYBACK"),
            Line::from("  ↑/↓              Navigate up/down"),
            Line::from("  ENTER            Play selected video"),
            Line::from("  TAB              Switch between views"),
            Line::from(""),
            Line::from("📁 FILE BROWSER"),
            Line::from("  a                Add selected video to playlist"),
            Line::from(""),
            Line::from("▶ PLAYLIST"),
            Line::from("  ENTER            Play selected from playlist"),
            Line::from("  DELETE           Remove from playlist"),
            Line::from("  c                Clear entire playlist"),
            Line::from(""),
            Line::from("⚙️  SETTINGS"),
            Line::from("  r                Cycle render mode (ASCII/Block/Braille)"),
            Line::from("  c                Cycle color mode"),
            Line::from("  q                Cycle quality preset"),
            Line::from("  d                Toggle dithering"),
            Line::from("  h                Toggle hardware decode"),
            Line::from(""),
            Line::from("⏺️  GENERAL"),
            Line::from("  F1/h             Show this help"),
            Line::from("  q/ESC            Quit without playing"),
            Line::from(""),
        ];

        let paragraph = Paragraph::new(help_text)
            .style(Style::default().fg(Color::Green))
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green)));

        f.render_widget(paragraph, chunks[1]);
    }

    fn render_title(&self, f: &mut ratatui::Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
        let title = Paragraph::new("🎬 HyperTerm Video Player 🎬")
            .style(Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::BOTTOM));
        f.render_widget(title, area);
    }

    fn render_status_bar(&self, f: &mut ratatui::Frame<CrosstermBackend<io::Stdout>>) {
        let size = f.size();
        let status_area = Rect {
            x: 0,
            y: size.height.saturating_sub(1),
            width: size.width,
            height: 1,
        };

        let mode_text = match self.menu_mode {
            MenuMode::FileBrowser => "📁 Browser",
            MenuMode::Playlist => "▶ Playlist",
            MenuMode::Settings => "⚙ Settings",
            MenuMode::Help => "❓ Help",
        };

        let message_part = if self.message_timer > 0 {
            self.message.clone()
        } else {
            "TAB: Switch • F1: Help • q: Quit • ENTER: Play".to_string()
        };

        let status = format!("  {}  |  {}", mode_text, message_part);
        let paragraph = Paragraph::new(status)
            .style(Style::default().bg(Color::DarkGray).fg(Color::White));

        f.render_widget(paragraph, status_area);
    }
}

impl FileBrowserUI {
    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn get_config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    pub fn get_playlist(&self) -> &Playlist {
        &self.playlist
    }
}
