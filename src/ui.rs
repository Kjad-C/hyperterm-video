use crate::config::{Config, QualityPreset, RenderMode, ColorMode};
use anyhow::Result;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, List, ListItem},
    Frame,
};
use std::io;

pub struct SettingsMenu {
    config: Config,
    selected_item: usize,
}

impl SettingsMenu {
    pub fn new(config: Config) -> Self {
        SettingsMenu {
            config,
            selected_item: 0,
        }
    }

    pub fn render(&self, frame: &mut Frame<CrosstermBackend<io::Stdout>>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(10),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(frame.size());

        let title = Paragraph::new("HyperTerm Video Settings")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
        frame.render_widget(title, chunks[0]);

        let items = vec![
            ListItem::new(format!(
                "Quality: {:?}",
                self.config.quality_preset
            )),
            ListItem::new(format!("Render Mode: {:?}", self.config.render_mode)),
            ListItem::new(format!("Color Mode: {:?}", self.config.color_mode)),
            ListItem::new(format!(
                "Hardware Decode: {}",
                if self.config.hardware_decode {
                    "Enabled"
                } else {
                    "Disabled"
                }
            )),
            ListItem::new(format!(
                "Dithering: {}",
                if self.config.dithering {
                    "Enabled"
                } else {
                    "Disabled"
                }
            )),
        ];

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Settings"))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");

        frame.render_widget(list, chunks[1]);

        let footer = Paragraph::new("Use arrow keys to select, Enter to confirm, Q to exit")
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(footer, chunks[2]);
    }

    pub fn next_item(&mut self) {
        self.selected_item = (self.selected_item + 1) % 5;
    }

    pub fn prev_item(&mut self) {
        if self.selected_item == 0 {
            self.selected_item = 4;
        } else {
            self.selected_item -= 1;
        }
    }

    pub fn toggle_current_item(&mut self) {
        match self.selected_item {
            0 => self.config.quality_preset = self.config.quality_preset.next(),
            1 => self.config.render_mode = self.config.render_mode.next(),
            2 => self.config.color_mode = self.config.color_mode.next(),
            3 => self.config.hardware_decode = !self.config.hardware_decode,
            4 => self.config.dithering = !self.config.dithering,
            _ => {}
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
