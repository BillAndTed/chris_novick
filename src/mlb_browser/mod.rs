pub mod mlb_api;
// use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, GlyphCache, ImageSize, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
// use piston::input::*;
use graphics::{character, math::Matrix2d, rectangle::*};
use graphics::{clear, Image};
use image::{DynamicImage, ImageFormat};
use mlb_api::Game;
use piston::window::WindowSettings;
use serde_derive::{Deserialize, Serialize};
use serde_json::Result;

struct MenuItem {
    is_selected: bool,
    game: Game,
    width: f64,
    height: f64,
    // img: image::RgbaImage,
    // img_height: f64,
    // img_width: f64,
    img_tex: Texture,
}

impl MenuItem {
    pub fn render(&self, transform: Matrix2d, c: graphics::context::Context, gl: &mut GlGraphics) {
        use graphics::{rectangle, text, Rectangle, Transformed};
        let texture_settings = TextureSettings::new();
        let font = include_bytes!("../assets/FiraSans-Regular.ttf");
        let g_text = format!(
            "{} vs {}",
            &self.game.teams.home.team.name, &self.game.teams.away.team.name
        );
        let (recap_text, recap_url) = self.game.get_recap();
        let mut glyph_cache = GlyphCache::from_bytes(font, (), TextureSettings::new()).unwrap();

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 0.25];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        let center_x = self.width / 2.0;
        let center_y = self.height / 2.0;
        let (img_width, img_height) = self.img_tex.get_size();
        let img_width = img_width as f64;
        let img_height = img_height as f64;
        // Center this block respectively
        let square = rectangle::rectangle_by_corners(0.0, 0.0, self.width, self.height);
        if self.is_selected {
            let scale = 1.5;
            let scaled_width = scale * self.width;
            let scaled_height = scale * self.height;
            let box_transform = transform.scale(scale, scale).trans(-center_x, -center_y);
            let vs_trans = transform.trans(-center_x * scale, -center_y * scale - 15.0);
            let title_trans = transform.trans(-center_x * scale, center_y * scale + 15.0);
            let img_trans = transform
                .scale(scaled_width / img_width, scaled_height / img_height)
                .trans(-0.5 * img_width, -0.5 * img_height);
            text(WHITE, 22, &g_text, &mut glyph_cache, vs_trans, gl).unwrap();
            text(WHITE, 15, &recap_text, &mut glyph_cache, title_trans, gl).unwrap();
            rectangle(BLUE, square, box_transform, gl);
            graphics::image(&self.img_tex, img_trans, gl);
        } else {
            let transform = transform.trans(-center_x, -center_y);
            // rectangle(RED, square, transform, gl);
            let img_trans = transform.scale(self.width / img_width, self.height / img_height);
            graphics::image(&self.img_tex, img_trans, gl);
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
    items: Option<Vec<MenuItem>>,
    selected_idx: Option<usize>,
}

impl App {
    pub fn new(
        gl: GlGraphics, // OpenGL drawing backend.
        bg_texture: Texture,
        bg_size: (f64, f64),
        games: Option<Vec<Game>>,
    ) -> App {
        App {
            gl: gl,
            rotation: 0.0,
            rate: 1.0,
            bg_texture: bg_texture,
            bg_size,
            items: {
                match games {
                    Some(games_list) => Some(
                        games_list
                            .iter()
                            .map(|g| MenuItem {
                                is_selected: false,
                                game: g.to_owned(),
                                width: 200.0,
                                height: 200.0 * 9.0 / 16.0,
                                img_tex: {
                                    let (_, url) = g.get_recap();
                                    let img_bytes =
                                        Game::get_img(url.clone(), g.gamePk.to_string());
                                    let img = match image::load_from_memory_with_format(
                                        &img_bytes,
                                        ImageFormat::JPEG,
                                    )
                                    .unwrap()
                                    {
                                        DynamicImage::ImageRgba8(data) => data,
                                        x => x.to_rgba(),
                                    };
                                    Texture::from_image(&img, &TextureSettings::new())
                                },
                            })
                            .collect(),
                    ),
                    _ => None,
                }
            },
            selected_idx: None,
        }
    }

    fn update_selected(&mut self) {
        let selected = self.selected_idx;
        if let Some(items_list) = &mut self.items {
            items_list.iter_mut().enumerate().for_each(|(idx, item)| {
                item.select(selected == Some(idx));
            });
        }
    }

    pub fn select_next(&mut self) {
        if let Some(items_list) = &self.items {
            if let Some(selected) = self.selected_idx {
                self.selected_idx = Some((selected + 1) % items_list.len());
            } else {
                self.selected_idx = Some(0);
            }
            self.update_selected();
        }
    }

    pub fn select_prev(&mut self) {
        if let Some(items_list) = &self.items {
            if let Some(selected) = self.selected_idx {
                self.selected_idx = if selected == 0 {
                    Some(items_list.len() - 1)
                } else {
                    Some(selected - 1 % items_list.len())
                };
            } else {
                self.selected_idx = Some(0);
            }
            self.update_selected();
        }
    }

    pub fn set_rate(&mut self, rate: f64) {
        self.rate = rate;
        // self.items.push(MenuItem{});
    }
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 0.25];

        let square = rectangle::centered([0.0, 0.0, 25.0, 25.0]);
        let rotation = self.rotation;
        let bg_texture = &self.bg_texture;
        let (center_x, center_y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let (bg_w, bg_h) = self.bg_size;
        let items = &self.items; //.clone();
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
            graphics::image(bg_texture, bg_trans, gl);

            let transform = c
                .transform
                // .trans(center_x, center_y)
                .trans(25.0, 25.0)
                .rot_rad(rotation);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);

            // For each item in our items list, render it
            if let Some(items_list) = items {
                items_list.iter().enumerate().for_each(|(idx, item)| {
                    let transform = c
                        .transform
                        .trans(center_x + (idx as f64 * item.width * 1.3), center_y * 1.1)
                        .trans(selected as f64 * -item.width * 1.3, 0.0);
                    item.render(transform, c, gl);
                });
            }
        });
    }

    pub fn update(&mut self, args: UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt * self.rate;
    }
}
