# HyperTerm Video

A production-grade terminal video player written in Rust, featuring high-performance multi-threaded decoding, advanced rendering modes, and cross-platform support.

## Features

### Performance
- **Multi-threaded pipeline**: Decoder → Scaler → Renderer with lock-free queues
- **SIMD optimizations** for pixel processing
- **Hardware video decoding** support (when available)
- **Double buffering** with dirty-region optimization to prevent flicker
- **Efficient terminal output** by only redrawing changed regions

### Rendering Modes
- **ASCII Mode**: Universal compatibility
- **Unicode Block Mode**: Better detail with block characters
- **Braille Mode** (default): Highest detail with 2x2 dot resolution

### Color Support
- **Monochrome**: Maximum compatibility
- **ANSI 16-color**: Standard terminal colors
- **ANSI 256-color**: Extended palette
- **Truecolor RGB**: Full 24-bit color support with Floyd-Steinberg dithering

### Video Format Support
- MP4
- MKV (Matroska)
- AVI
- MOV (QuickTime)
- WebM

### Advanced Features
- **Aspect-ratio correction**: Proper video display proportions
- **Variable frame rate** handling with real timestamps
- **Floyd-Steinberg dithering** for color reduction
- **Quality presets**: Performance, Balanced, Quality, Ultra
- **Real-time FPS monitoring**
- **Settings menu** for runtime configuration

## Controls

| Key | Action |
|-----|--------|
| `Space` | Pause/Resume |
| `Q` | Quit |
| `Left/Right` | Seek backward/forward |
| `Up/Down` | Adjust volume |
| `F` | Toggle FPS display |
| `C` | Cycle color modes |
| `R` | Cycle render modes |
| `M` | Open settings menu |

## Settings Menu

Access via `M` key:
- **Quality Presets**: Performance, Balanced, Quality, Ultra
- **Color Mode**: Select from available color modes
- **Render Mode**: Choose rendering algorithm
- **FPS Limit**: Cap frame rate
- **Dithering**: Enable/disable dithering
- **Hardware Decode**: Toggle hardware acceleration

## Building

### Prerequisites

**Linux:**
```bash
sudo apt-get install libavformat-dev libavcodec-dev libavutil-dev libavdevice-dev libswscale-dev pkg-config
```

**macOS:**
```bash
brew install ffmpeg pkg-config
```

**Windows:**
Install FFmpeg from https://ffmpeg.org/download.html or use:
```bash
choco install ffmpeg
```

### Compilation

```bash
# Build with release optimizations (recommended)
cargo build --release

# Run
./target/release/hyperterm-video <video_file>

# Or run directly
cargo run --release -- <video_file>
```

## Architecture

```
src/
├── main.rs           # Entry point and CLI argument handling
├── app.rs            # Application state and lifecycle management
├── decoder.rs        # FFmpeg-based video decoding thread
├── scaler.rs         # Video scaling and color conversion thread
├── renderer.rs       # Terminal rendering pipeline with buffering
├── braille.rs        # Braille character rendering engine
├── ansi.rs           # ANSI color and rendering utilities
├── ui.rs             # Ratatui-based settings menu UI
├── input.rs          # Crossterm keyboard input handling
├── config.rs         # Configuration and settings management
└── fps.rs            # FPS calculation and monitoring
```

### Threading Model

```
┌─────────────┐     ┌──────��──────┐     ┌──────────────┐
│ Decoder     │────▶│ Scaler      │────▶│ Renderer     │
│ Thread      │     │ Thread      │     │ Thread       │
└─────────────┘     └─────────────┘     └──────────────┘
     ▲                   ▲                     ▲
     │                   │                     │
   Lock-free queue (crossbeam-channel)
```

Each thread operates independently with minimal contention:
- **Decoder**: Reads frames from FFmpeg
- **Scaler**: Applies scaling and color space conversion
- **Renderer**: Generates terminal output and manages display

## Performance Characteristics

- **Memory efficient**: Streaming decoding without loading entire video
- **CPU optimized**: SIMD instructions for pixel operations
- **I/O optimized**: Asynchronous processing with lock-free queues
- **Terminal optimized**: Only updates changed regions

## Quality Presets

| Preset | Width | Height | Dithering | Details |
|--------|-------|--------|-----------|---------|
| Performance | 80 | 30 | No | Fast, low detail |
| Balanced | 120 | 45 | Yes | Good balance |
| Quality | 160 | 60 | Yes | High detail |
| Ultra | 200 | 75 | Yes | Maximum detail |

## Error Handling

Comprehensive error handling with `anyhow`:
- FFmpeg codec errors
- Invalid video format detection
- Terminal capability verification
- File I/O errors
- Configuration errors

## Cross-Platform Support

- ✅ Linux (x86_64, ARM64)
- ✅ macOS (Intel, Apple Silicon)
- ✅ Windows (x86_64)

## Development

### Code Quality
- No unsafe code outside FFmpeg bindings
- Comprehensive error handling
- Well-documented functions
- Modular architecture for extensibility

### Building for Development
```bash
cargo build
cargo run -- video.mp4
```

### Running Tests
```bash
cargo test
```

## License

MIT License - See LICENSE file for details

## Contributing

Contributions are welcome! Please ensure:
1. Code compiles without warnings
2. Follows Rust conventions
3. Includes error handling
4. Maintains modularity

## Performance Tips

1. **Use Quality preset** for best visual quality
2. **Enable Hardware Decode** if your GPU supports it
3. **Reduce terminal window size** for better performance
4. **Use Braille rendering** for maximum detail efficiency
5. **Enable dithering** for smoother color gradients

## Troubleshooting

**No video plays:**
- Ensure FFmpeg libraries are installed
- Verify video file format is supported
- Check file path is correct

**Performance issues:**
- Switch to Performance preset
- Reduce terminal window size
- Disable dithering
- Enable hardware decoding

**Color rendering issues:**
- Verify terminal supports target color mode
- Try different color modes (C key)
- Check terminal COLORTERM variable

## Future Enhancements

- Subtitle support
- Audio visualization
- Playlist support
- Video filters
- Recording capability
- Thumbnail preview
