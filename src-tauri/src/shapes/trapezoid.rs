use crate::models::Point;

pub fn generate_trapezoid(top_width: f64, bottom_width: f64, height: f64) -> Vec<Point> {
    let half_top_width = top_width / 2.0;
    let half_bottom_width = bottom_width / 2.0;
    let half_height = height / 2.0;
    
    // Center the trapezoid at origin
    vec![
        Point::new(-half_bottom_width, -half_height),  // Bottom-left
        Point::new(half_bottom_width, -half_height),   // Bottom-right
        Point::new(half_top_width, half_height),       // Top-right
        Point::new(-half_top_width, half_height),      // Top-left
        Point::new(-half_bottom_width, -half_height),  // Close the shape
    ]
}
