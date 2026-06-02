# HyperTerm Video

A production-grade terminal video player written in Rust, featuring high-performance multi-threaded decoding, advanced rendering modes, integrated file browser, and cross-platform support.

## Features

### Interactive File Browser & Playlist
- **Built-in file browser** - Browse your system for videos
- **Playlist management** - Organize videos and play them sequentially
- **Default video player** - Run without arguments to open the file browser
- **Direct playback** - Pass video file as argument for immediate playback
- **Intuitive UI** - Ratatui-based interface with visual feedback

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
- MP4, MKV (Matroska), AVI, MOV (QuickTime), WebM, FLV, WMV, M4V, 3GP, OGV

### Advanced Features
- **Aspect-ratio correction**: Proper video display proportions
- **Variable frame rate** handling with real timestamps
- **Floyd-Steinberg dithering** for color reduction
- **Quality presets**: Performance, Balanced, Quality, Ultra
- **Real-time FPS monitoring**
- **Settings menu** for runtime configuration

## Quick Start

### As Default Video Player (File Browser Mode)

```bash
# Just run the executable
hyperterm-video

# Browse videos with arrow keys, press ENTER to play
```

### Direct Playback

```bash
# Play a specific video immediately
hyperterm-video ~/Videos/movie.mp4
```

## File Browser Controls

| Key | Action |
|-----|--------|
| `↑/↓` | Navigate files |
| `ENTER` | Open directory or play video |
| `a` | Add video to playlist |
| `TAB` | Switch between Browser/Playlist/Settings |
| `DELETE` | Remove from playlist |
| `c` | Clear playlist |
| `F1/h` | Show help |
| `q/ESC` | Quit |

### Browser Views

#### 📁 File Browser Tab
- Browse your file system
- See video files with file sizes
- Add videos to playlist with `a` key

#### ▶ Playlist Tab
- View all queued videos
- Select which video to play next
- Delete individual entries or clear all
- Current video marked with ▶ indicator

#### ⚙️ Settings Tab
- **r** - Cycle render mode (ASCII/Block/Braille)
- **c** - Cycle color mode (Mono/ANSI16/ANSI256/Truecolor)
- **q** - Cycle quality preset (Performance/Balanced/Quality/Ultra)
- **d** - Toggle dithering on/off
- **h** - Toggle hardware decoding

### Playback Controls

| Key | Action |
|-----|--------|
| `Space` | Pause/Resume |
| `q` | Quit |
| `Left/Right` | Seek backward/forward |
| `Up/Down` | Adjust volume |
| `f` | Toggle FPS display |
| `c` | Cycle color modes |
| `r` | Cycle render modes |
| `m` | Open settings menu |

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

# Binary location
./target/release/hyperterm-video

# Run tests
cargo test

# Development build
cargo build
cargo run
```

## Architecture

```
src/
├── main.rs              # Entry point with optional CLI video path
├── app.rs               # Application state and lifecycle
├── file_browser.rs      # File system navigation
├── file_browser_ui.rs   # Multi-view UI (Browser/Playlist/Settings)
├── playlist.rs          # Playlist management
├── decoder.rs           # FFmpeg video decoding thread
├── scaler.rs            # Video scaling and color conversion
├── renderer.rs          # Terminal rendering pipeline
├── braille.rs           # Rendering engines (Braille/Block/ASCII)
├── ansi.rs              # Color utilities and dithering
├── ui.rs                # Ratatui settings menu
├── input.rs             # Keyboard input handling
├── config.rs            # Configuration management
└── fps.rs               # FPS calculation and monitoring
```

### Threading Model

```
┌─────────────┐     ┌─────────────┐     ┌────────��─────┐
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

## Setting as Default Video Player

### Linux
```bash
# Create .desktop file
sudo tee /usr/share/applications/hyperterm-video.desktop > /dev/null << EOF
[Desktop Entry]
Type=Application
Name=HyperTerm Video
Exec=/usr/local/bin/hyperterm-video %F
MimeType=video/mp4;video/x-matroska;video/x-msvideo;video/quicktime;video/webm;
Categories=Video;
EOF

# Make it default
xdg-mime default hyperterm-video.desktop video/mp4
```

### macOS
```bash
# Build and copy to Applications
cargo build --release
cp target/release/hyperterm-video /usr/local/bin/

# Set as default with `open -a HyperTerm\ Video video.mp4`
```

### Windows
```bash
# Copy executable to PATH or create shortcut
cargo build --release
copy target\release\hyperterm-video.exe "C:\Program Files\HyperTerm Video\"

# Associate via Settings > Apps > Default apps > Video player
```

## Development

### Code Quality
- No unsafe code outside FFmpeg bindings
- Comprehensive error handling
- Well-documented functions
- Modular architecture for extensibility

### Running Tests
```bash
cargo test
```

## License

MIT License - See LICENSE file for details

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

**File browser won't open:**
- Check HOME or USERPROFILE environment variable is set
- Ensure read permissions on video directories

**Performance issues:**
- Switch to Performance preset
- Reduce terminal window size
- Disable dithering
- Enable hardware decoding

**Color rendering issues:**
- Verify terminal supports target color mode
- Try different color modes (c key in settings)
- Check terminal COLORTERM variable

## Architecture Highlights

### Multi-threaded Design
The player uses three independent threads connected by lock-free queues:
1. **Decoder Thread** - Reads and decodes video frames
2. **Scaler Thread** - Scales frames and converts color spaces
3. **Renderer Thread** - Generates terminal output

This design maximizes throughput and responsiveness without busy-waiting.

### Smart Rendering
- Only updates terminal regions that changed
- Double buffering prevents flicker
- Adaptive quality based on playback speed
- SIMD-accelerated pixel operations

### File Browser Architecture
- Efficient directory traversal with caching
- Supports deep navigation
- Intelligent video format detection
- Cross-platform path handling

## Future Enhancements

- Subtitle support
- Audio visualization
- Advanced playlist features (shuffle, repeat)
- Video filters and effects
- Recording capability
- Thumbnail preview cache
- Recent files / bookmarks
