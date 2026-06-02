use anyhow::{Context, Result};
use crossbeam_channel::{bounded, Sender};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub struct Frame {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub timestamp: f64,
    pub format: FrameFormat,
}

#[derive(Clone, Copy, Debug)]
pub enum FrameFormat {
    RGB24,
    YUV420,
}

pub struct Decoder {
    input_ctx: ffmpeg_next::format::context::Input,
    video_stream_index: usize,
    width: u32,
    height: u32,
    duration: f64,
}

impl Decoder {
    pub fn new(path: &Path) -> Result<Self> {
        let input = ffmpeg_next::format::input(&path)
            .context("Failed to open video file")?;

        let video_stream_index = input
            .streams()
            .best(ffmpeg_next::media::Type::Video)
            .map(|s| s.index())
            .context("No video stream found")?;

        let stream = input.stream(video_stream_index as usize)
            .context("Failed to get video stream")?;

        let codec_params = stream.codecpar();
        let width = codec_params.width();
        let height = codec_params.height();
        let duration = (stream.duration() as f64) * stream.time_base().numerator() as f64
            / stream.time_base().denominator() as f64;

        Ok(Decoder {
            input_ctx: input,
            video_stream_index,
            width,
            height,
            duration,
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn duration(&self) -> f64 {
        self.duration
    }

    pub fn start_decoding(
        mut self,
        tx: Sender<Option<Frame>>,
        running: Arc<AtomicBool>,
    ) -> Result<()> {
        for (stream, packet) in self.input_ctx.packets() {
            if !running.load(Ordering::Relaxed) {
                break;
            }

            if stream.index() == self.video_stream_index {
                let packet_data = packet.data().context("Invalid packet data")?;
                let timestamp = (packet.timestamp() as f64) * stream.time_base().numerator() as f64
                    / stream.time_base().denominator() as f64;

                let frame_data = packet_data.to_vec();
                let frame = Frame {
                    data: frame_data,
                    width: self.width,
                    height: self.height,
                    timestamp,
                    format: FrameFormat::YUV420,
                };

                if tx.send(Some(frame)).is_err() {
                    break;
                }
            }
        }

        let _ = tx.send(None);
        Ok(())
    }
}
