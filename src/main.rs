use clap::Parser;
use image::{imageops::FilterType, io::Reader as ImageReader, RgbImage};
use rust_xlsxwriter::{Color, Format, Workbook, XlsxError};

fn worksheet_from_image(img: &RgbImage) -> Result<Workbook, XlsxError> {
    println!("{}/{}", img.height(), img.width());
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

/// Filter to use for scaling
#[derive(clap::ValueEnum, Clone, Debug)]
enum ScaleFilter {
    /// Nearest Neighbor
    Nearest,

    /// Linear Filter
    Triangle,

    /// Cubic Filter
    CatmullRom,

    /// Gaussian Filter
    Gaussian,

    /// Lanczos with window 3
    Lanczos3,
}
/// Convert an image to an xlsx file.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Source image
    image: String,

    /// Output path
    #[arg(short, long)]
    output: String,

    /// Number of columns to use. Defaults to image width.
    #[arg(short, long)]
    cols: Option<u32>,
    /// Number of rows to use. Defaults to image height.
    #[arg(short, long)]
    rows: Option<u32>,

    /// Filter used for scaling the image
    #[arg(short, long, default_value = "nearest")]
    filter: ScaleFilter,
}

fn main() -> Result<(), XlsxError> {
    let args = Args::parse();

    let img = match ImageReader::open(args.image)?.decode() {
        Ok(img) => img,
        Err(err) => panic!("{}", err),
    };

    if let Some(img) = img.as_rgb8() {
        let new_size = match (args.rows, args.cols) {
            (None, None) => None,
            (None, Some(cols)) => Some((img.height() * cols / img.width(), cols)),
            (Some(rows), None) => Some((rows, img.width() * rows / img.height())),
            (Some(rows), Some(cols)) => Some((rows, cols)),
        };
        let img = match new_size {
            Some((new_height, new_width)) => image::imageops::resize(
                img,
                new_width,
                new_height,
                match &args.filter {
                    ScaleFilter::Nearest => FilterType::Nearest,
                    ScaleFilter::Triangle => FilterType::Triangle,
                    ScaleFilter::CatmullRom => FilterType::CatmullRom,
                    ScaleFilter::Gaussian => FilterType::Gaussian,
                    ScaleFilter::Lanczos3 => FilterType::Lanczos3,
                },
            ),
            None => img.clone(),
        };

        let mut workbook = worksheet_from_image(&img)?;
        workbook.save(args.output)?;
    }
    Ok(())
}
