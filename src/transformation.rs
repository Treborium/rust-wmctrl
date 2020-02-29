use std::fmt;

/// A type holding the transformation properties of a `wmctrl::Window`
#[derive(Debug)]
pub struct Transformation {
    pub gravity: u16,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

impl Transformation {
    pub fn new(x: i16, y: i16, width: u16, height: u16) -> Transformation {
        Transformation {
            gravity: 0,
            x,
            y,
            width,
            height,
        }
    }
}

impl fmt::Display for Transformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{},{},{},{}",
            self.gravity, self.x, self.y, self.width, self.height
        )
    }
}
