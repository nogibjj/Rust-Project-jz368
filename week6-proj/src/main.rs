use raster::Image;
use raster::Color;

fn main() {
	let file_name = "lenna.png";
	let save_name = "histogram.png";


	println!("Loading image file: {}", file_name);
	let image = raster::open(file_name).unwrap();
	let (r_bin, _, _, _) = image.histogram().unwrap();
	let mut max_r_bin = 0;
	for (_, count) in &r_bin {
	    if *count > max_r_bin {
	        max_r_bin = *count;
	    }
	}

	let canvas_w = 256;
	let canvas_h: i32 = 100;
	let mut image = Image::blank(canvas_w, canvas_h);
	raster::editor::fill(&mut image, Color::rgb(214, 214, 214)).unwrap();

	for x in 0..256 as i32 { // 0-255
	    let key = x as u8;
	    match r_bin.get(&key) {
	        Some(count) => {

	            let height = (canvas_h as f32 * (*count as f32 / max_r_bin as f32)).round() as i32;

	            for y in canvas_h-height..canvas_h {

	                image.set_pixel(x, y, Color::hex("#e22d11").unwrap()).unwrap();

	            }
	        },
	        None => {}
	    }
	}
	println!("Saving file at: {}", save_name);
	raster::save(&image, save_name).unwrap();
}

