type Name = char;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: u32,
    y: u32,
    name: Name,
}

impl Point {
    pub fn new(x: u32, y: u32, name: Name) -> Self {
        Point { x, y, name }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    p1: Point,
    p2: Point,
}
impl Line {
    pub fn new(p1: Point, p2: Point) -> Self {
        Line { p1, p2 }
    }
}

#[derive(Debug, Clone)]
pub struct Polygon {
    polygon: Vec<Point>,
}
