use super::point::Point;
use rand::Rng;
use crate::geometrical_shapes::{ Displayable, Drawable };
use raster::{Color, Image};
pub struct Line {
    a: Point,
    b: Point,
}
impl Line {
    pub fn new(start: &Point, end: &Point) -> Self {
        Line {
            a: *start,
            b: *end,
        }
    }
    pub fn random(width: i32, height: i32) -> Self {
        let p1 = Point::random(width, height);
        let p2 = Point::random(width, height);

        Line::new(&p1, &p2)
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut raster::Image) {
        let color = self.color();
        draw_line(image, &self.a, &self.b, color);
    }
}

// Bresenham's Line Drawing Algorithm
pub fn draw_line(image: &mut Image, a: &Point, b: &Point, color: Color) {
    let x0 = a.x;
    let y0 = a.y;
    let x1 = b.x;
    let y1 = b.y;

    let dx = (x1 - x0).abs(); // d short for delta
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 }; // s short for step
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut parametre_de_decision = dx + dy;
    let mut x = x0;
    let mut y = y0;

    loop {
        // Draw the current pixel at (x, y)
        image.display(x, y, color.clone()); // put pixel

        // Check if we've reached the end point - if so, we're done
        if x == x1 && y == y1 {
            break;
        }

        // Calculate twice the error to avoid floating point arithmetic
        // This determines which direction(s) to step next
        let p = 2 * parametre_de_decision;

        // If the error suggests we're too far below the ideal line,
        // step in the x direction to get closer to the target
        if p >= dy {
            parametre_de_decision += dy;
            x += sx;
        }

        // If the error suggests we're too far to the side of the ideal line,
        // step in the y direction to get closer to the target
        if p <= dx {
            parametre_de_decision += dx;
            y += sy;
        }
    }
}