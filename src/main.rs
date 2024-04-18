use std::error::Error;

use image::{io::Reader as ImageReader, GenericImageView, Pixel, Rgb, Rgba};

const BRIGHTNESS_CHARS: &str =
    r#"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,"^`'. "#;
const BRAINFUCK_CHARS: &str = "<>[]+-,.";

// TODO: Remove BRAINFUCK_CHARS
fn get_char_by_brightness(brightness: u8) -> u8 {
  let i = (brightness as usize * (BRIGHTNESS_CHARS.len() - 1)) / 255;
  // Safe because maximum possible value is  BRIGHTNESS_CHARS.len() - 1
  *BRIGHTNESS_CHARS.as_bytes().get(i).unwrap()
}

fn read_image(path: &str) -> Result<(), Box<dyn Error>> {
    let required_width = 300;

    let img = ImageReader::open(path)?.decode()?.grayscale();
    let (w, h) = img.dimensions();

    // NOTE: Higher scale means smaller image
    let scale = w / required_width;
    for y in 0..h {
        for x in 0..w {
            if y % (scale * 2) == 0 && x % scale == 0 {
              let Rgba([brightness, _, _, _]) = img.get_pixel(x, y);
              print!("{}", char::from(get_char_by_brightness(brightness)));
            }
        }
        if y % (scale * 2) == 0 {
          println!("")
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    read_image("test.jpg").unwrap();
    Ok(())
}
