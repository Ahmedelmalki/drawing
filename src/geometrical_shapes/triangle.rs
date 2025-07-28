use crate::geometrical_shapes::{Drawable};
use raster::Color;
use super::point::Point;
use super::line::*;
use rand::Rng;

pub struct Triangle{
   pub a: Point,
   pub b: Point,
    pub c: Point
}

impl  Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point)-> Self{
        Triangle {
            a: *a,
            b:*b,
            c:*c,
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut raster::Image) {
        // todo : group the line drwaing algo in a separated func so u can use it here to get a triangle with same color
        let line1 = Line::new(&self.a,&self.b);
        let line2 = Line::new(&self.a,&self.c);
        let line3 = Line::new(&self.b,&self.c);
        line1.draw(image);
        line2.draw(image);
        line3.draw(image);
    }
    fn color(&self) -> raster::Color { // todo : remove this
        let mut rng = rand::rng();
        let r = rng.random_range(0..255);
        let g = rng.random_range(0..255);
        let b = rng.random_range(0..255);
        Color::rgb(r, g, b)
    }
}