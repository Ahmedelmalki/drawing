use raster::{Color, Image};
pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> raster::Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub mod circle;
pub mod line;
pub mod point;
pub mod triangle;
