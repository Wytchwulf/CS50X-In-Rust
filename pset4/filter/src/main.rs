use clap::{Parser, Subcommand};
use image::{DynamicImage, Rgb, RgbImage};

#[derive(Parser)]
#[command(name = "Filter", about = "Image Filter Tool")]
struct Cli {
    #[command(subcommand)]
    command: FilterCommand,
}

#[derive(Subcommand)]
enum FilterCommand {
    #[command(visible_alias = "g")]
    Greyscale { input: String, output: String },
    #[command(visible_alias = "s")]
    Sepia { input: String, output: String },
    #[command(visible_alias = "r")]
    Reflect { input: String, output: String },
    #[command(visible_alias = "b")]
    Blur { input: String, output: String },
}

impl FilterCommand {
    fn components(self) -> (Filter, String, String) {
        match self {
            FilterCommand::Greyscale { input, output } => (Filter::Greyscale, input, output),
            FilterCommand::Sepia { input, output } => (Filter::Sepia, input, output),
            FilterCommand::Reflect { input, output } => (Filter::Reflect, input, output),
            FilterCommand::Blur { input, output } => (Filter::Blur, input, output),
        }
    }
}

enum Filter {
    Greyscale,
    Sepia,
    Reflect,
    Blur,
}

fn main() {
    let cli = Cli::parse();
    let (filter, input, output) = cli.command.components();

    let img = image::open(&input).expect("Failed to open imgae input");

    let result = match filter {
        Filter::Greyscale => img.grayscale(),
        Filter::Sepia => image::DynamicImage::ImageRgb8(apply_sepia(&img)),
        Filter::Reflect => img.fliph(),
        Filter::Blur => img.blur(2.0),
    };

    result.save(&output).expect("Failed to save image");
}

fn apply_sepia(img: &DynamicImage) -> RgbImage {
    let rgb = img.to_rgb8();
    let (width, height) = rgb.dimensions();
    let mut output = RgbImage::new(width, height);

    for (x, y, pixel) in rgb.enumerate_pixels() {
        let [r, g, b] = pixel.0;

        let tr = (0.393 * r as f32 + 0.769 * g as f32 + 0.189 * b as f32).min(255.0) as u8;
        let tg = (0.349 * r as f32 + 0.686 * g as f32 + 0.168 * b as f32).min(255.0) as u8;
        let tb = (0.272 * r as f32 + 0.534 * g as f32 + 0.131 * b as f32).min(255.0) as u8;

        output.put_pixel(x, y, Rgb([tr, tg, tb]));
    }

    output
}