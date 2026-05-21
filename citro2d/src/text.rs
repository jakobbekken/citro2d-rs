use crate::Color;
use crate::Error;
use citro2d_sys::*;

/// A buffer to hold parsed glyph data for text rendering.
///
/// `TextBuf` is a fixed-size scratch buffer that stores glyphs parsed from strings.
/// It must be cleared each frame before parsing new strings.
///
/// # Examples
///
/// ```
/// let buf = TextBuf::new(256)?;
///
/// // each frame:
/// buf.clear();
/// if let Some(text) = buf.parse("Hey, hey!") {
///     scene.draw_text(&text, 10.0, 10.0, 1.0, Color::rgb(255, 255, 255));
/// }
/// ```
pub struct TextBuf {
    ptr: C2D_TextBuf,
}

/// A parsed text object ready to be drawn.
///
/// Created by the [`TextBuf::parse`]. Borrows from the `TextBuf` it was parsed from, so do not clear the buffer while a `Text` is still in use.
pub struct Text {
    inner: C2D_Text,
}

impl TextBuf {
    /// Creates a new text buffer with capacity for `max_glyphs` glyphs.
    ///
    /// A glyph is a single character. For most cases, `256` is a reasonable amount. If you need to render many strings per frame, increase this value.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TextBufAllocFailed`] if the allocation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let buf = TextBuf::new(256)?;
    /// ```
    pub fn new(max_glyphs: usize) -> Result<Self, Error> {
        let ptr = unsafe { C2D_TextBufNew(max_glyphs) };
        if ptr.is_null() {
            return Err(Error::TextBufAllocFailed);
        }
        Ok(Self { ptr })
    }

    /// Clears all parsed glyphs from the buffer.
    ///
    /// Call this at the start of each frame before parsing new strings.
    /// It does not free any memory, the buffer can be used immediately.
    pub fn clear(&self) {
        unsafe {
            C2D_TextBufClear(self.ptr);
        }
    }

    /// Parses a string into the buffer and returns some [`Text`].
    ///
    /// Returns `None` if the string is too long (over 255 bytes) or the buffer is full.
    ///
    /// # Examples
    ///
    /// ```
    /// buf.clear();
    /// if let Some(text) = buf.parse("Hey, hey!") {
    ///     scene.draw_text(&text, 10.0, 10.0, 1.0, Color::rgb(255, 255, 255));
    /// }
    /// ```
    pub fn parse(&self, s: &str) -> Option<Text> {
        let mut buf = [0u8; 256]; // null-terminated string
        let bytes = s.as_bytes();
        if bytes.len() >= buf.len() {
            return None;
        }
        buf[..bytes.len()].copy_from_slice(bytes);

        let mut text = core::mem::MaybeUninit::<C2D_Text>::uninit();
        unsafe {
            C2D_TextParse(text.as_mut_ptr(), self.ptr, buf.as_ptr() as *const _);
            C2D_TextOptimize(text.as_ptr());
            Some(Text {
                inner: text.assume_init(),
            })
        }
    }
}

impl Drop for TextBuf {
    fn drop(&mut self) {
        unsafe {
            C2D_TextBufDelete(self.ptr);
        }
    }
}

impl Text {
    /// Draws the text at the given position with a scale and color.
    ///
    /// `scale` of `1.0` corresponds to the native size of the system font, which has a glyph height of 30px with a baseline at 25px.
    ///
    /// # Arguments
    ///
    /// * `x` - Horizontal position in pixels
    /// * `y` - Vertical position in pixels
    /// * `z` - Depth value, use `0.0` if unsure
    /// * `scale` - Font size multiplier
    /// * `color` - Text color
    pub fn draw(&self, x: f32, y: f32, z: f32, scale: f32, color: Color) {
        unsafe {
            C2D_DrawText(
                &self.inner,
                C2D_WithColor as u32,
                x,
                y,
                z,
                scale,
                scale,
                color.value,
            );
        }
    }
}
