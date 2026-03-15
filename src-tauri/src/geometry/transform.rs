// Geometry transformations will be implemented in Phase 4
use crate::models::Point;
use std::f64::consts::PI;

/// Rotate a point around a center by the given angle in degrees
pub fn rotate_point(point: Point, angle_degrees: f64, center: Point) -> Point {
    let angle_radians = angle_degrees * PI / 180.0;
    let cos_angle = angle_radians.cos();
    let sin_angle = angle_radians.sin();
    
    // Translate point to origin
    let translated = point.subtract(&center);
    
    // Apply rotation matrix
    let rotated_x = translated.x * cos_angle - translated.y * sin_angle;
    let rotated_y = translated.x * sin_angle + translated.y * cos_angle;
    
    // Translate back and return
    Point::new(rotated_x + center.x, rotated_y + center.y)
}

/// Rotate a point around the origin by the given angle in degrees
pub fn rotate_point_origin(point: Point, angle_degrees: f64) -> Point {
    rotate_point(point, angle_degrees, Point::origin())
}

/// Flip a point horizontally around a center point
pub fn flip_point_x(point: Point, center: Point) -> Point {
    // Calculate distance from center and mirror
    let distance = point.x - center.x;
    Point::new(center.x - distance, point.y)
}

/// Flip a point vertically around a center point
pub fn flip_point_y(point: Point, center: Point) -> Point {
    // Calculate distance from center and mirror
    let distance = point.y - center.y;
    Point::new(point.x, center.y - distance)
}

/// Flip a point horizontally around the y-axis (x=0)
pub fn flip_point_x_origin(point: Point) -> Point {
    flip_point_x(point, Point::origin())
}

/// Flip a point vertically around the x-axis (y=0)
pub fn flip_point_y_origin(point: Point) -> Point {
    flip_point_y(point, Point::origin())
}

/// Apply transformations to a vector of points based on configuration
pub fn transform_points(points: &[Point], rotation: i32, flip_x: bool, flip_y: bool) -> Vec<Point> {
    let center = calculate_center(points);
    let mut transformed_points = points.to_vec();
    
    // Apply rotation
    if rotation != 0 {
        transformed_points = transformed_points
            .iter()
            .map(|&point| rotate_point(point, rotation as f64, center))
            .collect();
    }
    
    // Apply horizontal flip
    if flip_x {
        transformed_points = transformed_points
            .iter()
            .map(|&point| flip_point_x(point, center))
            .collect();
    }
    
    // Apply vertical flip
    if flip_y {
        transformed_points = transformed_points
            .iter()
            .map(|&point| flip_point_y(point, center))
            .collect();
    }
    
    transformed_points
}

/// Apply transformations to points with a specific center
pub fn transform_points_around_center(
    points: &[Point], 
    center: Point, 
    rotation: f64, 
    flip_x: bool, 
    flip_y: bool
) -> Vec<Point> {
    let mut transformed_points = points.to_vec();
    
    // Apply rotation
    if rotation != 0.0 {
        transformed_points = transformed_points
            .iter()
            .map(|&point| rotate_point(point, rotation, center))
            .collect();
    }
    
    // Apply horizontal flip
    if flip_x {
        transformed_points = transformed_points
            .iter()
            .map(|&point| flip_point_x(point, center))
            .collect();
    }
    
    // Apply vertical flip
    if flip_y {
        transformed_points = transformed_points
            .iter()
            .map(|&point| flip_point_y(point, center))
            .collect();
    }
    
    transformed_points
}

/// Calculate the center point of a set of points
pub fn calculate_center(points: &[Point]) -> Point {
    if points.is_empty() {
        return Point::origin();
    }
    
    let sum_x: f64 = points.iter().map(|p| p.x).sum();
    let sum_y: f64 = points.iter().map(|p| p.y).sum();
    let count = points.len() as f64;
    
    Point::new(sum_x / count, sum_y / count)
}

/// Calculate the bounding box of a set of points
pub fn calculate_bounds(points: &[Point]) -> Option<(Point, Point)> {
    if points.is_empty() {
        return None;
    }
    
    let mut min_x = points[0].x;
    let mut min_y = points[0].y;
    let mut max_x = points[0].x;
    let mut max_y = points[0].y;
    
    for point in points.iter().skip(1) {
        min_x = min_x.min(point.x);
        min_y = min_y.min(point.y);
        max_x = max_x.max(point.x);
        max_y = max_y.max(point.y);
    }
    
    Some((Point::new(min_x, min_y), Point::new(max_x, max_y)))
}

