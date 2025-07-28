use crate::geometrical_shapes::{ Drawable };
use super::point::Point;
use super::line::*;

pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Triangle {
            a: *a,
            b: *b,
            c: *c,
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut raster::Image) {
        let color = self.color();
        draw_line(image, &self.a, &self.b, color.clone());
        draw_line(image, &self.a, &self.c, color.clone());
        draw_line(image, &self.b, &self.c, color.clone());
    }
}
