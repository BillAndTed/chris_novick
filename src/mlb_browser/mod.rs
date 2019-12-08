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
use mlb_api::*;
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

// Load the font as a binary resource and put it in a static container
const FONT: &'static [u8] = include_bytes!("../assets/FiraSans-Regular.ttf");

impl MenuItem {
    pub fn new(is_selected: bool, game: Game, width: f64, height: f64, img_tex: Texture) -> Self {
        MenuItem {
            is_selected,
            game,
            width,
            height,
            img_tex,
        }
    }

    pub fn render(&self, is_selected: bool, transform: Matrix2d, scale: f64, gl: &mut GlGraphics) {
        use graphics::{character::CharacterCache, rectangle, text, Rectangle, Transformed};
        let vs_text = format!(
            "{} vs {}",
            &self.game.teams.home.team.name, &self.game.teams.away.team.name
        );
        let (desc_text, _) = self.game.get_recap();
        let mut glyph_cache = GlyphCache::from_bytes(FONT, (), TextureSettings::new()).unwrap();
        let vs_text_font_size = 22;
        let desc_text_font_size = 15;
        let vs_text_width: f64 = vs_text
            .chars()
            .map(|c| {
                glyph_cache
                    .character(vs_text_font_size, c)
                    .unwrap()
                    .advance_width()
            })
            .sum();
        let desc_text_width: f64 = desc_text
            .chars()
            .map(|c| {
                glyph_cache
                    .character(desc_text_font_size, c)
                    .unwrap()
                    .advance_width()
            })
            .sum();

        // const RED: [f32; 4] = [1.0, 0.0, 0.0, 0.25];
        // const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        // const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        let center_x = self.width / 2.0;
        let center_y = self.height / 2.0;
        let (img_width, img_height) = self.img_tex.get_size();
        let img_width = img_width as f64;
        let img_height = img_height as f64;
        // Center this block respectively
        // let square = rectangle::rectangle_by_corners(0.0, 0.0, self.width, self.height);
        if is_selected {
            let scaled_width = scale * self.width;
            let scaled_height = scale * self.height;
            // let box_transform = transform.scale(scale, scale).trans(-center_x, -center_y);
            let vs_trans = transform.trans(
                -vs_text_width / 2.0,
                -center_y * scale - vs_text_font_size as f64 / 2.0,
            );
            let desc_trans = transform.trans(
                -desc_text_width / 2.0,
                center_y * scale + desc_text_font_size as f64,
            );
            let img_trans = transform
                .scale(scaled_width / img_width, scaled_height / img_height)
                .trans(-0.5 * img_width, -0.5 * img_height);
            text(
                WHITE,
                vs_text_font_size,
                &vs_text,
                &mut glyph_cache,
                vs_trans,
                gl,
            )
            .unwrap();
            text(
                WHITE,
                desc_text_font_size,
                &desc_text,
                &mut glyph_cache,
                desc_trans,
                gl,
            )
            .unwrap();
            // rectangle(BLUE, square, box_transform, gl);
            graphics::image(&self.img_tex, img_trans, gl);
        } else {
            let transform = transform.trans(-center_x, -center_y);
            // rectangle(RED, square, transform, gl);
            let img_trans = transform.scale(self.width / img_width, self.height / img_height);
            graphics::image(&self.img_tex, img_trans, gl);
        }
    }
}

pub struct MlbApp {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    rate: f64,
    bvs_texture: Texture,
    bg_size: (f64, f64),
    items: Option<Vec<MenuItem>>,
    selected_idx: Option<usize>,
    prev_selected_idx: Option<usize>,
    date: (u16, u8, u8),
    trans_time: f64,
}

