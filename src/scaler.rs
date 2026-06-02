use crate::decoder::{Frame, FrameFormat};
use anyhow::Result;
use crossbeam_channel::{Receiver, Sender};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct ScaledFrame {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub timestamp: f64,
}

pub struct Scaler {
    target_width: u32,
    target_height: u32,
}

impl Scaler {
    pub fn new(target_width: u32, target_height: u32) -> Self {
        Scaler {
            target_width,
            target_height,
        }
    }

    pub fn start_scaling(
        self,
        rx: Receiver<Option<Frame>>,
        tx: Sender<Option<ScaledFrame>>,
        running: Arc<AtomicBool>,
    ) -> Result<()> {
        while running.load(Ordering::Relaxed) {
            match rx.recv() {
                Ok(Some(frame)) => {
                    let scaled = self.scale_frame(&frame)?;
                    if tx.send(Some(scaled)).is_err() {
                        break;
                    }
                }
                Ok(None) => {
                    let _ = tx.send(None);
                    break;
                }
                Err(_) => break,
            }
        }
        Ok(())
    }

    fn scale_frame(&self, frame: &Frame) -> Result<ScaledFrame> {
        let scaled_data = self.simple_scale_nearest(
            &frame.data,
            frame.width,
            frame.height,
            self.target_width,
            self.target_height,
        );

        Ok(ScaledFrame {
            data: scaled_data,
            width: self.target_width,
            height: self.target_height,
            timestamp: frame.timestamp,
        })
    }

    fn simple_scale_nearest(
        &self,
        data: &[u8],
        src_width: u32,
        src_height: u32,
        dst_width: u32,
        dst_height: u32,
    ) -> Vec<u8> {
        let mut output = vec![0u8; (dst_width * dst_height) as usize];
        let src_width = src_width as usize;
        let src_height = src_height as usize;
        let dst_width = dst_width as usize;
        let dst_height = dst_height as usize;

        for dst_y in 0..dst_height {
            for dst_x in 0..dst_width {
                let src_x = (dst_x * src_width) / dst_width;
                let src_y = (dst_y * src_height) / dst_height;

                if src_y < src_height && src_x < src_width {
                    output[dst_y * dst_width + dst_x] = data[src_y * src_width + src_x];
                }
            }
        }

        output
    }
}
