use crate::config::Config;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::path::PathBuf;
use std::fs;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistEntry {
    pub path: PathBuf,
    pub title: String,
    pub duration: f64,
    pub position: f64,
}

pub struct Playlist {
    entries: VecDeque<PlaylistEntry>,
    current_index: usize,
    repeat_mode: RepeatMode,
    shuffle: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepeatMode {
    None,
    One,
    All,
}

impl RepeatMode {
    pub fn next(&self) -> Self {
        match self {
            RepeatMode::None => RepeatMode::One,
            RepeatMode::One => RepeatMode::All,
            RepeatMode::All => RepeatMode::None,
        }
    }
}

impl Playlist {
    pub fn new() -> Self {
        Playlist {
            entries: VecDeque::new(),
            current_index: 0,
            repeat_mode: RepeatMode::None,
            shuffle: false,
        }
    }

    pub fn add_entry(&mut self, entry: PlaylistEntry) {
        self.entries.push_back(entry);
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.current_index = 0;
    }

    pub fn current(&self) -> Option<&PlaylistEntry> {
        self.entries.get(self.current_index)
    }

    pub fn next(&mut self) -> Option<&PlaylistEntry> {
        if self.entries.is_empty() {
            return None;
        }

        match self.repeat_mode {
            RepeatMode::One => self.entries.get(self.current_index),
            RepeatMode::All => {
                self.current_index = (self.current_index + 1) % self.entries.len();
                self.entries.get(self.current_index)
            }
            RepeatMode::None => {
                if self.current_index + 1 < self.entries.len() {
                    self.current_index += 1;
                    self.entries.get(self.current_index)
                } else {
                    None
                }
            }
        }
    }

    pub fn previous(&mut self) -> Option<&PlaylistEntry> {
        if self.entries.is_empty() {
            return None;
        }

        if self.current_index > 0 {
            self.current_index -= 1;
        }
        self.entries.get(self.current_index)
    }

    pub fn set_repeat_mode(&mut self, mode: RepeatMode) {
        self.repeat_mode = mode;
    }

    pub fn repeat_mode(&self) -> RepeatMode {
        self.repeat_mode
    }

    pub fn entries(&self) -> &VecDeque<PlaylistEntry> {
        &self.entries
    }

    pub fn entries_mut(&mut self) -> &mut VecDeque<PlaylistEntry> {
        &mut self.entries
    }

    pub fn current_index(&self) -> usize {
        self.current_index
    }

    pub fn set_current_index(&mut self, index: usize) -> Option<&PlaylistEntry> {
        if index < self.entries.len() {
            self.current_index = index;
            self.entries.get(self.current_index)
        } else {
            None
        }
    }

    pub fn save_to_file(&self, path: &PathBuf) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.entries)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(path: &PathBuf) -> Result<Self> {
        let json = fs::read_to_string(path)?;
        let entries: VecDeque<PlaylistEntry> = serde_json::from_str(&json)?;
        Ok(Playlist {
            entries,
            current_index: 0,
            repeat_mode: RepeatMode::None,
            shuffle: false,
        })
    }
}
