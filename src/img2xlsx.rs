use std::io::Cursor;

use wasm_bindgen::prelude::*;

use image::{imageops::FilterType, ImageError, RgbImage};
use rust_xlsxwriter::{Color, Format, Workbook, XlsxError};
pub fn workbook_from_image(img: &RgbImage) -> Result<Workbook, XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    for x in 0..img.width() {
        worksheet.set_column_width(x as u16, 2)?;
        for y in 0..img.height() {
            let pixel = img.get_pixel(x, y);
            let color = ((pixel[0] as u32) << 16) + ((pixel[1] as u32) << 8) + (pixel[2] as u32);
            let format = Format::new().set_background_color(Color::RGB(color));
            worksheet.write_string_with_format(y, x as u16, "", &format)?;
        }
    }

    Ok(workbook)
}

pub fn resize_image(
    img: &RgbImage,
    width: Option<u32>,
    height: Option<u32>,
    filter: FilterType,
) -> RgbImage {
    let new_size = match (height, width) {
        (None, None) => None,
        (None, Some(width)) => Some((img.height() * width / img.width(), width)),
        (Some(height), None) => Some((height, img.width() * height / img.height())),
        (Some(height), Some(width)) => Some((height, width)),
    };

    match new_size {
        Some((new_height, new_width)) => {
            image::imageops::resize(img, new_width, new_height, filter)
        }
        None => img.clone(),
    }
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn get_length(buffer: &[u8]) -> u32 {
    buffer.len() as u32
}

fn decode_image_buffer(buffer: &[u8]) -> Result<RgbImage, ImageError> {
    Ok(image::io::Reader::new(Cursor::new(buffer))
        .with_guessed_format()?
        .decode()?
        .into_rgb8())
}

#[wasm_bindgen]
pub fn image_width(buffer: &[u8]) -> u32 {
    match decode_image_buffer(buffer) {
        Err(_) => 0,
        Ok(img) => img.width(),
    }
}

#[wasm_bindgen]
pub fn get_buffer(size: u8) -> Vec<u8> {
    (0..size).collect()
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn img2xlsx(buffer: &[u8], width: Option<u32>, height: Option<u32>) -> Vec<u8> {
    let img = if let Ok(img) = decode_image_buffer(buffer) {
        img
    } else {
        return vec![];
    };

    let img = resize_image(&img, width, height, FilterType::Gaussian);

    let mut workbook = if let Ok(workbook) = workbook_from_image(&img) {
        workbook
    } else {
        return vec![];
    };
    workbook.save_to_buffer().unwrap_or(vec![])
}
