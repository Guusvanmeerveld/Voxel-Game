pub struct Orientation {
    yaw: isize,
    pitch: isize,
}

impl Orientation {
    const LIMIT: isize = 360;

    pub fn new(yaw: isize, pitch: isize) -> Self {
        Self {
            yaw: yaw % Self::LIMIT,
            pitch: pitch % Self::LIMIT,
        }
    }

    pub fn yaw(&self) -> isize {
        self.yaw
    }

    pub fn pitch(&self) -> isize {
        self.pitch
    }
}
