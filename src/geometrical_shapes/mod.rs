use rand::Rng;
use raster::{ Color, Image };
pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> raster::Color {
        let mut rng = rand::rng();
        let r = rng.random_range(0..255);
        let g = rng.random_range(0..255);
        let b = rng.random_range(0..255);
        Color::rgb(r, g, b)
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// module declaretion
pub mod circle;
pub mod line;
pub mod point;
pub mod triangle;
pub mod rectangle;

// module export
pub use point::Point;
pub use line::Line;
pub use rectangle::Rectangle;
pub use triangle::Triangle;
pub use circle::Circle;
