use ab_glyph::{FontRef, PxScale};
use clap::Parser;
use image::{ImageBuffer, ImageFormat, Rgba, codecs::jpeg::JpegEncoder};
use imageproc::drawing::{draw_text_mut, text_size};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

/// Placeholder image generator
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Output image width
    #[arg(short, long)]
    width: u32,

    /// Output image height
    #[arg(long)]
    height: u32,

    /// Background color in hex, e.g. "#cccccc"
    #[arg(long, default_value = "#cccccc")]
    bg: String,

    /// Foreground/text color in hex, e.g. "#333333"
    #[arg(long, default_value = "#333333")]
    fg: String,

    /// Text to display in the center
    #[arg(short, long, default_value = "holderplace")]
    text: String,

    /// Output format: png, jpeg, gif, bmp, webp
    #[arg(short, long, default_value = "png")]
    format: String,

    /// Output file path
    #[arg(short, long, default_value = "out.png")]
    output: String,
}

fn parse_hex_color(s: &str) -> Rgba<u8> {
    let s = s.trim_start_matches('#');
    let (r, g, b) = match s.len() {
        6 => {
            let r = u8::from_str_radix(&s[0..2], 16).unwrap();
            let g = u8::from_str_radix(&s[2..4], 16).unwrap();
            let b = u8::from_str_radix(&s[4..6], 16).unwrap();
            (r, g, b)
        }
        _ => panic!("Invalid hex color: {}", s),
    };
    Rgba([r, g, b, 255])
}

fn main() {
    let args = Args::parse();

    let width = args.width;
    let height = args.height;
    let bg = parse_hex_color(&args.bg);
    let fg = parse_hex_color(&args.fg);

    let mut image = ImageBuffer::from_pixel(width, height, bg);

    // Load font
    let font_data = include_bytes!("../fonts/InterVariable.ttf") as &[u8];
    let font = FontRef::try_from_slice(font_data).expect("Error loading font");

    let font_scale = (width.min(height) as f32 / 10.0).max(12.0);
    let scale = PxScale::from(font_scale);

    // Estimate text size
    let (text_width, text_height) = text_size(scale, &font, &args.text);

    let start_x = (width as i32 - text_width as i32) / 2;
    let start_y = (height as i32 - text_height as i32) / 2;

    draw_text_mut(&mut image, fg, start_x, start_y, scale, &font, &args.text);

    let out_path = Path::new(&args.output);

    match args.format.to_lowercase().as_str() {
        "png" => image.save_with_format(out_path, ImageFormat::Png).unwrap(),
        "jpeg" | "jpg" => {
            let file = File::create(out_path).expect("Cannot create file");
            let mut writer = BufWriter::new(file);
            let mut encoder = JpegEncoder::new_with_quality(&mut writer, 80);
            encoder.encode_image(&image).unwrap();
        }
        "bmp" => image.save_with_format(out_path, ImageFormat::Bmp).unwrap(),
        "gif" => image.save_with_format(out_path, ImageFormat::Gif).unwrap(),
        "webp" => {
            let encoder = webp::Encoder::from_rgba(&image, width, height);
            let webp_data = encoder.encode(75.0);
            std::fs::write(out_path, &*webp_data).unwrap();
        }
        _ => panic!("Unsupported format: {}", args.format),
    }

    println!("✅ Generated {}", args.output);
}
