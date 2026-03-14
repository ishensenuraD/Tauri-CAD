use serde::{Deserialize, Serialize};
use super::point::Point;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimension {
    pub start: Point,
    pub end: Point,
    pub offset: f64,
    pub label: String,
}

impl Dimension {
    pub fn new(start: Point, end: Point, offset: f64, label: String) -> Self {
        Dimension {
            start,
            end,
            offset,
            label,
        }
    }

    pub fn length(&self) -> f64 {
        self.start.distance_to(&self.end)
    }

    pub fn midpoint(&self) -> Point {
        self.start.midpoint(&self.end)
    }

    pub fn direction(&self) -> Option<Point> {
        let direction = self.end.subtract(&self.start);
        direction.normalize()
    }

    pub fn perpendicular(&self) -> Option<Point> {
        if let Some(direction) = self.direction() {
            // Rotate 90 degrees counter-clockwise: (x, y) -> (-y, x)
            Some(Point::new(-direction.y, direction.x))
        } else {
            None
        }
    }

    pub fn offset_line(&self) -> Option<(Point, Point)> {
        if let Some(perp) = self.perpendicular() {
            let offset_vector = perp.scale(self.offset);
            let offset_start = self.start.add(&offset_vector);
            let offset_end = self.end.add(&offset_vector);
            Some((offset_start, offset_end))
        } else {
            None
        }
    }

    pub fn extension_lines(&self, extension_length: f64) -> Option<(Point, Point, Point, Point)> {
        if let Some(direction) = self.direction() {
            let extension = direction.scale(extension_length);
            
            // Extension lines go perpendicular to the dimension line
            if let Some(perp) = self.perpendicular() {
                let start_extension = perp.scale(extension_length);
                let end_extension = perp.scale(extension_length);
                
                let start_ext1 = self.start.add(&start_extension);
                let start_ext2 = self.start.subtract(&start_extension);
                let end_ext1 = self.end.add(&end_extension);
                let end_ext2 = self.end.subtract(&end_extension);
                
                Some((start_ext1, start_ext2, end_ext1, end_ext2))
            } else {
                None
            }
        } else {
            None
        }
    }
}