/// Scale points around a center point
pub fn scale_points(points: &[Point], scale: f64, center: Option<Point>) -> Point {
    if points.is_empty() {
        return Point::origin();
    }
    
    let center = center.unwrap_or_else(|| calculate_center(points));
    let sum_x: f64 = points
        .iter()
        .map(|p| {
            let translated = p.subtract(&center);
            let scaled = translated.scale(scale);
            scaled.add(&center).x
        })
        .sum();
    
    let sum_y: f64 = points
        .iter()
        .map(|p| {
            let translated = p.subtract(&center);
            let scaled = translated.scale(scale);
            scaled.add(&center).y
        })
        .sum();
    
    let count = points.len() as f64;
    Point::new(sum_x / count, sum_y / count)
}

/// Translate points by a delta
pub fn translate_points(points: &[Point], dx: f64, dy: f64) -> Point {
    if points.is_empty() {
        return Point::origin();
    }
    
    let sum_x: f64 = points
        .iter()
        .map(|p| p.translate(dx, dy).x)
        .sum();
    
    let sum_y: f64 = points
        .iter()
        .map(|p| p.translate(dx, dy).y)
        .sum();
    
    let count = points.len() as f64;
    Point::new(sum_x / count, sum_y / count)
}

/// Normalize rotation angle to 0-360 degrees
pub fn normalize_rotation(angle: f64) -> f64 {
    let mut normalized = angle % 360.0;
    if normalized < 0.0 {
        normalized += 360.0;
    }
    normalized
}

/// Check if a rotation angle is valid (0, 90, 180, 270)
pub fn is_valid_rotation(angle: i32) -> bool {
    matches!(angle, 0 | 90 | 180 | 270)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_point() {
        let point = Point::new(1.0, 0.0);
        let center = Point::origin();
        let rotated = rotate_point(point, 90.0, center);
        
        assert!((rotated.x - 0.0).abs() < 0.001);
        assert!((rotated.y - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_rotate_point_around_center() {
        let point = Point::new(2.0, 0.0);
        let center = Point::new(1.0, 0.0);
        let rotated = rotate_point(point, 180.0, center);
        
        assert!((rotated.x - 0.0).abs() < 0.001);
        assert!((rotated.y - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_flip_point_x() {
        let point = Point::new(3.0, 2.0);
        let center = Point::new(1.0, 0.0);
        let flipped = flip_point_x(point, center);
        
        assert!((flipped.x - (-1.0)).abs() < 0.001);
        assert!((flipped.y - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_flip_point_y() {
        let point = Point::new(2.0, 3.0);
        let center = Point::new(0.0, 1.0);
        let flipped = flip_point_y(point, center);
        
        assert!((flipped.x - 2.0).abs() < 0.001);
        assert!((flipped.y - (-1.0)).abs() < 0.001);
    }

    #[test]
    fn test_transform_points() {
        let points = vec![
            Point::new(1.0, 0.0),
            Point::new(0.0, 1.0),
            Point::new(-1.0, 0.0),
        ];
        
        let transformed = transform_points(&points, 90, false, false);
        
        // Should be rotated 90 degrees around center
        assert!(!transformed.is_empty());
        assert_eq!(transformed.len(), points.len());
    }

    #[test]
    fn test_calculate_center() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(2.0, 0.0),
            Point::new(1.0, 2.0),
        ];
        
        let center = calculate_center(&points);
        assert!((center.x - 1.0).abs() < 0.001);
        assert!((center.y - 0.6666666667).abs() < 0.001);
    }

    #[test]
    fn test_calculate_bounds() {
        let points = vec![
            Point::new(-1.0, -2.0),
            Point::new(3.0, 4.0),
            Point::new(0.0, 1.0),
        ];
        
        let (min, max) = calculate_bounds(&points).unwrap();
        assert!((min.x - (-1.0)).abs() < 0.001);
        assert!((min.y - (-2.0)).abs() < 0.001);
        assert!((max.x - 3.0).abs() < 0.001);
        assert!((max.y - 4.0).abs() < 0.001);
    }

    #[test]
    fn test_normalize_rotation() {
        assert_eq!(normalize_rotation(450.0), 90.0);
        assert_eq!(normalize_rotation(-90.0), 270.0);
        assert_eq!(normalize_rotation(720.0), 0.0);
        assert_eq!(normalize_rotation(180.0), 180.0);
    }

    #[test]
    fn test_is_valid_rotation() {
        assert!(is_valid_rotation(0));
        assert!(is_valid_rotation(90));
        assert!(is_valid_rotation(180));
        assert!(is_valid_rotation(270));
        assert!(!is_valid_rotation(45));
        assert!(!is_valid_rotation(-90));
    }

    #[test]
    fn test_empty_points() {
        let empty: Vec<Point> = vec![];
        assert_eq!(calculate_center(&empty), Point::origin());
        assert!(calculate_bounds(&empty).is_none());
    }
}
