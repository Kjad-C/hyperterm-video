use crate::config::Config;
use crate::decoder::Decoder;
use crate::scaler::Scaler;
use crate::renderer::Renderer;
use crate::input::{InputHandler, InputEvent};
use crate::fps::{FpsCounter, FrameTimer};
use anyhow::Result;
use crossbeam_channel::bounded;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct App {
    config: Config,
    running: Arc<AtomicBool>,
    fps_counter: FpsCounter,
    frame_timer: FrameTimer,
}

impl App {
    pub async fn new(video_path: &Path, config: Config) -> Result<Self> {
        Ok(App {
            config,
            running: Arc::new(AtomicBool::new(true)),
            fps_counter: FpsCounter::new(30),
            frame_timer: FrameTimer::new(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        tracing::info!("Starting HyperTerm Video");

        let (decoder_tx, decoder_rx) = bounded(3);
        let (scaler_tx, scaler_rx) = bounded(3);

        let dims = self.config.quality_preset.dimensions();
        let running = self.running.clone();

        let input_handler = InputHandler::new();
        let input_running = running.clone();
        let (input_tx, _input_rx) = bounded(10);
        input_handler.start_input_thread(input_tx, input_running)?;

        let main_running = running.clone();
        while main_running.load(Ordering::Relaxed) {
            std::thread::sleep(std::time::Duration::from_millis(100));
            self.fps_counter.tick();
        }

        Ok(())
    }
}
