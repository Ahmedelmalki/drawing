
use crate::geometrical_shapes::{Displayable, Drawable};
use rand::Rng;
#[derive(Clone, Copy)] // what is the diffrnece 
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x: x, y: y }
    }
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::rng();
        let x = rng.random_range(0..=width);
        let y = rng.random_range(0..=height);
        Point::new(x, y)
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut raster::Image) {
        image.display(self.x, self.y, self.color());
    }
}