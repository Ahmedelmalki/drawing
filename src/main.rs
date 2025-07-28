mod geometrical_shapes;

// use geometrical_shapes as gs;
use geometrical_shapes::{ Displayable, Drawable };
use raster::{ Color, Image };

// imports
use crate::geometrical_shapes::line::Line;
use crate::geometrical_shapes::point::Point;
use crate::geometrical_shapes::rectangle::Rectangle;
use crate::geometrical_shapes::triangle::Triangle;
use crate::geometrical_shapes::circle::Circle;

fn main() {
    let mut image = Image::blank(1000, 1000);

    Line::random(image.width, image.height).draw(&mut image);

    Point::random(image.width, image.height).draw(&mut image);

    let rectangle = Rectangle::new(&Point::new(150, 150), &Point::new(50, 50));
    rectangle.draw(&mut image);

    let triangle = Triangle::new(
        &Point::new(500, 500),
        &Point::new(250, 700),
        &Point::new(700, 800)
    );
    triangle.draw(&mut image);

    for _ in 1..50 {
        Circle::random(image.width, image.height).draw(&mut image);
    }

    raster::save(&image, "image.png").unwrap();
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}
