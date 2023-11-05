use image::{
    imageops::FilterType, io::Reader as ImageReader, DynamicImage, GenericImageView, RgbImage,
};
use rust_xlsxwriter::{Color, Format, Workbook, XlsxError};

fn worksheet_from_image(img: &RgbImage) -> Result<Workbook, XlsxError> {
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

fn main() -> Result<(), XlsxError> {
    let img = match ImageReader::open("C:/Users/tamir/Downloads/frieren.webp")?.decode() {
        Ok(img) => img,
        Err(err) => panic!("{}", err),
    };

    if let Some(img) = img.as_rgb8() {
        let img = image::imageops::resize(img, 50, 50, FilterType::Gaussian);

        let mut workbook = worksheet_from_image(&img)?;
        workbook.save("image.xlsx")?;
    }
    Ok(())
}
