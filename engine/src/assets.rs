
use embedded_graphics::{
    pixelcolor::Rgb565,
};
use tinybmp::Bmp;

pub struct Assets<'a> {
    pub logo: Option<Bmp<'a, Rgb565>>,
}

impl Assets<'static> {
    pub fn new() -> Assets<'static> {
        Assets {
            logo: None,
        }
    }

    pub fn load(&mut self) {
        self.logo = Some(Bmp::<Rgb565>::from_slice(include_bytes!("../assets/img/espressif.bmp")).unwrap());
    }
}
