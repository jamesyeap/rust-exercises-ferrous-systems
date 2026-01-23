use std::f32::consts::PI;

pub struct Circle<T>
where
    T: num::Num + Copy,
{
    radius: T,
}

pub struct Square<T>
where
    T: num::Num + Copy,
{
    side: T,
}

impl<T: num::Num + From<f32> + Copy> Circle<T> {
    fn new(radius: T) -> Self {
        Self { radius }
    }

    fn area(&self) -> T {
        let my_pi: T = PI.into();
        my_pi * self.radius * self.radius
    }

    fn scale(&mut self, factor: T) {
        self.radius = self.radius * factor
    }

    fn destroy(self) -> T {
        self.radius
    }
}

impl<T: num::Num + Copy> Square<T> {
    fn new(side: T) -> Self {
        Self { side }
    }

    fn area(&self) -> T {
        self.side * self.side
    }

    fn scale(&mut self, factor: T) {
        self.side = self.side * factor
    }

    fn destroy(self) -> T {
        self.side
    }
}

pub trait HasArea<T> {
    fn area(&self) -> T;
}

impl<T> HasArea<T> for Circle<T>
where
    T: num::Num + Copy + From<f32>,
{
    fn area(&self) -> T {
        self.area()
    }
}

impl<T: num::Num + Copy> HasArea<T> for Square<T> {
    fn area(&self) -> T {
        self.area()
    }
}

enum Shape<T>
where
    T: num::Num + Copy,
{
    Circle(Circle<T>),
    Square(Square<T>),
}

impl<T> HasArea<T> for Shape<T>
where
    T: num::Num + Copy + From<f32>,
{
    fn area(&self) -> T {
        match self {
            Self::Circle(circle) => HasArea::area(circle),
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
        let test_square = Square::new(5f64);
        assert_eq!(test_square.area(), 25f64);
        let shape = Shape::Square(test_square);
        assert_eq!(shape.area(), 25.0);
    }

    #[test]
    fn circle() {
        let test_circle = Circle::new(4.0f32);
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
        let mut test_circle = Circle::new(4.0f32);
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
