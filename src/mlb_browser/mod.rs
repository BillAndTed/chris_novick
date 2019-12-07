pub mod mlb_api;
// use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
// use piston::input::*;
use graphics::{character, math::Matrix2d, rectangle::square};
use graphics::{clear, Image};
use image::{DynamicImage, ImageFormat};
use mlb_api::Game;
use piston::window::WindowSettings;
use serde_derive::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Clone)]
struct MenuItem {
    is_selected: bool,
    game: Game,
}

impl MenuItem {
    pub fn render(&self, transform: Matrix2d, c: graphics::context::Context, gl: &mut GlGraphics) {
        use graphics::*;
        let texture_settings = TextureSettings::new();
        let font = include_bytes!("../../FiraSans-Regular.ttf");
        let g_text = format!(
            "{} vs {}",
            &self.game.teams.home.team.name, &self.game.teams.away.team.name
        );
        let mut glyph_cache = GlyphCache::from_bytes(font, (), TextureSettings::new()).unwrap();

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 0.25];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        // Center this block respectively
        let square = rectangle::square(0.0, 0.0, 50.0);
        if self.is_selected {
            let box_transform = transform.scale(1.5, 1.5).trans(-25.0, -25.0);
            let text_transform = transform.trans((-25.0 * 1.5), (-25.0 * 1.5));
            text(GREEN, 15, &g_text, &mut glyph_cache, text_transform, gl).unwrap();
            rectangle(BLUE, square, box_transform, gl);
        } else {
            // text(GREEN, 8, &g_text, &mut glyph_cache, transform, gl).unwrap();
            let transform = transform.trans(-25.0, -25.0);
            rectangle(RED, square, transform, gl);
        }
    }

    pub fn select(&mut self, s: bool) {
        self.is_selected = s;
    }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    rate: f64,
    bg_texture: Texture,
    bg_size: (f64, f64),
    items: Vec<MenuItem>,
    selected_idx: Option<usize>,
}

impl App {
    pub fn new(
        gl: GlGraphics, // OpenGL drawing backend.
        bg_texture: Texture,
        bg_size: (f64, f64),
        games: Vec<Game>,
    ) -> App {
        App {
            gl: gl,
            rotation: 0.0,
            rate: 1.0,
            bg_texture: bg_texture,
            bg_size,
            // items: vec![MenuItem { is_selected: false }; 4],
            items: games
                .iter()
                .map(|g| MenuItem {
                    is_selected: false,
                    game: g.to_owned(),
                })
                .collect(),
            selected_idx: None,
        }
    }

    fn update_selected(&mut self) {
        let selected = self.selected_idx;
        self.items
            .iter_mut()
            .enumerate()
            .for_each(|(idx, mut item)| {
                item.select(selected == Some(idx));
            });
    }

    pub fn select_next(&mut self) {
        if let Some(selected) = self.selected_idx {
            self.selected_idx = Some((selected + 1) % self.items.len());
        } else {
            self.selected_idx = Some(0);
        }
        self.update_selected();
    }

    pub fn select_prev(&mut self) {
        if let Some(selected) = self.selected_idx {
            self.selected_idx = Some((selected - 1) % self.items.len());
        } else {
            self.selected_idx = Some(0);
        }
        self.update_selected();
    }

    pub fn set_rate(&mut self, rate: f64) {
        self.rate = rate;
        // self.items.push(MenuItem{});
    }
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 0.25];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let bg_texture = &self.bg_texture;
        let (center_x, center_y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let (bg_w, bg_h) = self.bg_size;
        let items = self.items.clone();
        let selected = {
            if let Some(selected) = self.selected_idx {
                selected
            } else {
                0
            }
        };

        self.gl.draw(args.viewport(), |c, gl| {
            // Stretch our background image to the window and draw it
            let bg_trans = c
                .transform
                .scale(args.window_size[0] / bg_w, args.window_size[1] / bg_h);
            image(bg_texture, bg_trans, gl);

            let transform = c
                .transform
                // .trans(center_x, center_y)
                .trans(25.0, 25.0)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);

            // For each item in our items list, render it
            items.iter().enumerate().for_each(|(idx, item)| {
                let transform = c
                    .transform
                    .trans(center_x + (idx * 100) as f64, center_y)
                    .trans(selected as f64 * -100.0, 0.0);
                item.render(transform, c, gl);
            });
        });
    }

    pub fn update(&mut self, args: UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt * self.rate;
    }
}
