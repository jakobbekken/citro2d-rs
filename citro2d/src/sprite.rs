use crate::Error;
use citro2d_sys::*;

/// A loaded sprite sheet containing one or more images.
///
/// Images are stored in teh `.t3x` format, converted from PNG using the `tex3ds` tool from devkitPro like this:
/// `tex3ds -f rgba8888 -z auto citro2d-test/assets/test.png -o citro2d-test/assets/test.t3x`
///
/// # Examples
///
/// ```
/// // From embedded bytes
/// let sheet = SpriteSheet::from_mem(include_bytes!("sprites.t3x"))?;
///
/// // From SD card
/// let sheet = SpriteSheet::load("/sprites.t3x")?;
/// ```
pub struct SpriteSheet {
    ptr: C2D_SpriteSheet,
}

/// A sprite instance created from [`SpriteSheet`].
///
/// Cannot outlive the sheet it was created from as it borrows from it.
pub struct Sprite<'sheet> {
    inner: C2D_Sprite,
    _sheet: core::marker::PhantomData<&'sheet SpriteSheet>,
}

impl SpriteSheet {
    /// Loads a prite sheet from memory.
    ///
    /// Use with `include_bytes!` to embed assets directly in the binary.
    ///
    /// # Errors
    ///
    /// Returns [`Error::SpriteSheetLoadFailed`] if loading fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let sheet = SpriteSheet::from_mem(include_bytes!("sprite.t3x"))?;
    /// ```
    pub fn from_mem(data: &[u8]) -> Result<Self, Error> {
        let ptr = unsafe { C2D_SpriteSheetLoadFromMem(data.as_ptr() as *const _, data.len()) };
        if ptr.is_null() {
            return Err(Error::SpriteSheetLoadFailed);
        }
        Ok(Self { ptr })
    }

    /// Loads a sprite sheet from a file path on the SD card.
    ///
    /// # Errors
    ///
    /// Returns [`Error::SpriteSheetLoadFailed`] if the file is not found or loading fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let sheet = SpriteSheet::load("/sprites.t3x")?;
    /// ```
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

    /// Returns the number of images in the sprite sheet.
    pub fn len(&self) -> usize {
        unsafe { C2D_SpriteSheetCount(self.ptr) }
    }

    /// Returns `true` of the sprite sheet contains no image.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Creates a [`Sprite`] from the image at the given index.
    ///
    /// # Errors
    ///
    /// Returns [`Error::SpriteIndexOutOfBounds`] if index >= len.
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
    /// Sets the position of the sprite.
    pub fn set_pos(&mut self, x: f32, y: f32) {
        unsafe {
            C2D_SpriteSetPos(&mut self.inner, x, y);
        }
    }

    /// Sets the scale of the sprite.
    ///
    /// Negative values flip the sprite horizontally or vertically.
    pub fn set_scale(&mut self, x: f32, y: f32) {
        unsafe {
            C2D_SpriteSetScale(&mut self.inner, x, y);
        }
    }

    /// Sets the rotattion of the sprite in radians.
    pub fn set_rotation(&mut self, radians: f32) {
        unsafe {
            C2D_SpriteSetRotation(&mut self.inner, radians);
        }
    }

    /// Sets the rotattion of the sprite in degrees.
    pub fn set_rotation_degrees(&mut self, degrees: f32) {
        unsafe {
            C2D_SpriteSetRotationDegrees(&mut self.inner, degrees);
        }
    }

    /// Sets the center point of the sprite in used for rotation and positioning.
    pub fn set_center(&mut self, x: f32, y: f32) {
        unsafe {
            C2D_SpriteSetCenter(&mut self.inner, x, y);
        }
    }

    /// Sets the depth of the sprite.
    pub fn set_depth(&mut self, depth: f32) {
        unsafe {
            C2D_SpriteSetDepth(&mut self.inner, depth);
        }
    }

    /// Draws the sprite.
    pub fn draw(&self) {
        unsafe {
            C2D_DrawSprite(&self.inner);
        }
    }
}
