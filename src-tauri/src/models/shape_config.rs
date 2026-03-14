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
        self.rotation = rotation;
        self
    }

    pub fn with_flips(mut self, flip_x: bool, flip_y: bool) -> Self {
        self.flip_x = flip_x;
        self.flip_y = flip_y;
        self
    }
}
