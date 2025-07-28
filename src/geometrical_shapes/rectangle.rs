use super::point::Point;
use crate::geometrical_shapes::{ Drawable };
use super::line::*;

// Rectangle: a new rectangle should be created
// from references to two different points.
pub struct Rectangle {
    pub a: Point,
    pub b: Point,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Rectangle {
            a: *p1,  
            b: *p2,
        }   
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut raster::Image) {
        
        let point_c = &Point::new(self.a.x, self.b.y);
        let point_d = &Point::new(self.b.x, self.a.y);

        let line1 = Line::new(&self.a, point_c);
        let line2 = Line::new(&self.a, point_d);
        let line3 = Line::new(point_c, &self.b);
        let line4 = Line::new(point_d, &self.b);
        line1.draw(image);
        line2.draw(image);
        line3.draw(image);
        line4.draw(image);
    }
}