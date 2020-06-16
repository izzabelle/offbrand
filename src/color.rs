/// an 8 bit rgb with built in conversion to u32
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    /// return a color from a given u32
    pub fn from_u32(color: u32) -> Color {
        let b = (color & 0xff) as u8;
        let g = ((color >> 8) & 0xff) as u8;
        let r = ((color >> 16) & 0xff) as u8;
        Color { r, g, b }
    }

    /// return color as a u32 structured 0bxxxx_rrrr_gggg_bbbb
    pub fn as_u32(&self) -> u32 {
        let (r, g, b) = (self.r as u32, self.g as u32, self.b as u32);
        (r << 16) | (g << 8) | b
    }

    /// white color
    pub const WHITE: Color = Color { r: 0xff, g: 0xff, b: 0xff };
    /// black color
    pub const BLACK: Color = Color { r: 0x00, g: 0x00, b: 0x00 };
}
