use crate::models::Point;
use crate::models::shape_config::{PARAM_WIDTH, PARAM_HEIGHT};

pub fn generate_rectangle(width: f64, height: f64) -> Vec<Point> {
    // Validate parameters
    if width <= 0.0 || height <= 0.0 {
        return Vec::new();
    }

    let half_width = width / 2.0;
    let half_height = height / 2.0;
    
    // Generate rectangle points counter-clockwise starting from bottom-left
    vec![
        Point::new(-half_width, -half_height), // Bottom-left
        Point::new(half_width, -half_height),  // Bottom-right
        Point::new(half_width, half_height),   // Top-right
        Point::new(-half_width, half_height),  // Top-left
        Point::new(-half_width, -half_height), // Close the shape
    ]
}

pub fn generate_rectangle_from_config(config: &crate::models::ShapeConfig) -> Vec<Point> {
    let width = config.get_parameter(PARAM_WIDTH).unwrap_or(100.0);
    let height = config.get_parameter(PARAM_HEIGHT).unwrap_or(50.0);
    generate_rectangle(width, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_rectangle() {
        let points = generate_rectangle(100.0, 50.0);
        assert_eq!(points.len(), 5); // 4 corners + closing point
        
        // Check bottom-left corner
        assert_eq!(points[0].x, -50.0);
        assert_eq!(points[0].y, -25.0);
        
        // Check bottom-right corner
        assert_eq!(points[1].x, 50.0);
        assert_eq!(points[1].y, -25.0);
        
        // Check top-right corner
        assert_eq!(points[2].x, 50.0);
        assert_eq!(points[2].y, 25.0);
        
        // Check top-left corner
        assert_eq!(points[3].x, -50.0);
        assert_eq!(points[3].y, 25.0);
        
        // Check closing point matches first point
        assert_eq!(points[4], points[0]);
    }

    #[test]
    fn test_generate_rectangle_invalid_params() {
        let points = generate_rectangle(0.0, 50.0);
        assert!(points.is_empty());
        
        let points = generate_rectangle(100.0, -10.0);
        assert!(points.is_empty());
    }

    #[test]
    fn test_generate_rectangle_from_config() {
        use std::collections::HashMap;
        
        let mut params = HashMap::new();
        params.insert(PARAM_WIDTH.to_string(), 200.0);
        params.insert(PARAM_HEIGHT.to_string(), 100.0);
        
        let config = crate::models::ShapeConfig::new("rectangle".to_string())
            .with_parameters(params);
        
        let points = generate_rectangle_from_config(&config);
        assert_eq!(points.len(), 5);
        
        // Check dimensions
        assert_eq!(points[1].x - points[0].x, 200.0); // width
        assert_eq!(points[3].y - points[0].y, 100.0); // height
    }
}