impl MlbApp {
    pub fn new(
        gl: GlGraphics, // OpenGL drawing backend.
        bvs_texture: Texture,
        bg_size: (f64, f64),
        date: (u16, u8, u8),
    ) -> Self {
        // Load JSON data, for now synchronously
        let games = MlbApi::get_items(date.0, date.1, date.2);
        MlbApp {
            gl: gl,
            rotation: 0.0,
            rate: 1.0,
            bvs_texture: bvs_texture,
            bg_size,
            items: MlbApp::build_menu_items(games),
            selected_idx: Some(0),
            prev_selected_idx: Some(0),
            date,
            trans_time: 0.0,
        }
    }

    pub fn increment_day(&mut self) {
        self.date.2 += 1;
        self.rebuild_menu();
    }

    pub fn decrement_day(&mut self) {
        self.date.2 -= 1;
        self.rebuild_menu();
    }

    fn rebuild_menu(&mut self) {
        if let Some(games) = MlbApi::get_items(self.date.0, self.date.1, self.date.2) {
            self.items = MlbApp::build_menu_items(Some(games));
        }
    }

    fn build_menu_items(games: Option<Vec<Game>>) -> Option<Vec<MenuItem>> {
        match games {
            Some(games_list) => Some(
                games_list
                    .iter()
                    .map(|g| {
                        let img_tex = {
                            let (_, url) = g.get_recap();
                            let img_bytes = Game::get_img(url.clone(), g.gamePk.to_string());
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
                        };
                        MenuItem::new(false, g.to_owned(), 200.0, 200.0 * 9.0 / 16.0, img_tex)
                    })
                    .collect(),
            ),
            _ => None,
        }
    }

    pub fn select_next(&mut self) {
        if let Some(items_list) = &self.items {
            if let Some(selected) = self.selected_idx {
                self.prev_selected_idx = self.selected_idx;
                self.selected_idx = Some((selected + 1) % items_list.len());
            } else {
                self.selected_idx = Some(0);
            }
        }
        self.trans_time = 0.0;
    }

    pub fn select_prev(&mut self) {
        if let Some(items_list) = &self.items {
            if let Some(selected) = self.selected_idx {
                self.prev_selected_idx = self.selected_idx;
                self.selected_idx = if selected == 0 {
                    Some(items_list.len() - 1)
                } else {
                    Some(selected - 1 % items_list.len())
                };
            } else {
                self.selected_idx = Some(0);
            }
        }
        self.trans_time = 0.0;
    }

    pub fn set_rate(&mut self, rate: f64) {
        self.rate = rate;
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 0.25];

        let square = rectangle::centered([0.0, 0.0, 25.0, 25.0]);
        let rotation = self.rotation;
        let bvs_texture = &self.bvs_texture;
        let (center_x, center_y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let (bg_w, bg_h) = self.bg_size;
        let items = &self.items; //.clone();
        let selected_idx = {
            if let Some(selected) = self.selected_idx {
                selected
            } else {
                0
            }
        };
        let prev_selected_idx = {
            if let Some(prev_selected_idx) = self.prev_selected_idx {
                prev_selected_idx
            } else {
                selected_idx
            }
        };
        let trans_time = self.trans_time;
        let animated_scroll_offset = prev_selected_idx as f64
            + (trans_time * (selected_idx as isize - prev_selected_idx as isize) as f64);

        let animated_scale = 1.0 + (self.trans_time * 0.5);

        self.gl.draw(args.viewport(), |c, gl| {
            // Stretch our background image to the window and draw it
            let bg_trans = c
                .transform
                .scale(args.window_size[0] / bg_w, args.window_size[1] / bg_h);
            graphics::image(bvs_texture, bg_trans, gl);

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
                        .trans(animated_scroll_offset * -item.width * 1.3, 0.0);
                    // (((selected_idx-prev_selected_idx) as f64 * trans_time) + prev_selected_idx as f64) * -item.width * 1.3, 0.0);
                    item.render(selected_idx == idx, transform, animated_scale, gl);
                });
            }
        });
    }

    pub fn update(&mut self, args: UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt * self.rate;
        if self.trans_time < 1.0 {
            self.trans_time += args.dt * 10.0;
            if self.trans_time > 1.0 {
                self.trans_time = 1.0;
            }
        }
    }
}
