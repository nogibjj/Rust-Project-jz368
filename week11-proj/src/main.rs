use qrcode::{QrCode, Version, EcLevel};
use std::io::{self, Write};
use std::fs::File;
use std::io::prelude::*;
use std::env;
extern crate image;
use crate::image::GenericImageView;
use image::DynamicImage;
use image::imageops::FilterType;

fn read_image(path: &str) -> DynamicImage {
    println!("Getting image data");
    let img = image::open(path).unwrap();
    return img
}

fn main() {
    println!("Load image:");
    let path = "test.jpg";
    let image_data = read_image(&path);
    let resolution: u32 = 3 ;
    let small_img = image_data.resize(image_data.width() / resolution, image_data.height() / resolution, FilterType::Nearest);

    let mut y = 0;
    let mut art = String::new();
    let mut sum_k:f32 = 0.0;
    for p in small_img.pixels() {
        if y != p.1 {
            art.push_str("\n");
            y = p.1;

        }
        let r = p.2.0[0] as f32;
        let g = p.2.0[1] as f32;
        let b = p.2.0[2] as f32 ;
        let k = r * 0.3 + g * 0.59 + b * 0.11;
        sum_k += k;
    }
    println!("{sum_k}");

    let code = QrCode::with_version(sum_k.to_be_bytes(), Version::Normal(4), EcLevel::L).unwrap();
    let QR_image = code.render::<char>().quiet_zone(false).build();
    let mut file = std::fs::File::create("qrcode.txt").unwrap();
    write!(&mut file, "{}", QR_image).unwrap();
    println!("Save QR code.");
}