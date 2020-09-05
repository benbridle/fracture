use super::{Area, Point, StaticRect};
use std::cmp;

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
        let parent_width = parent_area.width as isize;
        let parent_height = parent_area.height as isize;
        // Do start-end conversions
        let static_left = if self.left < 0 {
            parent_width + self.left
        } else {
            self.left
        };
        let static_right = if self.right < 0 {
            parent_width + self.right
        } else {
            self.right
        };
        let static_top = if self.top < 0 {
            parent_height + self.top
        } else {
            self.top
        };
        let static_bottom = if self.bottom < 0 {
            parent_height + self.bottom
        } else {
            self.bottom
        };

        let width = cmp::max(static_right - static_left, 0) as usize;
        let height = cmp::max(static_bottom - static_top, 0) as usize;

        StaticRect::new(
            Point::new(static_left as usize, static_top as usize),
            width,
            height,
        )
    }
}
