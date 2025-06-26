use clap::{Parser, Subcommand};
use image::{self, Rgba, RgbaImage};

#[derive(Parser)]
#[command(name = "Filter More", about = "CS50X Filter More")]
struct Cli {
    #[command(subcommand)]
    command: FilterCommand,
}

#[derive(Subcommand)]
enum FilterCommand {
    #[command(visible_alias = "g")]
    Greyscale { input: String, output: String },
    #[command(visible_alias = "r")]
    Reflection { input: String, output: String },
    #[command(visible_alias = "b")]
    Blur { input: String, output: String },
    #[command(visible_alias = "e")]
    Edges { input: String, output: String },
}

impl FilterCommand {
    fn components(self) -> (Filter, String, String) {
        match self {
            FilterCommand::Greyscale { input, output } => (Filter::Greyscale, input, output),
            FilterCommand::Reflection { input, output } => (Filter::Reflection, input, output),
            FilterCommand::Blur { input, output } => (Filter::Blur, input, output),
            FilterCommand::Edges { input, output } => (Filter::Edges, input, output),
        }
    }
}

enum Filter {
    Greyscale,
    Reflection,
    Blur,
    Edges,
}

fn main() {
    let cli = Cli::parse();
    let (filter, input, output) = cli.command.components();

    let img = image::open(&input).expect("Unable to open image");

    let result = match filter {
        Filter::Greyscale => img.grayscale(),
        Filter::Reflection => img.fliph(),
        Filter::Blur => img.blur(2.0),
        Filter::Edges => {
            let rgba = img.to_rgba8();
            let sobel_rgba = sobel_edge_detection(&rgba);
            let rgb_image = image::DynamicImage::ImageRgba8(sobel_rgba).into_rgb8(); // <--- key fix
            image::DynamicImage::ImageRgb8(rgb_image)
        }
    };

    result.save(&output).expect("Unable to create image");
}

fn sobel_edge_detection(image: &RgbaImage) -> RgbaImage {
    let (width, height) = image.dimensions();
    let mut result = RgbaImage::new(width, height);

    let gx = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
    let gy = [[-1, -2, -1], [0, 0, 0], [1, 2, 1]];

    for x in 0..width {
        for y in 0..height {
            let mut gx_r = 0i32;
            let mut gx_g = 0i32;
            let mut gx_b = 0i32;
            let mut gy_r = 0i32;
            let mut gy_g = 0i32;
            let mut gy_b = 0i32;

            for dx in -1..=1 {
                for dy in -1..=1 {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;

                    if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                        let pixel = image.get_pixel(nx as u32, ny as u32).0;

                        let r = pixel[0] as i32;
                        let g = pixel[1] as i32;
                        let b = pixel[2] as i32;

                        let kx = gx[(dy + 1) as usize][(dx + 1) as usize];
                        let ky = gy[(dy + 1) as usize][(dx + 1) as usize];

                        gx_r += kx * r;
                        gx_g += kx * g;
                        gx_b += kx * b;

                        gy_r += ky * r;
                        gy_g += ky * g;
                        gy_b += ky * b;
                    }
                }
            }

            let magnitude = |gx: i32, gy: i32| -> u8 {
                (((gx * gx + gy * gy) as f64).sqrt().round().min(255.0)) as u8
            };

            result.put_pixel(
                x,
                y,
                Rgba([
                    magnitude(gx_r, gy_r),
                    magnitude(gx_g, gy_g),
                    magnitude(gx_b, gy_b),
                    255,
                ]),
            );
        }
    }

    result
}
