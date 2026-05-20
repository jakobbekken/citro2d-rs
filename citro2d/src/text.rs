use crate::Color;
use crate::Error;
use citro2d_sys::*;

pub struct TextBuf {
    ptr: C2D_TextBuf,
}

pub struct Text {
    inner: C2D_Text,
}

impl TextBuf {
    pub fn new(max_glyphs: usize) -> Result<Self, Error> {
        let ptr = unsafe { C2D_TextBufNew(max_glyphs) };
        if ptr.is_null() {
            return Err(Error::TextBufAllocFailed);
        }
        Ok(Self { ptr })
    }

    pub fn clear(&self) {
        unsafe {
            C2D_TextBufClear(self.ptr);
        }
    }

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
                u32::from(color),
            );
        }
    }
}
