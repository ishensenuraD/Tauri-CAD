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
}
