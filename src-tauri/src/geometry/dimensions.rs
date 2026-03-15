// Dimension generation will be implemented in Phase 5
use crate::models::{Point, Dimension};
use super::transform::calculate_center;

/// Generate dimensions for a set of points
/// Returns width, height, and diagonal dimensions
pub fn generate_dimensions(points: &[Point]) -> Vec<Dimension> {
    if points.len() < 2 {
        return Vec::new();
    }

    let mut dimensions = Vec::new();
    let bounds = super::transform::calculate_bounds(points);
    
    if let Some((min_point, max_point)) = bounds {
        // Width dimension (horizontal)
        let width_start = Point::new(min_point.x, min_point.y - 20.0);
        let width_end = Point::new(max_point.x, min_point.y - 20.0);
        let width = width_end.distance_to(&width_start);
        dimensions.push(Dimension::new(
            width_start,
            width_end,
            20.0,
            format!("{:.1}", width)
        ));

        // Height dimension (vertical)
        let height_start = Point::new(max_point.x + 20.0, min_point.y);
        let height_end = Point::new(max_point.x + 20.0, max_point.y);
        let height = height_end.distance_to(&height_start);
        dimensions.push(Dimension::new(
            height_start,
            height_end,
            20.0,
            format!("{:.1}", height)
        ));

        // Diagonal dimension (from bottom-left to top-right)
        let diagonal_start = Point::new(min_point.x, min_point.y);
        let diagonal_end = Point::new(max_point.x, max_point.y);
        let diagonal_length = diagonal_end.distance_to(&diagonal_start);
        dimensions.push(Dimension::new(
            diagonal_start,
            diagonal_end,
            30.0,
            format!("{:.1}", diagonal_length)
        ));
    }

    // Generate dimensions for individual segments if it's a polygon
    if points.len() >= 3 {
        for i in 0..points.len() - 1 {
            let start = points[i];
            let end = points[i + 1];
            let length = start.distance_to(&end);
            
            // Skip if this is the closing point (same as first point)
            if i == points.len() - 1 && (start.x - points[0].x).abs() < 0.001 && (start.y - points[0].y).abs() < 0.001 {
                continue;
            }
            
            // Calculate offset for dimension line (perpendicular to segment)
            let mid_point = start.midpoint(&end);
            let offset = calculate_segment_offset(start, end, 15.0);
            
            dimensions.push(Dimension::new(
                start,
                end,
                offset,
                format!("{:.1}", length)
            ));
        }
    }

    dimensions
}

/// Calculate offset distance for dimension line perpendicular to segment
fn calculate_segment_offset(start: Point, end: Point, base_offset: f64) -> f64 {
    let segment_length = start.distance_to(&end);
    // Scale offset based on segment length for better visibility
    base_offset * (segment_length / 50.0).max(0.5).min(2.0)
}

/// Generate radial dimensions for circular shapes
pub fn generate_radial_dimensions(center: Point, radius: f64, num_radial: usize) -> Vec<Dimension> {
    let mut dimensions = Vec::new();
    
    // Radius dimension
    dimensions.push(Dimension::new(
        center,
        Point::new(center.x + radius, center.y),
        10.0,
        format!("R{:.1}", radius)
    ));
    
    // Diameter dimension
    dimensions.push(Dimension::new(
        Point::new(center.x - radius, center.y),
        Point::new(center.x + radius, center.y),
        20.0,
        format!("⌀{:.1}", radius * 2.0)
    ));
    
    // Radial dimensions at different angles
    if num_radial > 0 {
        use std::f64::consts::PI;
        for i in 0..num_radial {
            let angle = 2.0 * PI * (i as f64) / (num_radial as f64);
            let end_point = Point::new(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin()
            );
            
            dimensions.push(Dimension::new(
                center,
                end_point,
                15.0,
                format!("{:.1}°", angle * 180.0 / PI)
            ));
        }
    }
    
    dimensions
}

