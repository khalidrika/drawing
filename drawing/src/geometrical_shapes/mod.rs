use rand::Rng;
use raster::Color;

// ...existing code...

impl Drawable for Point {
    fn draw(&self, image: &mut impl Displayable) {
        image.display(self.x, self.y, self.color());
    }
    fn color(&self) -> Color {
        Color::rgb(255, 0, 0)
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub top_left: Point,
    pub size: Point,
}

impl Rectangle {
    pub fn new(top_left: &Point, size: &Point) -> Self {
        Rectangle {
            top_left: *top_left,
            size: *size,
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut impl Displayable) {
        let x0 = self.top_left.x;
        let y0 = self.top_left.y;
        let w = self.size.x;
        let h = self.size.y;
        for x in x0..x0 + w {
            image.display(x, y0, self.color());
            image.display(x, y0 + h - 1, self.color());
        }
        for y in y0..y0 + h {
            image.display(x0, y, self.color());
            image.display(x0 + w - 1, y, self.color());
        }
    }
    fn color(&self) -> Color {
        Color::rgb(0, 0, 255)
    }
}

#[derive(Debug)]
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
    fn draw(&self, image: &mut impl Displayable) {
        Line::new(&self.a, &self.b).draw(image);
        Line::new(&self.b, &self.c).draw(image);
        Line::new(&self.c, &self.a).draw(image);
    }
    fn color(&self) -> Color {
        Color::rgb(255, 255, 0)
    }
}

#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: i32,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        Circle {
            center: *center,
            radius,
        }
    }
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let radius = rng.gen_range(10..50);
        Circle {
            center: Point::random(width, height),
            radius,
        }
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut impl Displayable) {
        let (cx, cy, r) = (self.center.x, self.center.y, self.radius);
        let mut x = r;
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            image.display(cx + x, cy + y, self.color());
            image.display(cx + y, cy + x, self.color());
            image.display(cx - y, cy + x, self.color());
            image.display(cx - x, cy + y, self.color());
            image.display(cx - x, cy - y, self.color());
            image.display(cx - y, cy - x, self.color());
            image.display(cx + y, cy - x, self.color());
            image.display(cx + x, cy - y, self.color());

            y += 1;
            if err <= 0 {
                err += 2 * y + 1;
            } else {
                x -= 1;
                err -= 2 * x + 1;
            }
        }
    }
    fn color(&self) -> Color {
        Color::rgb(255, 0, 255)
    }
}

// ...existing code...
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self { Point { x, y } }
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Point {
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
        }
    }
}

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
