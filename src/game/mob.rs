use crate::rltk;
use rltk::Color;
use rltk::Point;

pub struct Mob {
    pub position : Point,
    pub glyph: u8,
    pub fg : Color,
}

impl Mob {
    pub fn new(x:i32, y:i32, glyph:u8, fg : Color) -> Mob {
        Mob{ position: Point::new(x, y), glyph, fg }
    }
}