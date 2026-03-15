use crate::models::{Point, ShapeConfig};
use crate::models::shape_config::*;
use crate::geometry::transform;
use super::*;

/// Master shape generator that creates points based on configuration
pub fn generate_shape_from_config(config: &ShapeConfig) -> Vec<Point> {
    // Generate base shape points
    let base_points = match config.shape_type.as_str() {
        SHAPE_RECTANGLE => generate_rectangle_from_config(config),
        SHAPE_CIRCLE => generate_circle_from_config(config),
        SHAPE_TRIANGLE => generate_triangle_from_config(config),
        SHAPE_TRAPEZOID => generate_trapezoid_from_config(config),
        SHAPE_LSHAPE => generate_l_shape_from_config(config),
        _ => Vec::new(), // Return empty for unknown shape types
    };

    // Apply transformations
    transform::transform_points(&base_points, config.rotation, config.flip_x, config.flip_y)
}

/// Generate shape with custom transformations
pub fn generate_shape_with_transforms(
    config: &ShapeConfig,
    custom_rotation: Option<f64>,
    custom_flip_x: Option<bool>,
    custom_flip_y: Option<bool>
) -> Vec<Point> {
    // Generate base shape points
    let base_points = match config.shape_type.as_str() {
        SHAPE_RECTANGLE => generate_rectangle_from_config(config),
        SHAPE_CIRCLE => generate_circle_from_config(config),
        SHAPE_TRIANGLE => generate_triangle_from_config(config),
        SHAPE_TRAPEZOID => generate_trapezoid_from_config(config),
        SHAPE_LSHAPE => generate_l_shape_from_config(config),
        _ => Vec::new(),
    };

    // Use custom transformations or fall back to config
    let rotation = custom_rotation.unwrap_or(config.rotation as f64);
    let flip_x = custom_flip_x.unwrap_or(config.flip_x);
    let flip_y = custom_flip_y.unwrap_or(config.flip_y);
    
    // Apply transformations
    transform::transform_points_around_center(&base_points, transform::calculate_center(&base_points), rotation, flip_x, flip_y)
}

/// Generate transformed shape around specific center
pub fn generate_shape_around_center(
    config: &ShapeConfig,
    center: Point,
    rotation: f64,
    flip_x: bool,
    flip_y: bool
) -> Vec<Point> {
    // Generate base shape points
    let base_points = match config.shape_type.as_str() {
        SHAPE_RECTANGLE => generate_rectangle_from_config(config),
        SHAPE_CIRCLE => generate_circle_from_config(config),
        SHAPE_TRIANGLE => generate_triangle_from_config(config),
        SHAPE_TRAPEZOID => generate_trapezoid_from_config(config),
        SHAPE_LSHAPE => generate_l_shape_from_config(config),
        _ => Vec::new(),
    };

    // Apply transformations around specified center
    transform::transform_points_around_center(&base_points, center, rotation, flip_x, flip_y)
}

/// Get default configuration for a shape type
pub fn get_default_config(shape_type: &str) -> ShapeConfig {
    let mut config = ShapeConfig::new(shape_type.to_string());
    
    // Set default parameters based on shape type
    match shape_type {
        SHAPE_RECTANGLE => {
            config.set_parameter(PARAM_WIDTH, 100.0);
            config.set_parameter(PARAM_HEIGHT, 50.0);
        },
        SHAPE_CIRCLE => {
            config.set_parameter(PARAM_RADIUS, 50.0);
        },
        SHAPE_TRIANGLE => {
            config.set_parameter(PARAM_BASE, 100.0);
            config.set_parameter(PARAM_HEIGHT, 80.0);
            config.set_parameter(PARAM_ANGLE, 60.0);
        },
        SHAPE_TRAPEZOID => {
            config.set_parameter(PARAM_TOP_WIDTH, 60.0);
            config.set_parameter(PARAM_BOTTOM_WIDTH, 100.0);
            config.set_parameter(PARAM_HEIGHT, 50.0);
        },
        SHAPE_LSHAPE => {
            config.set_parameter(PARAM_OUTER_WIDTH, 100.0);
            config.set_parameter(PARAM_OUTER_HEIGHT, 80.0);
            config.set_parameter(PARAM_INNER_WIDTH, 40.0);
            config.set_parameter(PARAM_INNER_HEIGHT, 40.0);
        },
        _ => {} // Unknown shape type, leave empty
    }
    
    config
}

/// Validate shape configuration and return detailed errors
pub fn validate_shape_config(config: &ShapeConfig) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    
    // Validate shape type
    if !config.is_valid_shape_type() {
        errors.push(format!("Invalid shape type: {}", config.shape_type));
    }
    
    // Validate rotation
    if !config.is_valid_rotation() {
        errors.push(format!("Invalid rotation: {}. Must be 0, 90, 180, or 270", config.rotation));
    }
    
    // Use the built-in validation from ShapeConfig
    if let Err(validation_error) = config.validate() {
        errors.push(validation_error);
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Get required parameters for a shape type
pub fn get_required_parameters(shape_type: &str) -> Vec<&'static str> {
    match shape_type {
        SHAPE_RECTANGLE => vec![PARAM_WIDTH, PARAM_HEIGHT],
        SHAPE_CIRCLE => vec![PARAM_RADIUS],
        SHAPE_TRIANGLE => vec![PARAM_BASE, PARAM_HEIGHT, PARAM_ANGLE],
        SHAPE_TRAPEZOID => vec![PARAM_TOP_WIDTH, PARAM_BOTTOM_WIDTH, PARAM_HEIGHT],
        SHAPE_LSHAPE => vec![
            PARAM_OUTER_WIDTH,
            PARAM_OUTER_HEIGHT,
            PARAM_INNER_WIDTH,
            PARAM_INNER_HEIGHT
        ],
        _ => Vec::new(),
    }
}

