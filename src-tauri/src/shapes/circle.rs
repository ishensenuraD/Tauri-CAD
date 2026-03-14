use crate::models::Point;
use crate::models::shape_config::PARAM_RADIUS;
use std::f64::consts::PI;

pub fn generate_circle(radius: f64, segments: usize) -> Vec<Point> {
    // Validate parameters
    if radius <= 0.0 || segments < 3 {
        return Vec::new();
    }

    let mut points = Vec::with_capacity(segments + 1);
    
    for i in 0..=segments {
        let angle = 2.0 * PI * (i as f64) / (segments as f64);
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        points.push(Point::new(x, y));
    }
    
    points
}

pub fn generate_circle_from_config(config: &crate::models::ShapeConfig) -> Vec<Point> {
    let radius = config.get_parameter(PARAM_RADIUS).unwrap_or(50.0);
    // Use more segments for larger circles for smoother appearance
    let segments = calculate_segments(radius);
    generate_circle(radius, segments)
}

pub fn calculate_segments(radius: f64) -> usize {
    // Minimum 16 segments, increase with radius for smoother circles
    let base_segments = 16;
    let additional_segments = (radius / 10.0) as usize;
    (base_segments + additional_segments).min(64) // Cap at 64 segments
}

pub fn generate_circle_smooth(radius: f64) -> Vec<Point> {
    generate_circle(radius, calculate_segments(radius))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_circle() {
        let points = generate_circle(50.0, 4);
        assert_eq!(points.len(), 5); // 4 points + closing point
        
        // Check that all points are at the correct distance from origin
        for point in &points {
            let distance = (point.x * point.x + point.y * point.y).sqrt();
            assert!((distance - 50.0).abs() < 0.001);
        }
    }

    #[test]
    fn test_generate_circle_invalid_params() {
        let points = generate_circle(0.0, 16);
        assert!(points.is_empty());
        
        let points = generate_circle(50.0, 2);
        assert!(points.is_empty());
        
        let points = generate_circle(-10.0, 16);
        assert!(points.is_empty());
    }

    #[test]
    fn test_generate_circle_from_config() {
        use std::collections::HashMap;
        
        let mut params = HashMap::new();
        params.insert(PARAM_RADIUS.to_string(), 100.0);
        
        let config = crate::models::ShapeConfig::new("circle".to_string())
            .with_parameters(params);
        
        let points = generate_circle_from_config(&config);
        assert!(!points.is_empty());
        
        // Check that all points are at the correct distance
        for point in &points {
            let distance = (point.x * point.x + point.y * point.y).sqrt();
            assert!((distance - 100.0).abs() < 0.001);
        }
    }

    #[test]
    fn test_calculate_segments() {
        assert_eq!(calculate_segments(5.0), 16);
        assert_eq!(calculate_segments(20.0), 18);
        assert_eq!(calculate_segments(100.0), 26);
        assert_eq!(calculate_segments(500.0), 64); // Should be capped
    }

    #[test]
    fn test_generate_circle_smooth() {
        let points = generate_circle_smooth(50.0);
        assert!(!points.is_empty());
        assert!(points.len() >= 17); // At least 16 + closing point
        
        // Check that it's actually smooth (more than 4 segments)
        assert!(points.len() > 5);
    }
}
