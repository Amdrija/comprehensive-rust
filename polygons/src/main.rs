use std::{f64::consts::PI, ops::Add, slice::Iter};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn magnitude(&self) -> f64 {
        self.dist(&Point::new(0, 0))
    }

    pub fn dist(&self, p: &Point) -> f64 {
        let dx = self.x - p.x;
        let dy = self.y - p.y;
        f64::sqrt(((dx * dx) + (dy * dy)) as f64)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    pub fn new() -> Polygon {
        Polygon { points: Vec::new() }
    }

    pub fn add_point(&mut self, p: Point) {
        self.points.push(p);
    }

    pub fn left_most_point(&self) -> Option<Point> {
        self.points.iter().min_by_key(|p| p.x).copied()
    }

    pub fn iter(&self) -> Iter<Point> {
        self.points.iter()
    }

    pub fn perimeter(&self) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }

        let mut last = self.points[self.points.len() - 1];
        let mut perimeter = 0.0;
        for i in 0..self.points.len() {
            perimeter += self.points[i].dist(&last);
            last = self.points[i];
        }

        perimeter
    }
}

pub struct Circle {
    center: Point,
    radius: f64,
}

impl Circle {
    pub fn new(center: Point, radius: f64) -> Circle {
        Circle { center, radius }
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }

    pub fn get_center(&self) -> &Point {
        &self.center
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl Shape {
    pub fn perimeter(&self) -> f64 {
        match self {
            Shape::Polygon(p) => p.perimeter(),
            Shape::Circle(c) => c.perimeter(),
        }
    }

    pub fn from(c: Circle) -> Shape {
        Shape::Circle(c)
    }
}

impl From<Polygon> for Shape {
    fn from(value: Polygon) -> Self {
        Shape::Polygon(value)
    }
}

impl From<Circle> for Shape {
    fn from(value: Circle) -> Self {
        Shape::Circle(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(&p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            <Shape as From<Polygon>>::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5.0)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {}
