#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub(crate) value: u32,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            value: (r as u32) | ((g as u32) << 8) | ((b as u32) << 16) | ((a as u32) << 24),
        }
    }
}
