use anyhow::{Context, Result};
use std::path::PathBuf;

mod app;
mod config;
mod decoder;
mod input;
mod renderer;
mod scaler;
mod ui;
mod ansi;
mod braille;
mod fps;
mod file_browser;
mod playlist;
mod file_browser_ui;

use app::App;
use config::Config;
use file_browser_ui::FileBrowserUI;

#[derive(Debug)]
struct Args {
    video_path: Option<PathBuf>,
}

impl Args {
    fn parse() -> Result<Self> {
        let args: Vec<String> = std::env::args().collect();
        
        let video_path = if args.len() > 1 && !args[1].starts_with('-') {
            Some(PathBuf::from(&args[1]))
        } else {
            None
        };

        Ok(Args { video_path })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .init();

    let args = Args::parse()?;

    ffmpeg_next::init().context("Failed to initialize FFmpeg")?;

    let config = Config::default();

    let video_path = match args.video_path {
        Some(path) => {
            if !path.exists() {
                anyhow::bail!("Video file not found: {}", path.display());
            }
            Some(path)
        }
        None => {
            let mut browser_ui = FileBrowserUI::new(config)?;
            browser_ui.run()?
        }
    };

    if let Some(path) = video_path {
        let config = Config::default();
        let mut app = App::new(&path, config).await?;
        app.run().await?;
    }

    Ok(())
}
