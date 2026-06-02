use anyhow::Result;
use crossbeam_channel::Sender;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    Pause,
    Resume,
    Quit,
    SeekForward,
    SeekBackward,
    VolumeUp,
    VolumeDown,
    ToggleFps,
    CycleColorMode,
    CycleRenderMode,
    OpenMenu,
}

pub struct InputHandler {
    paused: Arc<AtomicBool>,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            paused: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn is_paused(&self) -> bool {
        self.paused.load(Ordering::Relaxed)
    }

    pub fn start_input_thread(
        self,
        tx: Sender<InputEvent>,
        running: Arc<AtomicBool>,
    ) -> Result<()> {
        let paused = self.paused.clone();

        std::thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                if event::poll(Duration::from_millis(50)).is_ok() {
                    if let Ok(Event::Key(key_event)) = event::read() {
                        if let Some(input) = Self::handle_key(key_event) {
                            match input {
                                InputEvent::Pause => {
                                    paused.store(true, Ordering::Relaxed);
                                }
                                InputEvent::Resume => {
                                    paused.store(false, Ordering::Relaxed);
                                }
                                InputEvent::Quit => {
                                    running.store(false, Ordering::Relaxed);
                                    let _ = tx.send(InputEvent::Quit);
                                    return;
                                }
                                _ => {
                                    let _ = tx.send(input);
                                }
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    fn handle_key(key_event: KeyEvent) -> Option<InputEvent> {
        match key_event.code {
            KeyCode::Char(' ') => Some(InputEvent::Pause),
            KeyCode::Char('q') | KeyCode::Char('Q') => Some(InputEvent::Quit),
            KeyCode::Left => Some(InputEvent::SeekBackward),
            KeyCode::Right => Some(InputEvent::SeekForward),
            KeyCode::Up => Some(InputEvent::VolumeUp),
            KeyCode::Down => Some(InputEvent::VolumeDown),
            KeyCode::Char('f') | KeyCode::Char('F') => Some(InputEvent::ToggleFps),
            KeyCode::Char('c') | KeyCode::Char('C') => Some(InputEvent::CycleColorMode),
            KeyCode::Char('r') | KeyCode::Char('R') => Some(InputEvent::CycleRenderMode),
            KeyCode::Char('m') | KeyCode::Char('M') => Some(InputEvent::OpenMenu),
            _ => None,
        }
    }
}
