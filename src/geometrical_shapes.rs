use rand::{Rng, rng};
use raster::Color;

fn random_color() -> Color {
    let mut rng = rng();
    Color::rgb(
        rng.random_range(0..=255),
        rng.random_range(0..=255),
        rng.random_range(0..=255),
    )
}

pub trait Drawable {
    fn draw(&self, image: &mut dyn Displayable);
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

/// =================
///  Point
/// =================
#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub color: Color,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y, color: random_color() }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rng();
        Point {
            x: rng.random_range(0..width),
            y: rng.random_range(0..height),
            color: random_color(),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut dyn Displayable) {
        image.display(self.x, self.y, self.color());
    }

    fn color(&self) -> self:: Color {
        self.color.clone()
    }
}

/// =================
///  Line
/// =================
#[derive(Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
    pub color: Color,
}

impl Line {
    pub fn random(width: i32, height: i32) -> Self {
        Line {
            start: Point::random(width, height),
            end: Point::random(width, height),
            color: random_color(),
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut dyn Displayable) {
        let dx = (self.end.x - self.start.x).abs();
        let dy = (self.end.y - self.start.y).abs();
        let sx = if self.start.x < self.end.x { 1 } else { -1 };
        let sy = if self.start.y < self.end.y { 1 } else { -1 };
        let mut err = dx - dy;
        let mut current = self.start.clone();

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
        self.color.clone()
    }
}

/// =================
///  Rectangle
/// =================
#[derive(Debug)] //trait for debugging
pub struct Rectangle {
    p1: Point,
    p2: Point,
    color: Color,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Rectangle {
            p1: p1.clone(),
            p2: p2.clone(),
            color: random_color(),
        }
    }
}


impl Drawable for Rectangle {
    fn draw(&self, image: &mut dyn Displayable) {
        let x0 = self.p1.x.min(self.p2.x);
        let x1 = self.p1.x.max(self.p2.x);
        let y0 = self.p1.y.min(self.p2.y);
        let y1 = self.p1.y.max(self.p2.y);

        // top and bottom edges
        for x in x0..=x1 {
            image.display(x, y0, self.color());
            image.display(x, y1, self.color());
        }
        // left and right edges
        for y in y0..=y1 {
            image.display(x0, y, self.color());
            image.display(x1, y, self.color());
        }
    }

    fn color(&self) -> Color {
      self.color.clone()
    }
}

/// =================
///  Triangle
/// =================
#[derive(Debug)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub color: Color,
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Triangle {
            a: a.clone(),
            b: b.clone(),
            c: c.clone(),
            color: random_color(),
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut dyn Displayable) {
        let line_color = self.color();
        Line {
            start: self.a.clone(),
            end: self.b.clone(),
            color: line_color.clone(),
        }.draw(image);
        Line {
            start: self.b.clone(),
            end: self.c.clone(),
            color: line_color.clone(),
        }.draw(image);
        Line {
            start: self.c.clone(),
            end: self.a.clone(),
            color: line_color,
        }.draw(image);
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}
/// =================
///  Circle
/// =================
/// 
#[derive(Debug, Clone )]
pub struct Circle {
    center: Point,
    radius: i32,
    color:  Color,
}

impl Circle {
    //random center and radius.
      pub fn random(max_x: i32, max_y: i32) -> Self {
        let mut rng = rng();

        let max_r = max_x.min(max_y);
        let radius = rng.random_range(10..=max_r.max(10));

        //allow center anywhere so circles can overflow
        let center = Point::random(max_x, max_y);

        let color = Color::rgb(
            rng.random_range(0..=255),
            rng.random_range(0..=255),
            rng.random_range(0..=255),
        );

        Self { center, radius, color }
    }
}

impl Drawable for Circle {
    fn draw(&self, img: &mut dyn Displayable) {
        let cx = self.center.x;
        let cy = self.center.y;
        let mut x = self.radius;
        let mut y = 0;
        let mut err = 1 - x;
        let c = self.color.clone();

        while x >= y {
            // eight-way symmetry
            img.display(cx + x, cy + y, c.clone());
            img.display(cx - x, cy + y, c.clone());
            img.display(cx + x, cy - y, c.clone());
            img.display(cx - x, cy - y, c.clone());
            img.display(cx + y, cy + x, c.clone());
            img.display(cx - y, cy + x, c.clone());
            img.display(cx + y, cy - x, c.clone());
            img.display(cx - y, cy - x, c.clone());

            y += 1;
            if err < 0 {
                err += 2*y + 1;
            } else {
                x -= 1;
                err += 2*(y - x + 1);
            }
        }
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}