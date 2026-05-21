/// A 32-bit RGBA color value for use with citro2d.
///
/// Colors are stored internally in memory by the format `AABBGGRR` as expected by PICA200 GPU.
///
/// # Examples
///
/// ```
/// let red = Color::rgb(255, 0, 0);
/// let white = Color::rgb(255, 0, 0);
/// let mist = Color::rgba(200, 200, 255, 64);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub(crate) value: u32,
}

impl Color {
    /// Creates a fully opaque color from red, green and blue components.
    ///
    /// Each component is in the range `0..=255`.
    ///
    /// # Examples
    ///
    /// ```
    /// let red = Color::rgb(255, 0, 0);
    /// let gray = Color::rgb(80, 80, 80);
    /// let yellow = Color::rgb(255, 255, 0);
    /// ```
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    /// Creates a fully opaque color from red, green, blue and alpha components.
    ///
    /// Each component is in the range `0..=255`.
    /// An alpha of `255` is fully opaque, `0` is fully transparent.
    ///
    /// # Examples
    ///
    /// ```
    /// let mist = Color::rgba(200, 200, 255, 64);
    /// let invisible = Color::rgba(255, 255, 255, 0);
    /// ```
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            value: (r as u32) | ((g as u32) << 8) | ((b as u32) << 16) | ((a as u32) << 24),
        }
    }
}
