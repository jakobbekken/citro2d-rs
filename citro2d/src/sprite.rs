use crate::Error;
use citro2d_sys::*;

pub struct SpriteSheet {
    ptr: C2D_SpriteSheet,
}

pub struct Sprite<'sheet> {
    inner: C2D_Sprite,
    _sheet: core::marker::PhantomData<&'sheet SpriteSheet>,
}

impl SpriteSheet {
    pub fn from_mem(data: &[u8]) -> Result<Self, Error> {
        let ptr = unsafe { C2D_SpriteSheetLoadFromMem(data.as_ptr() as *const _, data.len()) };
        if ptr.is_null() {
            return Err(Error::SpriteSheetLoadFailed);
        }
        Ok(Self { ptr })
    }

    pub fn load(path: &str) -> Result<Self, Error> {
        let mut buf = [0u8; 256];
        let bytes = path.as_bytes();
        if bytes.len() >= buf.len() {
            return Err(Error::SpriteSheetLoadFailed);
        }
        buf[..bytes.len()].copy_from_slice(bytes);

        let ptr = unsafe { C2D_SpriteSheetLoad(buf.as_ptr() as *const _) };
        if ptr.is_null() {
            return Err(Error::SpriteSheetLoadFailed);
        }
        Ok(Self { ptr })
    }

    pub fn len(&self) -> usize {
        unsafe { C2D_SpriteSheetCount(self.ptr) }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn sprite(&self, index: usize) -> Result<Sprite<'_>, Error> {
        if index >= self.len() {
            return Err(Error::SpriteIndexOutOfBounds);
        }
        let mut inner = core::mem::MaybeUninit::<C2D_Sprite>::uninit();
        unsafe {
            C2D_SpriteFromSheet(inner.as_mut_ptr(), self.ptr, index);
            Ok(Sprite {
                inner: inner.assume_init(),
                _sheet: core::marker::PhantomData,
            })
        }
    }
}

impl Drop for SpriteSheet {
    fn drop(&mut self) {
        unsafe {
            C2D_SpriteSheetFree(self.ptr);
        }
    }
}

impl<'sheet> Sprite<'sheet> {
    pub fn set_pos(&mut self, x: f32, y: f32) {
        unsafe {
            C2D_SpriteSetPos(&mut self.inner, x, y);
        }
    }

    pub fn set_scale(&mut self, x: f32, y: f32) {
        unsafe {
            C2D_SpriteSetScale(&mut self.inner, x, y);
        }
    }

    pub fn set_rotation(&mut self, radians: f32) {
        unsafe {
            C2D_SpriteSetRotation(&mut self.inner, radians);
        }
    }

    pub fn set_rotation_degrees(&mut self, degrees: f32) {
        unsafe {
            C2D_SpriteSetRotationDegrees(&mut self.inner, degrees);
        }
    }

    pub fn set_center(&mut self, x: f32, y: f32) {
        unsafe {
            C2D_SpriteSetCenter(&mut self.inner, x, y);
        }
    }

    pub fn set_depth(&mut self, depth: f32) {
        unsafe {
            C2D_SpriteSetDepth(&mut self.inner, depth);
        }
    }

    pub fn draw(&self) {
        unsafe {
            C2D_DrawSprite(&self.inner);
        }
    }
}
