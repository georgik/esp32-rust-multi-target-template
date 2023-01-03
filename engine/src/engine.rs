

use embedded_graphics::{
    prelude::{Point, RgbColor},
    mono_font::{
        ascii::{FONT_8X13},
        MonoTextStyle,
    },
    text::Text,
    pixelcolor::Rgb565,
    Drawable,
    image::Image,
};

use crate::{assets::Assets};
use heapless::String;
use tinybmp::Bmp;

pub struct Engine<D> {
    pub start_time: u64,
    pub avatar_x: i32,
    pub avatar_y: i32,
    display: D,
    assets: Option<Assets<'static>>,
    step_size_x: u32,
    step_size_y: u32,
    animation_step: u32,
}


impl <D:embedded_graphics::draw_target::DrawTarget<Color = Rgb565>> Engine <D> {
    pub fn new(display:D, seed: Option<[u8; 32]>) -> Engine<D> {
        Engine {
            start_time: 0,
            avatar_x: 9*16,
            avatar_y: 7*16,
            display,
            assets: None,
            step_size_x: 16,
            step_size_y: 16,
            // #[cfg(any(feature = "imu_controls"))]
            animation_step: 0,
        }
    }

    fn check_object_collisions(&mut self) {
    }

    fn is_walkable(&self, x: i32, y: i32) -> bool {
        // Boundaries of the scene
        !( (x < 0) || (y < 0)  || (x > 280) || (y > 200) )
    }

    pub fn move_right(&mut self) {
        let new_avatar_x = self.avatar_x + self.step_size_x as i32;
        if self.is_walkable(new_avatar_x, self.avatar_y) {
            self.avatar_x = new_avatar_x;
            self.check_object_collisions();
        }
    }

    pub fn move_left(&mut self) {
        let new_avatar_x = self.avatar_x - self.step_size_x as i32;
        if self.is_walkable(new_avatar_x, self.avatar_y) {
            self.avatar_x = new_avatar_x;
            self.check_object_collisions();
        }
    }

    pub fn move_up(&mut self) {
        let new_avatar_y = self.avatar_y - self.step_size_y as i32;
        if self.is_walkable(self.avatar_x, new_avatar_y) {
            self.avatar_y = new_avatar_y;
            self.check_object_collisions();
        }
    }

    pub fn move_down(&mut self) {
        let new_avatar_y = self.avatar_y + self.step_size_y as i32;
        if self.is_walkable(self.avatar_x, new_avatar_y) {
            self.avatar_y = new_avatar_y;
            self.check_object_collisions();
        }
    }

    pub fn draw_background(&mut self, _camera_x: i32, _camera_y: i32) {
        self.display.clear(RgbColor::BLACK);
    }


    pub fn tick(&mut self) {
        // self.animation_step += 1;
        // if self.animation_step > 1 {
        //     self.animation_step = 0;
        // }

        // self.maze.move_npcs();
        // self.check_npc_collision();
    }

    pub fn initialize(&mut self) {
        let mut assets = Assets::new();
        assets.load();
        self.assets = Some(assets);

        self.draw_background(0, 0);

    }

    fn draw_status_number(&mut self, value: u32, x: i32, y: i32) {
        let value_message: String<5> = String::from(value);
        Text::new(&value_message, Point::new(x, y), MonoTextStyle::new(&FONT_8X13, Rgb565::WHITE))
            .draw(&mut self.display);
    }

    pub fn draw(&mut self) -> &mut D {
        self.draw_background(0, 0);

        match self.assets {
            Some(ref mut assets) => {

                let logo_bmp:Bmp<Rgb565> = assets.logo.unwrap();
                let position = Point::new(self.avatar_x, self.avatar_y);
                let tile = Image::new(&logo_bmp, position);
                tile.draw(&mut self.display);

                // display.flush().unwrap();
            },
            None => {
                panic!("Assets not loaded");
            }
        };

        &mut self.display
    }


}


