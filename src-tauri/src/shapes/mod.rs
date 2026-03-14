pub mod rectangle;
pub mod circle;
pub mod triangle;
pub mod trapezoid;
pub mod lshape;
pub mod generator;

// Basic shape generators
pub use rectangle::generate_rectangle;
pub use circle::generate_circle;
pub use triangle::generate_triangle;
pub use trapezoid::generate_trapezoid;
pub use lshape::generate_l_shape;

// Config-based generators
pub use rectangle::generate_rectangle_from_config;
pub use circle::generate_circle_from_config;
pub use triangle::generate_triangle_from_config;
pub use trapezoid::generate_trapezoid_from_config;
pub use lshape::generate_l_shape_from_config;

// Specialized generators
pub use circle::{generate_circle_smooth, calculate_segments};
pub use triangle::{generate_equilateral_triangle, generate_right_triangle};
pub use trapezoid::{generate_isosceles_trapezoid, generate_right_trapezoid};
pub use lshape::{generate_l_shape_bottom_left, generate_l_shape_bottom_right, generate_l_shape_top_left};

// Master generator and utilities
pub use generator::{
    generate_shape_from_config,
    get_default_config,
    validate_shape_config,
    get_required_parameters,
    get_parameter_description,
};
