use serde::{Deserialize, Serialize};
use super::point::Point;
use super::dimension::Dimension;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasData {
    pub points: Vec<Point>,
    pub dimensions: Vec<Dimension>,
}

impl CanvasData {
    pub fn new() -> Self {
        CanvasData {
            points: Vec::new(),
            dimensions: Vec::new(),
        }
    }

    pub fn with_points_and_dimensions(points: Vec<Point>, dimensions: Vec<Dimension>) -> Self {
        CanvasData {
            points,
            dimensions,
        }
    }
}
