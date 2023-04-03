extern crate piston_window;
extern crate rand;
mod my_snake;
mod my_game;
use piston_window::*;
use piston_window::types::Color;
use my_game::Game;
use my_game::to_gui_coord;
use my_game::to_gui_coord_u32;
extern crate clap;
use clap::Parser;

const BACK_COLOR: Color = [1.0, 0.753, 0.796, 1.0];

#[derive(Parser)]
#[clap(version = "1.0", author = "Jinghuai Zhang", about = None)]
struct Args {
    #[clap(long, default_value = "25")]
    width: i32,
    #[clap(long, default_value = "25")]
    height: i32,
}

fn main() {
	let args = Args::parse();
    let (width, height) = (args.width, args.height);

    // Prepare window settings
    let mut window_settings = WindowSettings::new("Jinghuai Zhang-week10",
    [to_gui_coord_u32(width), to_gui_coord_u32(height)]).exit_on_esc(true);

    // Fix vsync extension error for linux
    window_settings.set_vsync(true); 

    // Create a window
    let mut window: PistonWindow = window_settings.build().unwrap();

    // Create a snake
    let mut game = Game::new(width, height);

    // Event loop
    while let Some(event) = window.next() {

        // Catch the events of the keyboard
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        // Draw all of them
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        // Update the state of the game
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
