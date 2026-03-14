use crate::models::Point;
use crate::models::shape_config::{PARAM_BASE, PARAM_HEIGHT, PARAM_ANGLE};
use std::f64::consts::PI;

pub fn generate_triangle(base: f64, height: f64, apex_angle_degrees: f64) -> Vec<Point> {
    // Validate parameters
    if base <= 0.0 || height <= 0.0 || apex_angle_degrees <= 0.0 || apex_angle_degrees >= 180.0 {
        return Vec::new();
    }

    let half_base = base / 2.0;
    
    // Calculate the apex position based on the angle
    let apex_angle_radians = apex_angle_degrees * PI / 180.0;
    
    // For an isosceles triangle, the apex height is determined by the base and angle
    // Using trigonometry: tan(angle/2) = (base/2) / height
    let calculated_height = (half_base / (apex_angle_radians / 2.0).tan()).min(height);
    
    // Center the triangle at origin
    let vertical_offset = height / 2.0;
    
    vec![
        Point::new(-half_base, -vertical_offset),                    // Bottom-left
        Point::new(half_base, -vertical_offset),                     // Bottom-right
        Point::new(0.0, -vertical_offset + calculated_height),      // Top
        Point::new(-half_base, -vertical_offset),                    // Close the shape
    ]
}

pub fn generate_triangle_from_config(config: &crate::models::ShapeConfig) -> Vec<Point> {
    let base = config.get_parameter(PARAM_BASE).unwrap_or(100.0);
    let height = config.get_parameter(PARAM_HEIGHT).unwrap_or(80.0);
    let angle = config.get_parameter(PARAM_ANGLE).unwrap_or(60.0);
    generate_triangle(base, height, angle)
}

pub fn generate_equilateral_triangle(side_length: f64) -> Vec<Point> {
    if side_length <= 0.0 {
        return Vec::new();
    }
    
    // For equilateral triangle, all angles are 60 degrees
    let height = side_length * (3.0_f64.sqrt() / 2.0);
    generate_triangle(side_length, height, 60.0)
}

pub fn generate_right_triangle(base: f64, height: f64) -> Vec<Point> {
    if base <= 0.0 || height <= 0.0 {
        return Vec::new();
    }
    
    // For right triangle, apex angle is 90 degrees
    generate_triangle(base, height, 90.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_triangle() {
        let points = generate_triangle(100.0, 80.0, 60.0);
        assert_eq!(points.len(), 4); // 3 corners + closing point
        
        // Check bottom-left corner
        assert_eq!(points[0].x, -50.0);
        assert_eq!(points[0].y, -40.0);
        
        // Check bottom-right corner
        assert_eq!(points[1].x, 50.0);
        assert_eq!(points[1].y, -40.0);
        
        // Check that top point is centered
        assert_eq!(points[2].x, 0.0);
        assert!(points[2].y > -40.0); // Should be above base
    }

    #[test]
    fn test_generate_triangle_invalid_params() {
        let points = generate_triangle(0.0, 50.0, 60.0);
        assert!(points.is_empty());
        
        let points = generate_triangle(100.0, -10.0, 60.0);
        assert!(points.is_empty());
        
        let points = generate_triangle(100.0, 50.0, 0.0);
        assert!(points.is_empty());
        
        let points = generate_triangle(100.0, 50.0, 180.0);
        assert!(points.is_empty());
    }

    #[test]
    fn test_generate_triangle_from_config() {
        use std::collections::HashMap;
        
        let mut params = HashMap::new();
        params.insert(PARAM_BASE.to_string(), 200.0);
        params.insert(PARAM_HEIGHT.to_string(), 100.0);
        params.insert(PARAM_ANGLE.to_string(), 45.0);
        
        let config = crate::models::ShapeConfig::new("triangle".to_string())
            .with_parameters(params);
        
        let points = generate_triangle_from_config(&config);
        assert_eq!(points.len(), 4);
        
        // Check base width
        assert_eq!(points[1].x - points[0].x, 200.0);
    }

    #[test]
    fn test_generate_equilateral_triangle() {
        let points = generate_equilateral_triangle(100.0);
        assert_eq!(points.len(), 4);
        
        // Check base width
        assert_eq!(points[1].x - points[0].x, 100.0);
        
        // Check height (should be sqrt(3)/2 * side)
        let expected_height = 100.0 * (3.0_f64.sqrt() / 2.0);
        let actual_height = points[2].y - points[0].y;
        assert!((actual_height - expected_height).abs() < 0.001);
    }

    #[test]
    fn test_generate_right_triangle() {
        let points = generate_right_triangle(100.0, 80.0);
        assert_eq!(points.len(), 4);
        
        // Check base width
        assert_eq!(points[1].x - points[0].x, 100.0);
        
        // Check that apex is at 90 degrees
        assert_eq!(points[2].x, 0.0); // Centered
    }
}
