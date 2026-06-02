use crate::ansi::{AnsiRenderer, RgbColor};
use crate::braille::{BrailleRenderer, BlockRenderer, AsciiRenderer};
use crate::config::{ColorMode, RenderMode};
use crate::scaler::ScaledFrame;
use anyhow::Result;
use crossbeam_channel::Receiver;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct Renderer {
    render_mode: RenderMode,
    color_mode: ColorMode,
    width: u32,
    height: u32,
}

impl Renderer {
    pub fn new(render_mode: RenderMode, color_mode: ColorMode, width: u32, height: u32) -> Self {
        Renderer {
            render_mode,
            color_mode,
            width,
            height,
        }
    }

    pub fn start_rendering(
        self,
        rx: Receiver<Option<ScaledFrame>>,
        running: Arc<AtomicBool>,
    ) -> Result<()> {
        use crossterm::{
            execute,
            terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        };
        use std::io;

        let mut stdout = io::stdout();
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen)?;

        let ansi_renderer = AnsiRenderer::new(self.color_mode, self.width, self.height);

        while running.load(Ordering::Relaxed) {
            match rx.recv() {
                Ok(Some(frame)) => {
                    let output = self.render_frame(&frame)?;
                    execute!(stdout, crossterm::cursor::MoveTo(0, 0))?;
                    println!("{}", output);
                }
                Ok(None) => break,
                Err(_) => break,
            }
        }

        disable_raw_mode()?;
        execute!(stdout, LeaveAlternateScreen)?;
        Ok(())
    }

    fn render_frame(&self, frame: &ScaledFrame) -> Result<String> {
        match self.render_mode {
            RenderMode::Braille => {
                let renderer = BrailleRenderer::new(self.width, self.height);
                Ok(renderer.render(&frame.data))
            }
            RenderMode::Block => {
                let renderer = BlockRenderer::new(self.width, self.height);
                Ok(renderer.render(&frame.data))
            }
            RenderMode::ASCII => {
                let renderer = AsciiRenderer::new(self.width, self.height);
                Ok(renderer.render(&frame.data))
            }
        }
    }
}
