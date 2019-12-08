mod mlb_browser;
use glutin_window::{GlutinWindow as Window};
use image::{DynamicImage, ImageFormat};
use mlb_browser::*;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

const WIDTH: f64 = 1366.0;
const HEIGHT: f64 = 768.0;

fn main() {
    // Load OpenGL version
    let opengl = OpenGL::V3_3;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("DSS Exercise #1", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        // .fullscreen(true)
        .build()
        .unwrap();

    // Load the background image (binary resource)
    let img_bytes = include_bytes!("assets/1.jpg");
    let img = match image::load_from_memory_with_format(img_bytes, ImageFormat::JPEG).unwrap() {
        DynamicImage::ImageRgba8(data) => data,
        x => x.to_rgba(),
    };
    // Create texture from the background image
    let texture = Texture::from_image(&img, &TextureSettings::new());

    // Create our mlb_browser
    let mut app = MlbApp::new(
        GlGraphics::new(opengl),
        texture,
        (img.width() as f64, img.height() as f64),
        (2018, 6, 11),
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
                    app.select_next();
                }
                Button::Keyboard(Key::Left) => {
                    app.select_prev();
                }
                Button::Keyboard(Key::Up) => {
                    app.increment_day();
                }
                Button::Keyboard(Key::Down) => {
                    app.decrement_day();
                }
                _ => (),
            }
        }
    }
}
