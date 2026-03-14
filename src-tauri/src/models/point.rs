use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn origin() -> Self {
        Point::new(0.0, 0.0)
    }

    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn translate(&self, dx: f64, dy: f64) -> Point {
        Point::new(self.x + dx, self.y + dy)
    }

    pub fn scale(&self, factor: f64) -> Point {
        Point::new(self.x * factor, self.y * factor)
    }

    pub fn add(&self, other: &Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }

    pub fn subtract(&self, other: &Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }

    pub fn midpoint(&self, other: &Point) -> Point {
        Point::new((self.x + other.x) / 2.0, (self.y + other.y) / 2.0)
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Option<Point> {
        let mag = self.magnitude();
        if mag == 0.0 {
            None
        } else {
            Some(Point::new(self.x / mag, self.y / mag))
        }
    }

    pub fn dot_product(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross_product(&self, other: &Point) -> f64 {
        self.x * other.y - self.y * other.x
    }
}
