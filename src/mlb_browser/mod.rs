pub mod mlb_api;
// use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, GlyphCache, ImageSize, Texture, TextureSettings};
use piston::input::{RenderArgs, UpdateArgs};
// use piston::input::*;
use graphics::math::Matrix2d;
use image::{DynamicImage, ImageFormat};
use mlb_api::Game;
use mlb_api::*;

struct MenuItem {
    game: Game,
    width: f64,
    height: f64,
    // img: image::RgbaImage,
    // img_height: f64,
    // img_width: f64,
    img_tex: Texture,
}

// Load the font as a binary resource and put it in a static container
const FONT: &[u8] = include_bytes!("../assets/FiraSans-Regular.ttf");
#[allow(unused)]
const RED: [f32; 4] = [1.0, 0.0, 0.0, 0.25];
#[allow(unused)]
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
#[allow(unused)]
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const OFFWHITE: [f32; 4] = [1.0, 1.0, 1.0, 0.15];

impl MenuItem {
    pub fn new(game: Game, width: f64, height: f64, img_tex: Texture) -> Self {
        MenuItem {
            game,
            width,
            height,
            img_tex,
        }
    }

    pub fn render(
        &self,
        is_selected: bool,
        transform: Matrix2d,
        scale: f64,
        glyph_cache: &mut GlyphCache,
        gl: &mut GlGraphics,
    ) {
        use graphics::{character::CharacterCache, text, Transformed};
        let vs_text = format!(
            "{} vs {}",
            &self.game.teams.home.team.name, &self.game.teams.away.team.name
        );
        let (desc_text, _) = self.game.get_recap();
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
                glyph_cache,
                vs_trans,
                gl,
            )
            .unwrap();
            text(
                WHITE,
                desc_text_font_size,
                &desc_text,
                glyph_cache,
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
    bg_texture: Texture,
    glyph_cache: GlyphCache<'static>,
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
        bg_texture: Texture,
        bg_size: (f64, f64),
        date: (u16, u8, u8),
    ) -> Self {
        // Load JSON data, for now synchronously
        let games = MlbApi::get_items(date.0, date.1, date.2);
        MlbApp {
            gl,
            rotation: 0.0,
            rate: 1.0,
            bg_texture,
            glyph_cache: GlyphCache::from_bytes(FONT, (), TextureSettings::new()).unwrap(),
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
            if let Some(items) = MlbApp::build_menu_items(Some(games)) {
                let len = items.len();
                self.items = Some(items);
                if let Some(selected_idx) = self.selected_idx {
                    if selected_idx >= len {
                        self.selected_idx = Some(len - 1)
                    }
                }
            }
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
                        MenuItem::new(g.to_owned(), 200.0, 200.0 * 9.0 / 16.0, img_tex)
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

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let bg_texture = &self.bg_texture;
        let (center_x, center_y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let (bg_w, bg_h) = self.bg_size;
        let items = &self.items;
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

        let date_str = &format!("{}-{}-{}", self.date.0, self.date.1, self.date.2);
        let glyph_cache = &mut self.glyph_cache;
        let instruction_str = "Use ← → to navigate, ↑ ↓ to change dates, ESC to exit";

        self.gl.draw(args.viewport(), |c, gl| {
            // Stretch our background image to the window and draw it
            let bg_trans = c
                .transform
                .scale(args.window_size[0] / bg_w, args.window_size[1] / bg_h);
            graphics::image(bg_texture, bg_trans, gl);

            let title_transform = c.transform.trans(50.0, 50.0);
            text(WHITE, 25, &date_str, glyph_cache, title_transform, gl).unwrap();
            let instruction_transform = c.transform.trans(5.0, args.window_size[1] - 5.0);
            text(OFFWHITE, 20, &instruction_str, glyph_cache, instruction_transform, gl).unwrap();
            
            // For each item in our items list, render it
            if let Some(items_list) = items {
                items_list.iter().enumerate().for_each(|(idx, item)| {
                    // Compute the transform for this item
                    let transform = c
                        .transform
                        .trans(center_x + (idx as f64 * item.width * 1.3), center_y * 1.1)
                        .trans(animated_scroll_offset * -item.width * 1.3, 0.0);
                    // Render it
                    item.render(
                        selected_idx == idx,
                        transform,
                        animated_scale,
                        glyph_cache,
                        gl,
                    );
                });
            }
        });
    }

    pub fn update(&mut self, args: UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt * self.rate;
        // Until our transition time scalar reaches 1, increment
        if self.trans_time < 1.0 {
            // This means 100%  in 100 ms
            self.trans_time += args.dt * 10.0;
            if self.trans_time > 1.0 {
                self.trans_time = 1.0;
            }
        }
    }
}
