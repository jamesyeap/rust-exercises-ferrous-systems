use std::f32::consts::PI;

pub struct Circle {
    radius: f32,
}

pub struct Square {
    side: i32,
}

impl Circle {
    fn new(radius: f32) -> Self {
        Self { radius }
    }

    fn area(&self) -> f32 {
        PI * self.radius * self.radius
    }

    fn scale(&mut self, factor: f32) {
        self.radius *= factor
    }

    fn destroy(self) -> f32 {
        self.radius
    }
}

impl Square {
    fn new(side: i32) -> Self {
        Self { side }
    }

    fn area(&self) -> i32 {
        self.side * self.side
    }

    fn scale(&mut self, factor: i32) {
        self.side *= factor
    }

    fn destroy(self) -> i32 {
        self.side
    }
}

pub trait HasArea {
    fn area(&self) -> f32;
}

impl HasArea for Circle {
    fn area(&self) -> f32 {
        self.area()
    }
}

impl HasArea for Square {
    fn area(&self) -> f32 {
        self.area() as f32
    }
}

enum Shape {
    Circle(Circle),
    Square(Square),
}

impl HasArea for Shape {
    fn area(&self) -> f32 {
        match self {
            Self::Circle(circle) => circle.area(),
            Self::Square(square) => HasArea::area(square),
        }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squares() {
        let test_square = Square::new(5);
        assert_eq!(test_square.area(), 25);
        let shape = Shape::Square(test_square);
        assert_eq!(shape.area(), 25.0);
    }

    #[test]
    fn circle() {
        let test_circle = Circle::new(4.0);
        assert!((test_circle.area() - 50.265484).abs() < 0.001);
        let shape = Shape::Circle(test_circle);
        assert!((shape.area() - 50.265484).abs() < 0.001);
    }

    #[test]
    fn square_scale() {
        let mut test_square = Square::new(4);
        test_square.scale(2);
        assert_eq!(test_square.area(), 64);
    }

    #[test]
    fn circle_scale() {
        let mut test_circle = Circle::new(4.0);
        test_circle.scale(2.0);
        assert!((test_circle.area() - 201.06194).abs() < 0.001);
    }

    #[test]
    fn square_destroy() {
        let test_square = Square::new(4);
        assert_eq!(test_square.destroy(), 4);
    }

    #[test]
    fn circle_destroy() {
        let test_circle = Circle::new(4.0);
        assert_eq!(test_circle.destroy(), 4.0);
    }
}