/// Generate angular dimensions between three points
pub fn generate_angular_dimensions(vertex: Point, start: Point, end: Point) -> Vec<Dimension> {
    let mut dimensions = Vec::new();
    
    // Calculate angle
    let start_vector = start.subtract(&vertex);
    let end_vector = end.subtract(&vertex);
    
    let start_angle = start_vector.y.atan2(start_vector.x);
    let end_angle = end_vector.y.atan2(end_vector.x);
    let mut angle = end_angle - start_angle;
    
    // Normalize angle to 0-360
    if angle < 0.0 {
        angle += 2.0 * std::f64::consts::PI;
    }
    
    // Create arc points for dimension
    let arc_radius = 30.0;
    let arc_start = Point::new(
        vertex.x + arc_radius * start_angle.cos(),
        vertex.y + arc_radius * start_angle.sin()
    );
    let arc_end = Point::new(
        vertex.x + arc_radius * end_angle.cos(),
        vertex.y + arc_radius * end_angle.sin()
    );
    
    dimensions.push(Dimension::new(
        arc_start,
        arc_end,
        arc_radius,
        format!("{:.1}°", angle * 180.0 / std::f64::consts::PI)
    ));
    
    dimensions
}

/// Generate center marks for shapes
pub fn generate_center_marks(points: &[Point]) -> Vec<Dimension> {
    let mut dimensions = Vec::new();
    
    if let Some((min_point, max_point)) = super::transform::calculate_bounds(points) {
        let center = Point::new(
            (min_point.x + max_point.x) / 2.0,
            (min_point.y + max_point.y) / 2.0
        );
        
        // Cross center mark
        let mark_size = 10.0;
        dimensions.push(Dimension::new(
            Point::new(center.x - mark_size, center.y),
            Point::new(center.x + mark_size, center.y),
            0.0,
            "".to_string()
        ));
        
        dimensions.push(Dimension::new(
            Point::new(center.x, center.y - mark_size),
            Point::new(center.x, center.y + mark_size),
            0.0,
            "".to_string()
        ));
    }
    
    dimensions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_dimensions_rectangle() {
        let points = vec![
            Point::new(-50.0, -25.0),
            Point::new(50.0, -25.0),
            Point::new(50.0, 25.0),
            Point::new(-50.0, 25.0),
        ];
        
        let dimensions = generate_dimensions(&points);
        
        // Should have width, height, and diagonal dimensions
        assert!(dimensions.len() >= 3);
        
        // Check width dimension
        let width_dim = &dimensions[0];
        assert_eq!(width_dim.label, "100.0");
        
        // Check height dimension
        let height_dim = &dimensions[1];
        assert_eq!(height_dim.label, "50.0");
    }

    #[test]
    fn test_generate_radial_dimensions() {
        let center = Point::origin();
        let dimensions = generate_radial_dimensions(center, 50.0, 4);
        
        assert!(dimensions.len() >= 2); // At least radius and diameter
        
        // Check radius dimension
        let radius_dim = &dimensions[0];
        assert_eq!(radius_dim.label, "R50.0");
        
        // Check diameter dimension
        let diameter_dim = &dimensions[1];
        assert_eq!(diameter_dim.label, "⌀100.0");
    }

    #[test]
    fn test_generate_angular_dimensions() {
        let vertex = Point::origin();
        let start = Point::new(1.0, 0.0);
        let end = Point::new(0.0, 1.0);
        
        let dimensions = generate_angular_dimensions(vertex, start, end);
        
        assert_eq!(dimensions.len(), 1);
        assert!(dimensions[0].label.contains("90.0"));
    }

    #[test]
    fn test_generate_center_marks() {
        let points = vec![
            Point::new(-50.0, -25.0),
            Point::new(50.0, -25.0),
            Point::new(50.0, 25.0),
            Point::new(-50.0, 25.0),
        ];
        
        let dimensions = generate_center_marks(&points);
        
        assert_eq!(dimensions.len(), 2); // Horizontal and vertical marks
    }
}
