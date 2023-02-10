extern crate clap;
use clap::Parser;
use plotly::Scatter;
use plotly::{ImageFormat, Plot};
use clap::Arg;

#[derive(Parser)]
#[clap(version = "1.0", author = "Jinghuai Zhang", about = None)]
struct Args {
    #[clap(long, default_value = "1.0")]
    slope: f64,
    #[clap(long, default_value = "1.0")]
    bias: f64,
    #[clap(long, default_value = "-1.0")]
    low: f64,
    #[clap(long, default_value = "1.0")]
    high: f64,
}

fn line_and_scatter_plot(slope: f64, bias: f64, low: f64, high: f64) {
	let mut plot = Plot::new();
	let x1 = low;
	let y1 = (low-0.0)*slope+bias;
	let x2 = high;
	let y2 = (high-0.0)*slope+bias;

	let trace = Scatter::new(vec![x1, x2], vec![y1, y2]);
	plot.add_trace(trace);
	plot.write_image("result.png", ImageFormat::PNG, 800, 600, 1.0);
}

fn main() -> std::io::Result<()> {
	let args = Args::parse();
    line_and_scatter_plot(args.slope, args.bias, args.low, args.high);
    Ok(())
}


