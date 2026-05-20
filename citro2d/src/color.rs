#[derive(Debug, Copy, Clone)]
pub struct Color(pub u32);

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(unsafe { citro2d_sys::C2D_Color32(r, g, b, a) })
    }
}

impl From<Color> for u32 {
    fn from(c: Color) -> u32 {
        c.0
    }
}
