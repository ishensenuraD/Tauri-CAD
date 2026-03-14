use crate::models::{Point, ShapeConfig};
use crate::models::shape_config::*;
use super::*;

/// Master shape generator that creates points based on configuration
pub fn generate_shape_from_config(config: &ShapeConfig) -> Vec<Point> {
    match config.shape_type.as_str() {
        SHAPE_RECTANGLE => generate_rectangle_from_config(config),
        SHAPE_CIRCLE => generate_circle_from_config(config),
        SHAPE_TRIANGLE => generate_triangle_from_config(config),
        SHAPE_TRAPEZOID => generate_trapezoid_from_config(config),
        SHAPE_LSHAPE => generate_l_shape_from_config(config),
        _ => Vec::new(), // Return empty for unknown shape types
    }
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
        (SHAPE_RECTANGLE, PARAM_WIDTH) => "Width of the rectangle",
        (SHAPE_RECTANGLE, PARAM_HEIGHT) => "Height of the rectangle",
        
        // Circle
        (SHAPE_CIRCLE, PARAM_RADIUS) => "Radius of the circle",
        
        // Triangle
        (SHAPE_TRIANGLE, PARAM_BASE) => "Base width of the triangle",
        (SHAPE_TRIANGLE, PARAM_HEIGHT) => "Height of the triangle",
        (SHAPE_TRIANGLE, PARAM_ANGLE) => "Apex angle in degrees",
        
        // Trapezoid
        (SHAPE_TRAPEZOID, PARAM_TOP_WIDTH) => "Width of the top edge",
        (SHAPE_TRAPEZOID, PARAM_BOTTOM_WIDTH) => "Width of the bottom edge",
        (SHAPE_TRAPEZOID, PARAM_HEIGHT) => "Height of the trapezoid",
        
        // L-Shape
        (SHAPE_LSHAPE, PARAM_OUTER_WIDTH) => "Total width of the L-shape",
        (SHAPE_LSHAPE, PARAM_OUTER_HEIGHT) => "Total height of the L-shape",
        (SHAPE_LSHAPE, PARAM_INNER_WIDTH) => "Width of the inner cutout",
        (SHAPE_LSHAPE, PARAM_INNER_HEIGHT) => "Height of the inner cutout",
        
        _ => "Parameter",
    }
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
    fn test_get_default_config() {
        let config = get_default_config(SHAPE_CIRCLE);
        assert_eq!(config.shape_type, SHAPE_CIRCLE);
        assert_eq!(config.get_parameter(PARAM_RADIUS), Some(50.0));
    }

    #[test]
    fn test_validate_shape_config() {
        let mut config = get_default_config(SHAPE_RECTANGLE);
        
        // Valid config should pass
        assert!(validate_shape_config(&config).is_ok());
        
        // Invalid rotation should fail
        config.rotation = 45;
        assert!(validate_shape_config(&config).is_err());
    }

    #[test]
    fn test_get_required_parameters() {
        let params = get_required_parameters(SHAPE_TRIANGLE);
        assert_eq!(params.len(), 3);
        assert!(params.contains(&PARAM_BASE));
        assert!(params.contains(&PARAM_HEIGHT));
        assert!(params.contains(&PARAM_ANGLE));
    }

    #[test]
    fn test_get_parameter_description() {
        let desc = get_parameter_description(SHAPE_RECTANGLE, PARAM_WIDTH);
        assert_eq!(desc, "Width of the rectangle");
    }

    #[test]
    fn test_generate_shape_unknown_type() {
        let config = ShapeConfig::new("unknown".to_string());
        let points = generate_shape_from_config(&config);
        assert!(points.is_empty());
    }
}
