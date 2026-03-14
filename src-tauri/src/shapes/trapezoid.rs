use crate::models::Point;
use crate::models::shape_config::{PARAM_TOP_WIDTH, PARAM_BOTTOM_WIDTH, PARAM_HEIGHT};

pub fn generate_trapezoid(top_width: f64, bottom_width: f64, height: f64) -> Vec<Point> {
    // Validate parameters
    if top_width <= 0.0 || bottom_width <= 0.0 || height <= 0.0 {
        return Vec::new();
    }

    let half_top_width = top_width / 2.0;
    let half_bottom_width = bottom_width / 2.0;
    let half_height = height / 2.0;
    
    // Center the trapezoid at origin
    // Generate points counter-clockwise starting from bottom-left
    vec![
        Point::new(-half_bottom_width, -half_height),  // Bottom-left
        Point::new(half_bottom_width, -half_height),   // Bottom-right
        Point::new(half_top_width, half_height),       // Top-right
        Point::new(-half_top_width, half_height),      // Top-left
        Point::new(-half_bottom_width, -half_height),  // Close the shape
    ]
}

pub fn generate_trapezoid_from_config(config: &crate::models::ShapeConfig) -> Vec<Point> {
    let top_width = config.get_parameter(PARAM_TOP_WIDTH).unwrap_or(60.0);
    let bottom_width = config.get_parameter(PARAM_BOTTOM_WIDTH).unwrap_or(100.0);
    let height = config.get_parameter(PARAM_HEIGHT).unwrap_or(50.0);
    generate_trapezoid(top_width, bottom_width, height)
}

pub fn generate_isosceles_trapezoid(bottom_width: f64, top_width: f64, height: f64) -> Vec<Point> {
    // Ensure bottom is wider than top for isosceles trapezoid
    let (actual_bottom, actual_top) = if bottom_width >= top_width {
        (bottom_width, top_width)
    } else {
        (top_width, bottom_width)
    };
    
    generate_trapezoid(actual_top, actual_bottom, height)
}

pub fn generate_right_trapezoid(bottom_width: f64, top_width: f64, height: f64) -> Vec<Point> {
    if bottom_width <= 0.0 || top_width <= 0.0 || height <= 0.0 {
        return Vec::new();
    }

    let half_height = height / 2.0;
    
    // Right trapezoid has one vertical side (left side)
    // Generate points counter-clockwise starting from bottom-left
    vec![
        Point::new(0.0, -half_height),                    // Bottom-left (aligned vertically)
        Point::new(bottom_width, -half_height),                // Bottom-right
        Point::new(top_width, half_height),                    // Top-right
        Point::new(0.0, half_height),                       // Top-left (aligned vertically)
        Point::new(0.0, -half_height),                    // Close the shape
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_trapezoid() {
        let points = generate_trapezoid(60.0, 100.0, 50.0);
        assert_eq!(points.len(), 5); // 4 corners + closing point
        
        // Check bottom-left corner
        assert_eq!(points[0].x, -50.0);
        assert_eq!(points[0].y, -25.0);
        
        // Check bottom-right corner
        assert_eq!(points[1].x, 50.0);
        assert_eq!(points[1].y, -25.0);
        
        // Check top-right corner
        assert_eq!(points[2].x, 30.0);
        assert_eq!(points[2].y, 25.0);
        
        // Check top-left corner
        assert_eq!(points[3].x, -30.0);
        assert_eq!(points[3].y, 25.0);
    }

    #[test]
    fn test_generate_trapezoid_invalid_params() {
        let points = generate_trapezoid(0.0, 100.0, 50.0);
        assert!(points.is_empty());
        
        let points = generate_trapezoid(60.0, -10.0, 50.0);
        assert!(points.is_empty());
        
        let points = generate_trapezoid(60.0, 100.0, 0.0);
        assert!(points.is_empty());
    }

    #[test]
    fn test_generate_trapezoid_from_config() {
        use std::collections::HashMap;
        
        let mut params = HashMap::new();
        params.insert(PARAM_TOP_WIDTH.to_string(), 40.0);
        params.insert(PARAM_BOTTOM_WIDTH.to_string(), 120.0);
        params.insert(PARAM_HEIGHT.to_string(), 80.0);
        
        let config = crate::models::ShapeConfig::new("trapezoid".to_string())
            .with_parameters(params);
        
        let points = generate_trapezoid_from_config(&config);
        assert_eq!(points.len(), 5);
        
        // Check bottom width
        assert_eq!(points[1].x - points[0].x, 120.0);
        
        // Check top width
        assert_eq!(points[2].x - points[3].x, 40.0);
    }

    #[test]
    fn test_generate_isosceles_trapezoid() {
        let points = generate_isosceles_trapezoid(100.0, 60.0, 50.0);
        assert_eq!(points.len(), 5);
        
        // Should be centered
        let left_x = points[0].x.min(points[3].x);
        let right_x = points[1].x.max(points[2].x);
        let center_x = (left_x + right_x) / 2.0;
        assert!((center_x - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_generate_right_trapezoid() {
        let points = generate_right_trapezoid(100.0, 60.0, 50.0);
        assert_eq!(points.len(), 5);
        
        // Check that left side is vertical
        assert_eq!(points[0].x, points[3].x);
        assert_eq!(points[0].x, 0.0);
        
        // Check bottom width
        assert_eq!(points[1].x - points[0].x, 100.0);
        
        // Check top width
        assert_eq!(points[2].x - points[3].x, 60.0);
    }
}
