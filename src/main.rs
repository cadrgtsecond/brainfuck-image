use std::{error::Error, io::Write, iter::repeat};

use image::{io::Reader as ImageReader, DynamicImage, GenericImageView, Rgba};

const BRIGHTNESS_CHARS: &str =
    r#" .'`^",:;Il!i><~+_-?][}{1)(|\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$"#;
const BRAINFUCK_CHARS: &str = "<>[]+-,.";

// TODO: Remove BRAINFUCK_CHARS
fn get_pixel_char(brightness: u8) -> char {
    let i = (brightness as usize * (BRIGHTNESS_CHARS.len() - 1)) / 255;
    // Safe because maximum possible value is  BRIGHTNESS_CHARS.len() - 1
    char::from(*BRIGHTNESS_CHARS.as_bytes().get(i).unwrap())
}
fn pixel_brightness(Rgba([r, g, b, a]): Rgba<u8>) -> u8 {
    let avg_color = (r as u32 * g as u32 * b as u32) / 3;
    ((avg_color * a as u32) / 255) as u8
}

fn image_ascii_chars(img: DynamicImage, required_width: u32) -> impl Iterator<Item = char> {
    let (w, h) = img.dimensions();

    // NOTE: Higher scale means smaller image
    let scale = w / required_width;
    (0..h)
        .filter(move |y| y % (scale * 3) == 0)
        .zip(repeat(img))
        .flat_map(move |(y, img)| {
            (0..w)
                .filter(move |x| x % scale == 0)
                .zip(repeat(img))
                .map(move |(x, img)| get_pixel_char(pixel_brightness(img.get_pixel(x, y))))
                .chain(['\n'])
        })
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = std::io::stdout().lock();
    let img = ImageReader::open("image.png")?.decode()?.grayscale();
    let res: String = image_ascii_chars(img, 80).collect();
    write!(stdout, "{}", res)?;
    Ok(())
}
