

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
    pub ghost_x: i32,
    pub ghost_y: i32,
    display: D,
    assets: Option<Assets<'static>>,
    step_size_x: u32,
    step_size_y: u32,
    camera_x: i32,
    camera_y: i32,
    animation_step: u32,
    teleport_counter: u32,
    walker_counter: u32,
    dynamite_counter: u32,
}


impl <D:embedded_graphics::draw_target::DrawTarget<Color = Rgb565>> Engine <D> {
    pub fn new(display:D, seed: Option<[u8; 32]>) -> Engine<D> {
        Engine {
            start_time: 0,
            ghost_x: 9*16,
            ghost_y: 7*16,
            display,
            assets: None,
            step_size_x: 16,
            step_size_y: 16,
            camera_x: 0,
            camera_y: 0,
            // #[cfg(any(feature = "imu_controls"))]
            animation_step: 0,
            teleport_counter: 100,
            walker_counter: 0,
            dynamite_counter: 1,
        }
    }

    fn check_object_collisions(&mut self) {
        let x = self.camera_x + self.ghost_x;
        let y = self.camera_y + self.ghost_y;

    }

    fn relocate_avatar(&mut self) {
        // let (new_camera_x, new_camera_y) = self.maze.get_random_coordinates();
        // (self.camera_x, self.camera_y) = (new_camera_x - self.ghost_x, new_camera_y - self.ghost_y);
    }

    fn relocate_coins(&mut self, amount: u32) {
        // self.maze.relocate_coins(amount);
    }

    fn check_npc_collision(&mut self) {
        let x = self.camera_x + self.ghost_x;
        let y = self.camera_y + self.ghost_y;

    }

    fn is_walkable(&self, x: i32, y: i32) -> bool {
        // Walk through walls
        true
    }

    pub fn move_right(&mut self) {
        let new_camera_x = self.camera_x + self.step_size_x as i32;
        if self.is_walkable(new_camera_x + self.ghost_x, self.camera_y + self.ghost_y) {
            self.camera_x = new_camera_x;
            self.check_object_collisions();
        }
    }

    pub fn move_left(&mut self) {
        let new_camera_x = self.camera_x - self.step_size_x as i32;
        if self.is_walkable(new_camera_x + self.ghost_x, self.camera_y + self.ghost_y) {
            self.camera_x = new_camera_x;
            self.check_object_collisions();
        }
    }

    pub fn move_up(&mut self) {
        let new_camera_y = self.camera_y - self.step_size_y as i32;
        if self.is_walkable(self.camera_x + self.ghost_x, new_camera_y + self.ghost_y) {
            self.camera_y = new_camera_y;
            self.check_object_collisions();
        }
    }

    pub fn move_down(&mut self) {
        let new_camera_y = self.camera_y + self.step_size_y as i32;
        if self.is_walkable(self.camera_x + self.ghost_x, new_camera_y + self.ghost_y) {
            self.camera_y = new_camera_y;
            self.check_object_collisions();
        }
    }

    pub fn teleport(&mut self) {
        if self.teleport_counter == 100 {
            self.relocate_avatar();
            self.teleport_counter = 0;
        }
    }

    pub fn place_dynamite(&mut self) {
    }

    pub fn draw_maze(&mut self, camera_x: i32, camera_y: i32) {
    }


    pub fn tick(&mut self) {
        // self.animation_step += 1;
        // if self.animation_step > 1 {
        //     self.animation_step = 0;
        // }

        // // Recharge teleport
        // if self.teleport_counter < 100 {
        //     self.teleport_counter += 1;
        // }

        // // Decrement remaining time when Walker is active
        // if self.walker_counter > 0 {
        //     self.walker_counter -= 1;
        // }

        // self.maze.move_npcs();
        // self.check_npc_collision();
    }

    pub fn initialize(&mut self) {
        let mut assets = Assets::new();
        assets.load();
        self.assets = Some(assets);

        // self.draw_maze(self.camera_x,self.camera_y);

    }

    fn draw_status_number(&mut self, value: u32, x: i32, y: i32) {
        let value_message: String<5> = String::from(value);
        Text::new(&value_message, Point::new(x, y), MonoTextStyle::new(&FONT_8X13, Rgb565::WHITE))
            .draw(&mut self.display);
    }

    pub fn draw(&mut self) -> &mut D {
        // self.draw_maze(self.camera_x,self.camera_y);


        match self.assets {
            Some(ref mut assets) => {

                let logo_bmp:Bmp<Rgb565> = assets.logo.unwrap();
                let position = Point::new(10, 10);
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


