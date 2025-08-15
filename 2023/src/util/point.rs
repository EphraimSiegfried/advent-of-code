use std::ops::Add;
pub const ORIGIN: Point = Point(0, 0);
pub const UP: Point = Point(-1, 0);
pub const DOWN: Point = Point(1, 0);
pub const LEFT: Point = Point(0, -1);
pub const RIGHT: Point = Point(0, 1);
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
#[derive(Copy, Clone, Debug, Hash, Eq)]
pub struct Point(pub i32, pub i32);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}
