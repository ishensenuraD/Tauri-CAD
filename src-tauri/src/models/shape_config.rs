use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeConfig {
    pub shape_type: String,
    pub parameters: HashMap<String, f64>,
    pub rotation: i32, // 0, 90, 180, 270
    pub flip_x: bool,
    pub flip_y: bool,
}

// Supported shape types
pub const SHAPE_RECTANGLE: &str = "rectangle";
pub const SHAPE_CIRCLE: &str = "circle";
pub const SHAPE_TRIANGLE: &str = "triangle";
pub const SHAPE_TRAPEZOID: &str = "trapezoid";
pub const SHAPE_LSHAPE: &str = "lshape";

// Parameter names
pub const PARAM_WIDTH: &str = "width";
pub const PARAM_HEIGHT: &str = "height";
pub const PARAM_RADIUS: &str = "radius";
pub const PARAM_BASE: &str = "base";
pub const PARAM_ANGLE: &str = "angle";
pub const PARAM_TOP_WIDTH: &str = "top_width";
pub const PARAM_BOTTOM_WIDTH: &str = "bottom_width";
pub const PARAM_OUTER_WIDTH: &str = "outer_width";
pub const PARAM_OUTER_HEIGHT: &str = "outer_height";
pub const PARAM_INNER_WIDTH: &str = "inner_width";
pub const PARAM_INNER_HEIGHT: &str = "inner_height";

impl ShapeConfig {
    pub fn new(shape_type: String) -> Self {
        ShapeConfig {
            shape_type,
            parameters: HashMap::new(),
            rotation: 0,
            flip_x: false,
            flip_y: false,
        }
    }

    pub fn with_parameters(mut self, parameters: HashMap<String, f64>) -> Self {
        self.parameters = parameters;
        self
    }

    pub fn with_rotation(mut self, rotation: i32) -> Self {
        self.rotation = self.normalize_rotation(rotation);
        self
    }

    pub fn with_flips(mut self, flip_x: bool, flip_y: bool) -> Self {
        self.flip_x = flip_x;
        self.flip_y = flip_y;
        self
    }

    pub fn set_parameter(&mut self, key: &str, value: f64) {
        self.parameters.insert(key.to_string(), value);
    }

    pub fn get_parameter(&self, key: &str) -> Option<f64> {
        self.parameters.get(key).copied()
    }

    pub fn get_parameter_or_default(&self, key: &str, default: f64) -> f64 {
        self.parameters.get(key).copied().unwrap_or(default)
    }

    pub fn has_parameter(&self, key: &str) -> bool {
        self.parameters.contains_key(key)
    }

    pub fn remove_parameter(&mut self, key: &str) -> Option<f64> {
        self.parameters.remove(key)
    }

    pub fn clear_parameters(&mut self) {
        self.parameters.clear();
    }

    pub fn normalize_rotation(&self, rotation: i32) -> i32 {
        match rotation % 360 {
            r if r < 0 => (r + 360) % 360,
            r => r,
        }
    }

    pub fn is_valid_rotation(&self) -> bool {
        matches!(self.rotation, 0 | 90 | 180 | 270)
    }

    pub fn is_valid_shape_type(&self) -> bool {
        matches!(
            self.shape_type.as_str(),
            SHAPE_RECTANGLE | SHAPE_CIRCLE | SHAPE_TRIANGLE | SHAPE_TRAPEZOID | SHAPE_LSHAPE
        )
    }

    pub fn validate(&self) -> Result<(), String> {
        if !self.is_valid_shape_type() {
            return Err(format!("Invalid shape type: {}", self.shape_type));
        }

        if !self.is_valid_rotation() {
            return Err(format!("Invalid rotation: {}. Must be 0, 90, 180, or 270", self.rotation));
        }

        // Validate parameters based on shape type
        match self.shape_type.as_str() {
            SHAPE_RECTANGLE => self.validate_rectangle_params(),
            SHAPE_CIRCLE => self.validate_circle_params(),
            SHAPE_TRIANGLE => self.validate_triangle_params(),
            SHAPE_TRAPEZOID => self.validate_trapezoid_params(),
            SHAPE_LSHAPE => self.validate_lshape_params(),
            _ => Ok(()),
        }
    }

    fn validate_rectangle_params(&self) -> Result<(), String> {
        let width = self.get_parameter(PARAM_WIDTH).unwrap_or(0.0);
        let height = self.get_parameter(PARAM_HEIGHT).unwrap_or(0.0);
        
        if width <= 0.0 {
            return Err("Rectangle width must be positive".to_string());
        }
        if height <= 0.0 {
            return Err("Rectangle height must be positive".to_string());
        }
        Ok(())
    }

    fn validate_circle_params(&self) -> Result<(), String> {
        let radius = self.get_parameter(PARAM_RADIUS).unwrap_or(0.0);
        
        if radius <= 0.0 {
            return Err("Circle radius must be positive".to_string());
        }
        Ok(())
    }

    fn validate_triangle_params(&self) -> Result<(), String> {
        let base = self.get_parameter(PARAM_BASE).unwrap_or(0.0);
        let height = self.get_parameter(PARAM_HEIGHT).unwrap_or(0.0);
        let angle = self.get_parameter(PARAM_ANGLE).unwrap_or(0.0);
        
        if base <= 0.0 {
            return Err("Triangle base must be positive".to_string());
        }
        if height <= 0.0 {
            return Err("Triangle height must be positive".to_string());
        }
        if angle <= 0.0 || angle >= 180.0 {
            return Err("Triangle angle must be between 0 and 180 degrees".to_string());
        }
        Ok(())
    }

    fn validate_trapezoid_params(&self) -> Result<(), String> {
        let top_width = self.get_parameter(PARAM_TOP_WIDTH).unwrap_or(0.0);
        let bottom_width = self.get_parameter(PARAM_BOTTOM_WIDTH).unwrap_or(0.0);
        let height = self.get_parameter(PARAM_HEIGHT).unwrap_or(0.0);
        
        if top_width <= 0.0 {
            return Err("Trapezoid top width must be positive".to_string());
        }
        if bottom_width <= 0.0 {
            return Err("Trapezoid bottom width must be positive".to_string());
        }
        if height <= 0.0 {
            return Err("Trapezoid height must be positive".to_string());
        }
        Ok(())
    }

    fn validate_lshape_params(&self) -> Result<(), String> {
        let outer_width = self.get_parameter(PARAM_OUTER_WIDTH).unwrap_or(0.0);
        let outer_height = self.get_parameter(PARAM_OUTER_HEIGHT).unwrap_or(0.0);
        let inner_width = self.get_parameter(PARAM_INNER_WIDTH).unwrap_or(0.0);
        let inner_height = self.get_parameter(PARAM_INNER_HEIGHT).unwrap_or(0.0);
        
        if outer_width <= 0.0 {
            return Err("L-shape outer width must be positive".to_string());
        }
        if outer_height <= 0.0 {
            return Err("L-shape outer height must be positive".to_string());
        }
        if inner_width <= 0.0 {
            return Err("L-shape inner width must be positive".to_string());
        }
        if inner_height <= 0.0 {
            return Err("L-shape inner height must be positive".to_string());
        }
        if inner_width >= outer_width {
            return Err("L-shape inner width must be less than outer width".to_string());
        }
        if inner_height >= outer_height {
            return Err("L-shape inner height must be less than outer height".to_string());
        }
        Ok(())
    }
}
