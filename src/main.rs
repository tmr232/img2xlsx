use image::{io::Reader as ImageReader, DynamicImage, GenericImageView, ImageError};
use rust_xlsxwriter::{Color, DocProperties, Format, FormatBorder, Workbook, XlsxError};

fn worksheet_from_image(img: &DynamicImage) -> Result<Workbook, XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    for x in 0..img.width() {
        worksheet.set_column_width(x as u16, 2)?;
        for y in 0..img.height() {
            let pixel = img.get_pixel(x, y);
            let color =
                ((pixel[0] as u32) << 16) + ((pixel[1] as u32) << 8) + ((pixel[2] as u32) << 0);
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

    let mut workbook = worksheet_from_image(&img)?;
    workbook.save("image.xlsx")?;
    Ok(())
}
