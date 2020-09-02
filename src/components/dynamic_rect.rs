use super::{Area, Point, StaticRect};

pub struct DynamicRect {
    left: isize,
    top: isize,
    right: isize,
    bottom: isize,
}

impl DynamicRect {
    // The distance that each edge comes in from the parent rectangle edges.
    // Positive values come in from the left/top edges, negative values come in
    //  from the right/bottom edges.

    pub fn new(left: isize, top: isize, right: isize, bottom: isize) -> DynamicRect {
        DynamicRect {
            left,
            top,
            right,
            bottom,
        }
    }
    pub fn to_static_rect(&self, parent_area: Area) -> StaticRect {
        let static_left = if self.left < 0 {
            parent_area.width + self.left as usize
        } else {
            self.left as usize
        };
        let static_right = if self.right < 0 {
            parent_area.width + self.right as usize
        } else {
            self.right as usize
        };
        let static_top = if self.top < 0 {
            parent_area.height + self.top as usize
        } else {
            self.top as usize
        };
        let static_bottom = if self.bottom < 0 {
            parent_area.height + self.bottom as usize
        } else {
            self.bottom as usize
        };

        StaticRect::new(
            Point::new(static_left, static_top),
            static_right - static_left,
            static_bottom - static_top,
        )
    }
}
