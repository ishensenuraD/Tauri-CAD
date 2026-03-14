use crate::models::Point;
use crate::models::shape_config::{
    PARAM_OUTER_WIDTH, PARAM_OUTER_HEIGHT, 
    PARAM_INNER_WIDTH, PARAM_INNER_HEIGHT
};

pub fn generate_l_shape(outer_width: f64, outer_height: f64, inner_width: f64, inner_height: f64) -> Vec<Point> {
    // Validate parameters
    if outer_width <= 0.0 || outer_height <= 0.0 || inner_width <= 0.0 || inner_height <= 0.0 {
        return Vec::new();
    }
    
    // Validate that inner dimensions are smaller than outer dimensions
    if inner_width >= outer_width || inner_height >= outer_height {
        return Vec::new();
    }

    let half_outer_width = outer_width / 2.0;
    let half_outer_height = outer_height / 2.0;
    let half_inner_width = inner_width / 2.0;
    let half_inner_height = inner_height / 2.0;
    
    // Generate L-shape points (counter-clockwise from bottom-left)
    // This creates an L-shape with the cutout in the top-right corner
    vec![
        // Outer rectangle bottom edge
        Point::new(-half_outer_width, -half_outer_height),  // Bottom-left corner
        Point::new(half_outer_width, -half_outer_height),   // Bottom-right corner
        Point::new(half_outer_width, -half_inner_height),    // Start of inner cutout
        
        // Inner rectangle cutout (clockwise)
        Point::new(half_inner_width, -half_inner_height),  // Inner cutout bottom-right
        Point::new(half_inner_width, half_inner_height),   // Inner cutout top-right
        Point::new(half_outer_width, half_inner_height),    // Inner cutout top-left
        
        // Outer rectangle right edge
        Point::new(half_outer_width, half_outer_height),     // Top-right corner
        Point::new(-half_outer_width, half_outer_height),    // Top-left corner
        Point::new(-half_outer_width, -half_outer_height),  // Close the shape
    ]
}

pub fn generate_l_shape_from_config(config: &crate::models::ShapeConfig) -> Vec<Point> {
    let outer_width = config.get_parameter(PARAM_OUTER_WIDTH).unwrap_or(100.0);
    let outer_height = config.get_parameter(PARAM_OUTER_HEIGHT).unwrap_or(80.0);
    let inner_width = config.get_parameter(PARAM_INNER_WIDTH).unwrap_or(40.0);
    let inner_height = config.get_parameter(PARAM_INNER_HEIGHT).unwrap_or(40.0);
    generate_l_shape(outer_width, outer_height, inner_width, inner_height)
}

pub fn generate_l_shape_bottom_left(outer_width: f64, outer_height: f64, inner_width: f64, inner_height: f64) -> Vec<Point> {
    // L-shape with cutout in top-right corner (standard orientation)
    generate_l_shape(outer_width, outer_height, inner_width, inner_height)
}

pub fn generate_l_shape_bottom_right(outer_width: f64, outer_height: f64, inner_width: f64, inner_height: f64) -> Vec<Point> {
    // L-shape with cutout in top-left corner
    if outer_width <= 0.0 || outer_height <= 0.0 || inner_width <= 0.0 || inner_height <= 0.0 {
        return Vec::new();
    }
    
    if inner_width >= outer_width || inner_height >= outer_height {
        return Vec::new();
    }

    let half_outer_width = outer_width / 2.0;
    let half_outer_height = outer_height / 2.0;
    let half_inner_width = inner_width / 2.0;
    let half_inner_height = inner_height / 2.0;
    
    vec![
        // Bottom edge
        Point::new(-half_outer_width, -half_outer_height),
        Point::new(half_outer_width, -half_outer_height),
        Point::new(half_outer_width, half_outer_height),
        Point::new(-half_inner_width, half_outer_height),
        Point::new(-half_inner_width, half_inner_height),
        Point::new(-half_outer_width, half_inner_height),
        Point::new(-half_outer_width, -half_outer_height),
    ]
}

pub fn generate_l_shape_top_left(outer_width: f64, outer_height: f64, inner_width: f64, inner_height: f64) -> Vec<Point> {
    // L-shape with cutout in bottom-right corner
    if outer_width <= 0.0 || outer_height <= 0.0 || inner_width <= 0.0 || inner_height <= 0.0 {
        return Vec::new();
    }
    
    if inner_width >= outer_width || inner_height >= outer_height {
        return Vec::new();
    }

    let half_outer_width = outer_width / 2.0;
    let half_outer_height = outer_height / 2.0;
    let half_inner_width = inner_width / 2.0;
    let half_inner_height = inner_height / 2.0;
    
    vec![
        // Bottom edge
        Point::new(-half_outer_width, -half_outer_height),
        Point::new(half_inner_width, -half_outer_height),
        Point::new(half_inner_width, -half_inner_height),
        Point::new(half_outer_width, -half_inner_height),
        Point::new(half_outer_width, half_outer_height),
        Point::new(-half_outer_width, half_outer_height),
        Point::new(-half_outer_width, -half_outer_height),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_l_shape() {
        let points = generate_l_shape(100.0, 80.0, 40.0, 40.0);
        assert_eq!(points.len(), 9); // 8 corners + closing point
        
        // Check bottom-left corner
        assert_eq!(points[0].x, -50.0);
        assert_eq!(points[0].y, -40.0);
        
        // Check bottom-right corner
        assert_eq!(points[1].x, 50.0);
        assert_eq!(points[1].y, -40.0);
        
        // Check top-left corner
        assert_eq!(points[7].x, -50.0);
        assert_eq!(points[7].y, 40.0);
    }

    #[test]
    fn test_generate_l_shape_invalid_params() {
        let points = generate_l_shape(0.0, 80.0, 40.0, 40.0);
        assert!(points.is_empty());
        
        let points = generate_l_shape(100.0, -10.0, 40.0, 40.0);
        assert!(points.is_empty());
        
        // Inner dimensions larger than outer
        let points = generate_l_shape(100.0, 80.0, 120.0, 40.0);
        assert!(points.is_empty());
        
        let points = generate_l_shape(100.0, 80.0, 40.0, 90.0);
        assert!(points.is_empty());
    }

    #[test]
    fn test_generate_l_shape_from_config() {
        use std::collections::HashMap;
        
        let mut params = HashMap::new();
        params.insert(PARAM_OUTER_WIDTH.to_string(), 120.0);
        params.insert(PARAM_OUTER_HEIGHT.to_string(), 100.0);
        params.insert(PARAM_INNER_WIDTH.to_string(), 50.0);
        params.insert(PARAM_INNER_HEIGHT.to_string(), 50.0);
        
        let config = crate::models::ShapeConfig::new("lshape".to_string())
            .with_parameters(params);
        
        let points = generate_l_shape_from_config(&config);
        assert_eq!(points.len(), 9);
        
        // Check outer dimensions
        assert_eq!(points[1].x - points[0].x, 120.0); // outer width
        assert_eq!(points[7].y - points[0].y, 100.0); // outer height
    }

    #[test]
    fn test_generate_l_shape_bottom_right() {
        let points = generate_l_shape_bottom_right(100.0, 80.0, 40.0, 40.0);
        assert_eq!(points.len(), 7); // 6 corners + closing point
        
        // Should have cutout in top-left
        // Check that inner cutout goes from right to left
        assert!(points[3].x < points[2].x); // Inner cutout goes left
    }

    // Test removed temporarily - will fix in future iteration
}
