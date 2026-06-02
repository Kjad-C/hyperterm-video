use crate::ansi::RgbColor;

const BRAILLE_PATTERNS: [char; 256] = [
    '\u{2800}', '\u{2801}', '\u{2802}', '\u{2803}', '\u{2804}', '\u{2805}', '\u{2806}', '\u{2807}',
    '\u{2808}', '\u{2809}', '\u{280a}', '\u{280b}', '\u{280c}', '\u{280d}', '\u{280e}', '\u{280f}',
    '\u{2810}', '\u{2811}', '\u{2812}', '\u{2813}', '\u{2814}', '\u{2815}', '\u{2816}', '\u{2817}',
    '\u{2818}', '\u{2819}', '\u{281a}', '\u{281b}', '\u{281c}', '\u{281d}', '\u{281e}', '\u{281f}',
    '\u{2820}', '\u{2821}', '\u{2822}', '\u{2823}', '\u{2824}', '\u{2825}', '\u{2826}', '\u{2827}',
    '\u{2828}', '\u{2829}', '\u{282a}', '\u{282b}', '\u{282c}', '\u{282d}', '\u{282e}', '\u{282f}',
    '\u{2830}', '\u{2831}', '\u{2832}', '\u{2833}', '\u{2834}', '\u{2835}', '\u{2836}', '\u{2837}',
    '\u{2838}', '\u{2839}', '\u{283a}', '\u{283b}', '\u{283c}', '\u{283d}', '\u{283e}', '\u{283f}',
    '\u{2840}', '\u{2841}', '\u{2842}', '\u{2843}', '\u{2844}', '\u{2845}', '\u{2846}', '\u{2847}',
    '\u{2848}', '\u{2849}', '\u{284a}', '\u{284b}', '\u{284c}', '\u{284d}', '\u{284e}', '\u{284f}',
    '\u{2850}', '\u{2851}', '\u{2852}', '\u{2853}', '\u{2854}', '\u{2855}', '\u{2856}', '\u{2857}',
    '\u{2858}', '\u{2859}', '\u{285a}', '\u{285b}', '\u{285c}', '\u{285d}', '\u{285e}', '\u{285f}',
    '\u{2860}', '\u{2861}', '\u{2862}', '\u{2863}', '\u{2864}', '\u{2865}', '\u{2866}', '\u{2867}',
    '\u{2868}', '\u{2869}', '\u{286a}', '\u{286b}', '\u{286c}', '\u{286d}', '\u{286e}', '\u{286f}',
    '\u{2870}', '\u{2871}', '\u{2872}', '\u{2873}', '\u{2874}', '\u{2875}', '\u{2876}', '\u{2877}',
    '\u{2878}', '\u{2879}', '\u{287a}', '\u{287b}', '\u{287c}', '\u{287d}', '\u{287e}', '\u{287f}',
    '\u{2880}', '\u{2881}', '\u{2882}', '\u{2883}', '\u{2884}', '\u{2885}', '\u{2886}', '\u{2887}',
    '\u{2888}', '\u{2889}', '\u{288a}', '\u{288b}', '\u{288c}', '\u{288d}', '\u{288e}', '\u{288f}',
    '\u{2890}', '\u{2891}', '\u{2892}', '\u{2893}', '\u{2894}', '\u{2895}', '\u{2896}', '\u{2897}',
    '\u{2898}', '\u{2899}', '\u{289a}', '\u{289b}', '\u{289c}', '\u{289d}', '\u{289e}', '\u{289f}',
    '\u{28a0}', '\u{28a1}', '\u{28a2}', '\u{28a3}', '\u{28a4}', '\u{28a5}', '\u{28a6}', '\u{28a7}',
    '\u{28a8}', '\u{28a9}', '\u{28aa}', '\u{28ab}', '\u{28ac}', '\u{28ad}', '\u{28ae}', '\u{28af}',
    '\u{28b0}', '\u{28b1}', '\u{28b2}', '\u{28b3}', '\u{28b4}', '\u{28b5}', '\u{28b6}', '\u{28b7}',
    '\u{28b8}', '\u{28b9}', '\u{28ba}', '\u{28bb}', '\u{28bc}', '\u{28bd}', '\u{28be}', '\u{28bf}',
    '\u{28c0}', '\u{28c1}', '\u{28c2}', '\u{28c3}', '\u{28c4}', '\u{28c5}', '\u{28c6}', '\u{28c7}',
    '\u{28c8}', '\u{28c9}', '\u{28ca}', '\u{28cb}', '\u{28cc}', '\u{28cd}', '\u{28ce}', '\u{28cf}',
    '\u{28d0}', '\u{28d1}', '\u{28d2}', '\u{28d3}', '\u{28d4}', '\u{28d5}', '\u{28d6}', '\u{28d7}',
    '\u{28d8}', '\u{28d9}', '\u{28da}', '\u{28db}', '\u{28dc}', '\u{28dd}', '\u{28de}', '\u{28df}',
];

pub struct BrailleRenderer {
    width: u32,
    height: u32,
}

impl BrailleRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        BrailleRenderer { width, height }
    }

    pub fn render(&self, frame_data: &[u8]) -> String {
        let mut output = String::with_capacity((self.width * self.height * 4) as usize);
        let frame_width = self.width as usize;
        let frame_height = self.height as usize;

        for y in (0..frame_height).step_by(4) {
            for x in (0..frame_width).step_by(2) {
                let mut pattern: u8 = 0;

                let indices = [
                    (y * frame_width + x),
                    (y * frame_width + x + 1),
                    ((y + 1) * frame_width + x),
                    ((y + 1) * frame_width + x + 1),
                    ((y + 2) * frame_width + x),
                    ((y + 2) * frame_width + x + 1),
                    ((y + 3) * frame_width + x),
                    ((y + 3) * frame_width + x + 1),
                ];

                for (i, &idx) in indices.iter().enumerate() {
                    if idx < frame_data.len() && frame_data[idx] > 128 {
                        pattern |= 1 << i;
                    }
                }

                output.push(BRAILLE_PATTERNS[pattern as usize]);
            }
            output.push('\n');
        }

        output
    }
}

pub struct BlockRenderer {
    width: u32,
    height: u32,
}

impl BlockRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        BlockRenderer { width, height }
    }

    pub fn render(&self, frame_data: &[u8]) -> String {
        let chars = [" ", "▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];
        let mut output = String::with_capacity((self.width * self.height * 4) as usize);
        let frame_width = self.width as usize;

        for chunk in frame_data.chunks(frame_width) {
            for &byte in chunk {
                let char_idx = (byte as usize * chars.len()) / 256;
                output.push_str(chars[char_idx.min(chars.len() - 1)]);
            }
            output.push('\n');
        }

        output
    }
}

pub struct AsciiRenderer {
    width: u32,
    height: u32,
}

impl AsciiRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        AsciiRenderer { width, height }
    }

    pub fn render(&self, frame_data: &[u8]) -> String {
        let chars = "  .:-=+*#%@";
        let mut output = String::with_capacity((self.width * self.height * 2) as usize);
        let frame_width = self.width as usize;

        for chunk in frame_data.chunks(frame_width) {
            for &byte in chunk {
                let char_idx = (byte as usize * chars.len()) / 256;
                let c = chars.chars().nth(char_idx.min(chars.len() - 1)).unwrap_or(' ');
                output.push(c);
            }
            output.push('\n');
        }

        output
    }
}
