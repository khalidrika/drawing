use rand::Rng;
use raster::Color;

pub trait Drawable {
    fn draw(&self, image: &mut impl Displayable);
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

#[derive(Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(start: &Point, end: &Point) -> Self {
        Line {
            start: *start,
            end: *end,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Line {
            start: Point::random(width, height),
            end: Point::random(width, height),
        }
    }
}

impl Drawable for Line {
    //Bresenhem algorithm Line
    fn draw(&self, image: &mut impl Displayable) {
        let dx = (self.end.x - self.start.x).abs();
        let dy = (self.end.y - self.start.y).abs();
        let sx = if self.start.x < self.end.x { 1 } else { -1 };
        let sy = if self.start.y < self.end.y { 1 } else { -1 };
        let mut err = dx - dy; // Variable Error
        let mut current = self.start;

        loop {
            image.display(current.x, current.y, self.color());
            if current.x == self.end.x && current.y == self.end.y {
                break;
            }
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                current.x += sx;
            }
            if e2 < dx {
                err += dx;
                current.y += sy;
            }
        }
    }

    fn color(&self) -> Color {
        Color::rgb(0, 255, 0)
    }
}
