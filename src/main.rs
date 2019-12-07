use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
// use piston::input::*;
use graphics::rectangle::square;
use graphics::{clear, Image};
use image::{DynamicImage, ImageFormat};
use piston::window::WindowSettings;
use std::path::Path;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 600.0;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    rate: f64,
    bg_texture: Texture,
}

impl App {
    pub fn set_rate(&mut self, rate: f64) {
        self.rate = rate;
    }
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 0.25];
        const CLEAR: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let bg_texture = &self.bg_texture;
        let (center_x, center_y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Draw the background
            let bg_trans = c
                .transform
                .scale(args.window_size[0] / WIDTH, args.window_size[1] / HEIGHT);

            image(bg_texture, bg_trans, gl);

            let transform = c
                .transform
                .trans(center_x, center_y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt * self.rate;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V4_5;
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Load the background image
    let img_bytes = include_bytes!("../1.jpg");
    let img = match image::load_from_memory_with_format(img_bytes, ImageFormat::JPEG).unwrap() {
        DynamicImage::ImageRgba8(data) => data,
        x => x.to_rgba(),
    };
    // let texture = Texture::from_path(Path::new("./1.jpg"), &TextureSettings::new()).unwrap();
    let texture = Texture::from_image(&img, &TextureSettings::new());

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        rate: 1.0,
        bg_texture: texture,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(args);
        }

        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(Key::Right) => app.set_rate(2.0f64),
                Button::Keyboard(Key::Left) => app.set_rate(-2.0f64),
                _ => app.set_rate(1.0f64),
            }
        }
    }
}
