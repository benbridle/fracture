pub struct Area {
    pub width: usize,
    pub height: usize,
}

impl Area {
    pub fn new(width: usize, height: usize) -> Area {
        Area { width, height }
    }
}
