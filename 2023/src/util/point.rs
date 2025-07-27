use std::ops::Add;
pub const ORIGIN: Point = Point(0, 0);
pub const UP: Point = Point(0, -1);
pub const DOWN: Point = Point(0, 1);
pub const LEFT: Point = Point(-1, 0);
pub const RIGHT: Point = Point(1, 0);
pub const ORTHOGONAL: [Point; 4] = [UP, DOWN, LEFT, RIGHT];
pub const DIAGONAL: [Point; 8] = [
    Point(-1, -1),
    UP,
    Point(1, -1),
    LEFT,
    RIGHT,
    Point(-1, 1),
    DOWN,
    Point(1, 1),
];
pub struct Point(pub i32, pub i32);

impl Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Point(self.0 + rhs.1, self.0 + rhs.1)
    }
}
