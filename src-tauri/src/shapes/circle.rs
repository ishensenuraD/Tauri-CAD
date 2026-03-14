use crate::models::Point;
use std::f64::consts::PI;

pub fn generate_circle(radius: f64, segments: usize) -> Vec<Point> {
    let mut points = Vec::new();
    
    for i in 0..=segments {
        let angle = 2.0 * PI * (i as f64) / (segments as f64);
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        points.push(Point::new(x, y));
    }
    
    points
}