/// Get parameter description for UI display
pub fn get_parameter_description(shape_type: &str, param_name: &str) -> &'static str {
    match (shape_type, param_name) {
        // Rectangle
        (SHAPE_RECTANGLE, PARAM_WIDTH) => "Width of rectangle",
        (SHAPE_RECTANGLE, PARAM_HEIGHT) => "Height of rectangle",
        
        // Circle
        (SHAPE_CIRCLE, PARAM_RADIUS) => "Radius of circle",
        
        // Triangle
        (SHAPE_TRIANGLE, PARAM_BASE) => "Base width of triangle",
        (SHAPE_TRIANGLE, PARAM_HEIGHT) => "Height of triangle",
        (SHAPE_TRIANGLE, PARAM_ANGLE) => "Apex angle in degrees",
        
        // Trapezoid
        (SHAPE_TRAPEZOID, PARAM_TOP_WIDTH) => "Width of top edge",
        (SHAPE_TRAPEZOID, PARAM_BOTTOM_WIDTH) => "Width of bottom edge",
        (SHAPE_TRAPEZOID, PARAM_HEIGHT) => "Height of trapezoid",
        
        // L-Shape
        (SHAPE_LSHAPE, PARAM_OUTER_WIDTH) => "Total width of L-shape",
        (SHAPE_LSHAPE, PARAM_OUTER_HEIGHT) => "Total height of L-shape",
        (SHAPE_LSHAPE, PARAM_INNER_WIDTH) => "Width of inner cutout",
        (SHAPE_LSHAPE, PARAM_INNER_HEIGHT) => "Height of inner cutout",
        
        _ => "Parameter",
    }
}

/// Get shape bounds after transformations
pub fn get_shape_bounds(config: &ShapeConfig) -> Option<(Point, Point)> {
    let points = generate_shape_from_config(config);
    transform::calculate_bounds(&points)
}

/// Get shape center after transformations
pub fn get_shape_center(config: &ShapeConfig) -> Point {
    let points = generate_shape_from_config(config);
    transform::calculate_center(&points)
}

/// Apply transformations to existing points
pub fn apply_transformations(
    points: &[Point],
    rotation: f64,
    flip_x: bool,
    flip_y: bool,
    center: Option<Point>
) -> Vec<Point> {
    let center = center.unwrap_or_else(|| transform::calculate_center(points));
    transform::transform_points_around_center(points, center, rotation, flip_x, flip_y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_shape_from_config() {
        let config = get_default_config(SHAPE_RECTANGLE);
        let points = generate_shape_from_config(&config);
        assert_eq!(points.len(), 5); // Rectangle has 4 corners + closing point
    }

    #[test]
    fn test_generate_shape_with_transforms() {
        let config = get_default_config(SHAPE_RECTANGLE);
        let points = generate_shape_with_transforms(&config, Some(90.0), Some(true), Some(false));
        assert_eq!(points.len(), 5);
        
        // Points should be transformed
        let original_points = generate_shape_from_config(&config);
        assert!(points != original_points);
    }

    #[test]
    fn test_get_default_config() {
        let config = get_default_config(SHAPE_CIRCLE);
        assert_eq!(config.shape_type, SHAPE_CIRCLE);
        assert_eq!(config.get_parameter(PARAM_RADIUS), Some(50.0));
    }

    #[test]
    fn test_get_parameter_description() {
        let desc = get_parameter_description(SHAPE_RECTANGLE, PARAM_WIDTH);
        assert_eq!(desc, "Width of rectangle");
    }

    #[test]
    fn test_generate_shape_unknown_type() {
        let config = ShapeConfig::new("unknown".to_string());
        let points = generate_shape_from_config(&config);
        assert!(points.is_empty());
    }

    #[test]
    fn test_get_shape_bounds() {
        let config = get_default_config(SHAPE_RECTANGLE);
        let bounds = get_shape_bounds(&config);
        assert!(bounds.is_some());
    }

    #[test]
    fn test_get_shape_center() {
        let config = get_default_config(SHAPE_CIRCLE);
        let center = get_shape_center(&config);
        // Circle should be centered at origin even after transformations
        assert!((center.x - 0.0).abs() < 0.001);
        assert!((center.y - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_apply_transformations() {
        let points = vec![
            Point::new(1.0, 0.0),
            Point::new(0.0, 1.0),
        ];
        
        let transformed = apply_transformations(&points, 90.0, false, false, None);
        assert_eq!(transformed.len(), points.len());
        // Should be rotated 90 degrees
        assert!(transformed != points);
    }
}
