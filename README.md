# Drawing

A Rust project for creating geometric shapes and drawing them to an image using raster graphics.

## Overview

This project implements a geometric shapes drawing system that can generate various shapes (points, lines, circles, triangles, rectangles) and render them to a PNG image. Each shape is implemented with random generation capabilities and customizable drawing logic.

## Features

- **Multiple Geometric Shapes**: Point, Line, Circle, Triangle, Rectangle
- **Random Shape Generation**: Automatic generation of shapes with random positions and properties
- **Trait-based Design**: Clean separation of concerns using `Drawable` and `Displayable` traits
- **Image Output**: Generates PNG images using the raster crate
- **Optimized Algorithms**: Implements Bresenham's line drawing algorithm and midpoint circle algorithm

## Project Structure

```
src/
├── main.rs
└── geometrical_shapes/
    ├── mod.rs          # Module definitions and traits
    ├── point.rs        # Point implementation
    ├── line.rs         # Line implementation with Bresenham's algorithm
    ├── circle.rs       # Circle implementation with midpoint algorithm
    ├── triangle.rs     # Triangle implementation
    └── rectangle.rs    # Rectangle implementation
```

## Dependencies

Add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
raster = "0.2.0"
rand = "0.8"
```

## Core Traits

### Drawable
```rust
pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> raster::Color;
}
```

### Displayable
```rust
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}
```

## Implemented Shapes

### Point
- **Constructor**: `Point::new(x: i32, y: i32)`
- **Random Generation**: `Point::random(width: i32, height: i32)`
- **Features**: Basic pixel drawing with `Clone` and `Copy` traits

### Line
- **Constructor**: `Line::new(start: &Point, end: &Point)`
- **Random Generation**: `Line::random(width: i32, height: i32)`
- **Algorithm**: Bresenham's Line Drawing Algorithm for efficient line rendering

### Circle
- **Constructor**: `Circle::new(center: &Point, radius: i32)`
- **Random Generation**: `Circle::random(width: i32, height: i32)`
- **Algorithm**: Midpoint Circle Algorithm with 8-fold symmetry for smooth circle rendering

### Triangle
- **Constructor**: `Triangle::new(p1: &Point, p2: &Point, p3: &Point)`
- **Features**: Draws triangle using line segments between vertices

### Rectangle
- **Constructor**: `Rectangle::new(corner1: &Point, corner2: &Point)`
- **Features**: Draws rectangle using four line segments

## Algorithms Implemented

### Bresenham's Line Drawing Algorithm
Used in `line.rs` for efficient line drawing with minimal floating-point operations. The implementation uses decision parameters to determine which pixels to plot.

### Midpoint Circle Algorithm
Used in `circle.rs` for drawing smooth circles by leveraging 8-fold symmetry. This algorithm only calculates points for one octant and mirrors them to draw the complete circle.

## Building and Running

```bash
# Build the project
cargo build

# Run the project
cargo run
```

This will generate an `image.png` file in your project directory containing the drawn shapes.

## Color Generation

Each shape automatically generates a random RGB color when drawn, with values between 0-255 for each color channel.

## Error Handling

The project includes bounds checking in the `Displayable` trait implementation to ensure pixels are only drawn within the image boundaries, preventing runtime errors.

## Technical Details

- **Memory Efficiency**: Point struct uses `Copy` trait for efficient pass-by-value
- **Safety**: Bounds checking prevents out-of-bounds pixel access  
- **Performance**: Optimized drawing algorithms minimize computational overhead
- **Modularity**: Each shape is implemented in its own module with consistent interfaces

## Potential Extensions

- Add more complex shapes (Pentagon, Cube, Ellipse)
- Implement filled shapes vs outline-only shapes
- Add color customization options per shape
- Implement shape transformations (rotation, scaling, translation)
- Add shape intersection detection
- Support for different image formats beyond PNG
- Anti-aliasing for smoother edges

## Learning Resources

- [Rust Traits Documentation](https://doc.rust-lang.org/stable/book/ch10-02-traits.html)
- [Raster Crate Documentation](https://docs.rs/raster/0.2.0/raster/)
- [Computer Graphics Algorithms](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
- [Midpoint Circle Algorithm](https://en.wikipedia.org/wiki/Midpoint_circle_algorithm)

## License

This project is created for educational purposes.