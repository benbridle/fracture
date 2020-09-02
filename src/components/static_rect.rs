use super::Point;

pub struct StaticRect {
    pub origin: Point,
    pub width: usize,
    pub height: usize,
}

impl StaticRect {
    pub fn new(origin: Point, width: usize, height: usize) -> StaticRect {
        StaticRect {
            origin,
            width,
            height,
        }
    }
}
