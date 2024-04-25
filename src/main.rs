use std::{error::Error, io::Write, path::PathBuf};

use clap::Parser;
use image::{io::Reader as ImageReader, DynamicImage, GenericImageView, Rgba};
use itertools::unfold;

const CHARS: &str = r#"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,"^`'. "#;
const BRAINFUCK_CHARS: &str = "<>[]+-,.!";
const BRIGHTNESS_CHARS: &str = r#"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}?_~ilI;:"^`' "#;

fn get_pixel_char(brightness: f64) -> char {
    let i = (brightness * (BRIGHTNESS_CHARS.len() - 1) as f64) as usize;
    // Safe because maximum possible value is  BRIGHTNESS_CHARS.len() - 1
    char::from(*BRIGHTNESS_CHARS.as_bytes().get(i).unwrap())
}
/// Computes the brightness value of a pixel
fn pixel_brightness(Rgba([r, g, b, _]): Rgba<u8>) -> f64 {
    ((r as f64 * 0.299) + (g as f64 * 0.587) + (b as f64 * 0.114)) / 255.0
}

/// Computes the distance between two characters in CHARS
fn char_distance(c1: char, c2: char) -> Option<usize> {
    Some(usize::abs_diff(CHARS.find(c1)?, CHARS.find(c2)?))
}

/// Checks if `c1` and `c2` are within `threshold` of each other within CHARS
fn is_char_in_threshold(c1: char, c2: char, threshold: usize) -> bool {
    let Some(dist) = char_distance(c1, c2) else {
        return false;
    };
    dist <= threshold
}

fn image_ascii(img: &DynamicImage, required_width: u32) -> impl Iterator<Item = char> + '_ {
    let (w, h) = img.dimensions();

    // NOTE: Higher scale means smaller image
    let scale = w / required_width;
    (0..h)
        .filter(move |y| y % (scale * 3) == 0)
        .flat_map(move |y| {
            (0..w)
                .filter(move |x| x % scale == 0)
                .map(move |x| get_pixel_char(pixel_brightness(img.get_pixel(x, y))))
                .chain(['\n'])
        })
}

pub fn brainfuckify<'a>(
    mut input: impl Iterator<Item = char> + 'a,
    program: &'a str,
    threshold: usize,
) -> impl Iterator<Item = char> + 'a {
    unfold(program.chars().peekable(), move |program| {
        let Some(next) = input.next() else {
            return program.next();
        };
        match program.next_if(|c| is_char_in_threshold(next, *c, threshold)) {
            Some(c) => Some(c),
            _ => Some(next),
        }
    })
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
    #[arg(short, long, default_value_t = 4)]
    threshold: usize,
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

    let res: String = brainfuckify(image_ascii(&img, width), &program, threshold).collect();
    write!(stdout, "{}", res)?;
    Ok(())
}
