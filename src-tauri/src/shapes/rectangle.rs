use crate::models::Point;

pub fn generate_rectangle(width: f64, height: f64) -> Vec<Point> {
    let half_width = width / 2.0;
    let half_height = height / 2.0;
    
    vec![
        Point::new(-half_width, -half_height), // Bottom-left
        Point::new(half_width, -half_height),  // Bottom-right
        Point::new(half_width, half_height),   // Top-right
        Point::new(-half_width, half_height),  // Top-left
        Point::new(-half_width, -half_height), // Close the shape
    ]
}
