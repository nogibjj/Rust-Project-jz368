extern crate image;
extern crate clap;
use clap::Parser;
use imageproc::filter::gaussian_blur_f32;

#[derive(Parser)]
#[clap(version = "1.0", author = "Jinghuai Zhang", about = None)]
struct Args {
    #[clap(short, long, default_value = "lenna.png")]
    file_name: String,
    #[clap(short, long, default_value = "5.0")]
    blur: f32,
    #[clap(short, long, default_value = "blur_lenna.png")]
    save_name: String,
}

// create a function that loads an image file and uses gaussian blurring to process it

fn main() {
    let args = Args::parse();
    println!("Loading image file: {}", args.file_name);
    let dynamic_img = image::open(args.file_name).unwrap();
    let img = dynamic_img.into_rgb8();
    let blurred_img = gaussian_blur_f32(&img, args.blur);
    blurred_img.save(args.save_name).expect("Failed to save.");
}


