use crate::models::Point;
use std::f64::consts::PI;

pub fn generate_triangle(base: f64, height: f64, apex_angle_degrees: f64) -> Vec<Point> {
    let half_base = base / 2.0;
    
    // Calculate the apex position based on the angle
    let apex_angle_radians = apex_angle_degrees * PI / 180.0;
    let apex_height = (half_base / (apex_angle_radians / 2.0).tan()).min(height);
    
    vec![
        Point::new(-half_base, -height / 2.0),      // Bottom-left
        Point::new(half_base, -height / 2.0),       // Bottom-right
        Point::new(0.0, -height / 2.0 + apex_height), // Top
        Point::new(-half_base, -height / 2.0),      // Close the shape
    ]
}
