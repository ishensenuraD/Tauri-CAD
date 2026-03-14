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

    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    pub fn add_dimension(&mut self, dimension: Dimension) {
        self.dimensions.push(dimension);
    }

    pub fn clear(&mut self) {
        self.points.clear();
        self.dimensions.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty() && self.dimensions.is_empty()
    }

    pub fn bounds(&self) -> Option<(Point, Point)> {
        if self.points.is_empty() {
            return None;
        }

        let mut min_x = self.points[0].x;
        let mut min_y = self.points[0].y;
        let mut max_x = self.points[0].x;
        let mut max_y = self.points[0].y;

        for point in &self.points[1..] {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
        }

        Some((Point::new(min_x, min_y), Point::new(max_x, max_y)))
    }

    pub fn center(&self) -> Option<Point> {
        if let Some((min, max)) = self.bounds() {
            Some(min.midpoint(&max))
        } else {
            None
        }
    }

    pub fn translate(&mut self, dx: f64, dy: f64) {
        for point in &mut self.points {
            *point = point.translate(dx, dy);
        }
        for dimension in &mut self.dimensions {
            dimension.start = dimension.start.translate(dx, dy);
            dimension.end = dimension.end.translate(dx, dy);
        }
    }

    pub fn scale(&mut self, factor: f64, center: Option<Point>) {
        let center = center.unwrap_or_else(|| Point::origin());
        
        for point in &mut self.points {
            let translated = point.subtract(&center);
            let scaled = translated.scale(factor);
            *point = scaled.add(&center);
        }
        
        for dimension in &mut self.dimensions {
            let start_translated = dimension.start.subtract(&center);
            let start_scaled = start_translated.scale(factor);
            dimension.start = start_scaled.add(&center);
            
            let end_translated = dimension.end.subtract(&center);
            let end_scaled = end_translated.scale(factor);
            dimension.end = end_scaled.add(&center);
        }
    }
}
