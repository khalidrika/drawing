mod geometrical_shapes;

use geometrical_shapes as gs;
use gs::{Displayable, Drawable}; // Importing the geometrical shapes module and traits
use raster::{Color, Image};// lib crate

fn main() {
    let mut image = Image::blank(1000, 1000);

    gs::Line::random(image.width, image.height).draw(&mut image);

    gs::Point::random(image.width, image.height).draw(&mut image);

    let rectangle = gs::Rectangle::new(
        &gs::Point::new(10, 300), &gs::Point::new(60, 500));
        
    rectangle.draw(&mut image);

    let triangle = gs::Triangle::new (
            &gs::Point::new(500, 500),
            &gs::Point::new(250, 700),
            &gs::Point::new(700, 800),
    );
    triangle.draw(&mut image);

    for _ in 1..2 {
        gs::Circle::random(image.width, image.height).draw(&mut image);
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