use clap::Parser;
use image::{
    imageops::{colorops, FilterType},
    io::Reader as ImageReader,
    RgbImage,
};
use img2xlsx::{resize_image, worksheet_from_image};
use rust_xlsxwriter::{Color, Format, Workbook, XlsxError};

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
        let img = resize_image(
            img,
            args.cols,
            args.rows,
            match &args.filter {
                ScaleFilter::Nearest => FilterType::Nearest,
                ScaleFilter::Triangle => FilterType::Triangle,
                ScaleFilter::CatmullRom => FilterType::CatmullRom,
                ScaleFilter::Gaussian => FilterType::Gaussian,
                ScaleFilter::Lanczos3 => FilterType::Lanczos3,
            },
        );

        let mut workbook = worksheet_from_image(&img)?;
        workbook.save(args.output)?;
    }
    Ok(())
}
