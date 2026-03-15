pub mod transform;
pub mod dimensions;

// Transformation functions
pub use transform::{
    rotate_point,
    rotate_point_origin,
    flip_point_x,
    flip_point_y,
    flip_point_x_origin,
    flip_point_y_origin,
    transform_points,
    transform_points_around_center,
    calculate_center,
    calculate_bounds,
    scale_points,
    translate_points,
    normalize_rotation,
    is_valid_rotation,
};

// Dimension generation functions
pub use dimensions::{
    generate_dimensions,
    generate_radial_dimensions,
    generate_angular_dimensions,
    generate_center_marks,
};

pub use transform::*;
pub use dimensions::*;
