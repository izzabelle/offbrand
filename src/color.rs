// a 32 bit rgb color
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// create a new color
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    /// return the color as a u32
    pub fn as_u32(&self) -> u32 {
        let (r, g, b) = (self.r as u32, self.g as u32, self.b as u32);
        (r << 16) | (g << 8) | b
    }
}

pub const BLACK: Color = Color { r: 0x00, g: 0x00, b: 0x00 };
pub const WHITE: Color = Color { r: 0xff, g: 0xff, b: 0xff };
