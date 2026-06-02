use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use anyhow::Result;
use std::fs;

const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mkv", "avi", "mov", "webm", "flv", "wmv", "m4v", "3gp", "ogv"];

#[derive(Debug, Clone)]
pub struct MediaFile {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub extension: String,
}

impl MediaFile {
    pub fn from_path(path: PathBuf) -> Result<Self> {
        let metadata = fs::metadata(&path)?;
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        Ok(MediaFile {
            path,
            name,
            size: metadata.len(),
            extension,
        })
    }

    pub fn is_video(&self) -> bool {
        VIDEO_EXTENSIONS.contains(&self.extension.as_str())
    }

    pub fn size_string(&self) -> String {
        if self.size < 1024 {
            format!("{}B", self.size)
        } else if self.size < 1024 * 1024 {
            format!("{:.2}KB", self.size as f64 / 1024.0)
        } else if self.size < 1024 * 1024 * 1024 {
            format!("{:.2}MB", self.size as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.2}GB", self.size as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }
}

pub struct FileBrowser {
    current_path: PathBuf,
    files: Vec<MediaFile>,
    selected_index: usize,
}

impl FileBrowser {
    pub fn new(start_path: Option<PathBuf>) -> Result<Self> {
        let home = std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(|_| std::env::var("USERPROFILE").map(PathBuf::from))
            .unwrap_or_else(|_| PathBuf::from("."));

        let path = start_path.unwrap_or(home);

        let mut browser = FileBrowser {
            current_path: path,
            files: Vec::new(),
            selected_index: 0,
        };

        browser.refresh()?;
        Ok(browser)
    }

    pub fn refresh(&mut self) -> Result<()> {
        self.files.clear();

        if self.current_path.parent().is_some() {
            self.files.push(MediaFile {
                path: self.current_path.parent().unwrap().to_path_buf(),
                name: ".. (Parent Directory)".to_string(),
                size: 0,
                extension: String::new(),
            });
        }

        if let Ok(entries) = fs::read_dir(&self.current_path) {
            let mut items: Vec<_> = entries
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        let path = e.path();
                        if path.is_dir() {
                            MediaFile::from_path(path).ok()
                        } else if let Ok(media) = MediaFile::from_path(path) {
                            if media.is_video() {
                                return Some(media);
                            }
                            None
                        } else {
                            None
                        }
                    })
                })
                .collect();

            items.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            self.files.extend(items);
        }

        self.selected_index = self.selected_index.min(self.files.len().saturating_sub(1));
        Ok(())
    }

    pub fn navigate_to(&mut self, path: PathBuf) -> Result<()> {
        if path.is_dir() {
            self.current_path = path;
            self.refresh()?;
        }
        Ok(())
    }

    pub fn move_down(&mut self) {
        if self.files.len() > 0 {
            self.selected_index = (self.selected_index + 1) % self.files.len();
        }
    }

    pub fn move_up(&mut self) {
        if self.files.len() > 0 {
            self.selected_index = if self.selected_index == 0 {
                self.files.len() - 1
            } else {
                self.selected_index - 1
            };
        }
    }

    pub fn get_selected(&self) -> Option<&MediaFile> {
        self.files.get(self.selected_index)
    }

    pub fn files(&self) -> &[MediaFile] {
        &self.files
    }

    pub fn current_path(&self) -> &Path {
        &self.current_path
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
    }
}
