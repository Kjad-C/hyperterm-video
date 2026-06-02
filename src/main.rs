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

use app::App;

#[derive(Debug)]
struct Args {
    video_path: PathBuf,
    quality: Option<String>,
    render: Option<String>,
    color: Option<String>,
    no_hwdec: bool,
    no_dither: bool,
}

impl Args {
    fn parse() -> Result<Self> {
        let args: Vec<String> = std::env::args().collect();
        
        if args.len() < 2 {
            eprintln!("Usage: {} <video_file> [OPTIONS]", args[0]);
            eprintln!("Options:");
            eprintln!("  --quality <preset>     Quality preset: performance, balanced, quality, ultra");
            eprintln!("  --render <mode>        Rendering mode: ascii, block, braille");
            eprintln!("  --color <mode>         Color mode: mono, ansi16, ansi256, truecolor");
            eprintln!("  --no-hwdec             Disable hardware decoding");
            eprintln!("  --no-dither            Disable dithering");
            anyhow::bail!("Invalid arguments");
        }

        let video_path = PathBuf::from(&args[1]);
        let mut quality = None;
        let mut render = None;
        let mut color = None;
        let mut no_hwdec = false;
        let mut no_dither = false;

        let mut i = 2;
        while i < args.len() {
            match args[i].as_str() {
                "--quality" => {
                    if i + 1 < args.len() {
                        quality = Some(args[i + 1].clone());
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                "--render" => {
                    if i + 1 < args.len() {
                        render = Some(args[i + 1].clone());
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                "--color" => {
                    if i + 1 < args.len() {
                        color = Some(args[i + 1].clone());
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                "--no-hwdec" => {
                    no_hwdec = true;
                    i += 1;
                }
                "--no-dither" => {
                    no_dither = true;
                    i += 1;
                }
                _ => i += 1,
            }
        }

        Ok(Args {
            video_path,
            quality,
            render,
            color,
            no_hwdec,
            no_dither,
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .init();

    let args = Args::parse()?;

    if !args.video_path.exists() {
        anyhow::bail!("Video file not found: {}", args.video_path.display());
    }

    let mut config = config::Config::default();

    if let Some(quality) = args.quality {
        config.quality_preset = match quality.to_lowercase().as_str() {
            "performance" => config::QualityPreset::Performance,
            "balanced" => config::QualityPreset::Balanced,
            "quality" => config::QualityPreset::Quality,
            "ultra" => config::QualityPreset::Ultra,
            _ => anyhow::bail!("Invalid quality preset: {}", quality),
        };
    }

    if let Some(render) = args.render {
        config.render_mode = match render.to_lowercase().as_str() {
            "ascii" => config::RenderMode::ASCII,
            "block" => config::RenderMode::Block,
            "braille" => config::RenderMode::Braille,
            _ => anyhow::bail!("Invalid render mode: {}", render),
        };
    }

    if let Some(color) = args.color {
        config.color_mode = match color.to_lowercase().as_str() {
            "mono" => config::ColorMode::Monochrome,
            "ansi16" => config::ColorMode::ANSI16,
            "ansi256" => config::ColorMode::ANSI256,
            "truecolor" => config::ColorMode::Truecolor,
            _ => anyhow::bail!("Invalid color mode: {}", color),
        };
    }

    if args.no_hwdec {
        config.hardware_decode = false;
    }

    if args.no_dither {
        config.dithering = false;
    }

    ffmpeg_next::init().context("Failed to initialize FFmpeg")?;

    let mut app = App::new(&args.video_path, config).await?;
    app.run().await?;

    Ok(())
}
