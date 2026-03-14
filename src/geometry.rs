/// 2D point with f32 precision
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn distance_to(self, other: Point) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn scale(self, factor: f32) -> Point {
        Point {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

/// 2D axis-aligned bounding box
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl Bounds {
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    pub fn from_min_size(min: Point, size: Point) -> Self {
        Self {
            min_x: min.x,
            min_y: min.y,
            max_x: min.x + size.x,
            max_y: min.y + size.y,
        }
    }

    pub fn width(&self) -> f32 {
        self.max_x - self.min_x
    }

    pub fn height(&self) -> f32 {
        self.max_y - self.min_y
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.min_x
            && point.x <= self.max_x
            && point.y >= self.min_y
            && point.y <= self.max_y
    }

    pub fn center(&self) -> Point {
        Point {
            x: (self.min_x + self.max_x) / 2.0,
            y: (self.min_y + self.max_y) / 2.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_add() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 4.0);
        let result = p1.add(p2);
        assert_eq!(result, Point::new(4.0, 6.0));
    }

    #[test]
    fn test_point_distance() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        assert!((p1.distance_to(p2) - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_bounds_contains() {
        let b = Bounds::new(0.0, 0.0, 10.0, 10.0);
        assert!(b.contains(Point::new(5.0, 5.0)));
        assert!(!b.contains(Point::new(11.0, 5.0)));
    }

    #[test]
    fn test_bounds_center() {
        let b = Bounds::new(0.0, 0.0, 10.0, 10.0);
        assert_eq!(b.center(), Point::new(5.0, 5.0));
    }
}
