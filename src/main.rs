use std::{error::Error, io::Write, path::PathBuf};

use clap::Parser;
use image::{io::Reader as ImageReader, DynamicImage, GenericImageView, Rgba};

const CHARS: &str = r#" .'`^",:;Il!i><~+_-?][}{1)(|\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B$"#;
const BRAINFUCK_CHARS: &str = "<>[]+-,.!";
const ASCII_ART_CHARS: &str = r#" '`^":;Ili~_?}{1)(|\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$"#;

fn get_pixel_char(brightness: f64) -> char {
    let i = (brightness * (ASCII_ART_CHARS.len() - 1) as f64) as usize;
    // Safe because maximum possible value is  BRIGHTNESS_CHARS.len() - 1
    char::from(ASCII_ART_CHARS.as_bytes()[i])
}

/// Computes the brightness value of a pixel
fn pixel_brightness(Rgba([r, g, b, _]): Rgba<u8>) -> f64 {
    // Green is brighter than red, which is brighter than blue
    ((r as f64 * 0.299) + (g as f64 * 0.587) + (b as f64 * 0.114)) / 255.0
}

/// Checks if `c1` and `c2` are within `threshold` of each other within CHARS
fn is_char_in_threshold(c1: char, c2: char, threshold: f32) -> bool {
    let Some(dist) = char_distance(c1, c2) else {
        return false;
    };
    dist <= threshold
}

fn char_distance(c1: char, c2: char) -> Option<f32> {
   Some((((CHARS.find(c1)? - CHARS.find(c2)?) as f32) / (CHARS.len() as f32)).abs())
}

fn image_ascii(img: &DynamicImage, required_width: u32) -> String {
    let (w, h) = img.dimensions();

    // NOTE: Higher scale means smaller image
    let scale = w / required_width;
    (0..h)
        // In most fonts the height of a glyph is about 3 times it's width
        .filter(|y| y % (scale * 3) == 0)
        .flat_map(|y| {
            (0..w)
                .filter(|x| x % scale == 0)
                .map(move |x| get_pixel_char(pixel_brightness(img.get_pixel(x, y))))
                .chain(['\n'])
        }).collect()
}

pub fn brainfuckify(
    input: &str,
    program: &str,
    threshold: f32,
) -> Option<String> {
    let mut res = String::with_capacity(input.len());
    let mut original = input.chars();

    for new in program.chars() {
        for orig in &mut original {
            if is_char_in_threshold(new, orig, threshold) {
                res.push(new);
            } else {
                res.push(orig);
            }
        }
        // TODO: If original finishes before program, then return None
    }
    Some(res)
}

/// Convert images into ascii art and embed brainfuck programs in them
#[derive(Parser, Debug)]
struct Args {
    /// The image to use
    image: PathBuf,
    /// The program to embed within the image
    #[arg(short, long)]
    program: String,
    /// The width of the generated ascii art
    #[arg(short, long, default_value_t = 100)]
    width: u32,
    /// The threshold for replacing characters. Higher means more characters will be replaced
    #[arg(short, long, default_value_t = 0.2)]
    threshold: f32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Args {
        program,
        width,
        image,
        threshold,
    } = Args::parse();
    // Remove non-brainfuck characters
    let program = program.replace(|c| !BRAINFUCK_CHARS.contains(c), "");

    let mut stdout = std::io::stdout().lock();
    let img = ImageReader::open(image)?.decode()?.grayscale();

    let res: String = brainfuckify(&image_ascii(&img, width), &program, threshold).expect("Threshold too low");
    write!(stdout, "{}", res)?;
    Ok(())
}
