use crate::models::Point;

pub fn generate_l_shape(outer_width: f64, outer_height: f64, inner_width: f64, inner_height: f64) -> Vec<Point> {
    let half_outer_width = outer_width / 2.0;
    let half_outer_height = outer_height / 2.0;
    let half_inner_width = inner_width / 2.0;
    let half_inner_height = inner_height / 2.0;
    
    // Generate L-shape points (counterclockwise from bottom-left)
    vec![
        // Outer rectangle bottom edge
        Point::new(-half_outer_width, -half_outer_height),
        Point::new(half_outer_width, -half_outer_height),
        Point::new(half_outer_width, -half_inner_height),
        
        // Inner rectangle cutout
        Point::new(half_inner_width, -half_inner_height),
        Point::new(half_inner_width, half_inner_height),
        Point::new(half_outer_width, half_inner_height),
        
        // Outer rectangle right edge
        Point::new(half_outer_width, half_outer_height),
        Point::new(-half_outer_width, half_outer_height),
        Point::new(-half_outer_width, -half_outer_height), // Close the shape
    ]
}
