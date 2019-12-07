mod mlb_browser;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
// use piston::input::*;
use graphics::rectangle::square;
use graphics::{clear, Image};
use image::{DynamicImage, ImageFormat};
use mlb_browser::*;
use piston::window::WindowSettings;
use std::collections::HashMap;
use std::path::Path;
use reqwest::*;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 600.0;

fn main() {
    // Load OpenGL version
    let opengl = OpenGL::V4_5;
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("DSS Exercise #1", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Load the background image (binary resource)
    let img_bytes = include_bytes!("../1.jpg");
    let img = match image::load_from_memory_with_format(img_bytes, ImageFormat::JPEG).unwrap() {
        DynamicImage::ImageRgba8(data) => data,
        x => x.to_rgba(),
    };
    // Create texture from the background image
    let texture = Texture::from_image(&img, &TextureSettings::new());

    // Load JSON data, for now synchronously
    let resp: HashMap<String, String> = reqwest::blocking::get("https://httpbin.org/ip")
        .unwrap()
        .json()
        .unwrap();
    println!("{:?}", resp);

    // Create our mlb_browser
    let mut app = App::new(
        GlGraphics::new(opengl),
        texture,
        (img.width() as f64, img.height() as f64),
    );

    // Event loop for created window
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        // Render our window contents
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        // Update based on dt
        if let Some(args) = e.update_args() {
            app.update(args);
        }

        // Handle button events
        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(Key::Right) => {
                    app.set_rate(2.0f64);
                    app.select_next();
                }
                Button::Keyboard(Key::Left) => {
                    app.set_rate(-2.0f64);
                    app.select_prev();
                }
                _ => app.set_rate(1.0f64),
            }
        }
    }
}
