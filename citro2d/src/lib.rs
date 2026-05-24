//! Safe Rust bindings for the citro2d graphic library for Nintendo 3DS.
//! citro2d is a 2D abstraction layer built on top of citro3d and PICA200 GPU.
//! This crate provides a safe and idiomatic Rust API for drawing shapes, text, and images on both screens.
//!
//! # Structure
//! - `citro2d` - Safe, idiomatic wrapper around `citro2d-sys`
//! - `citro2d-sys` - Low-level, unsafe bindings for the graphic library
//! - `citro2d-test` - A program to test on 3DS
//!
//! # Getting started
//!
//! ```
//! use citro2d::{Citro2d, Color, TextBuf};
//! use ctru::prelude::*;
//!
//! let gfx = Gfx::new().expect("Failed to init gfx");
//! let apt = Apt::new().expect("Failed to init APT");
//! let c2d = Citro2d::new(&gfx).expect("Failed to init citro2d");
//!
//! let text_buf = TextBuf::new(256).expect("Failed to create text buffer");
//! let red = Color::rgb(255, 0, 0);
//! let black = Color::rgb(0, 0, 0);
//!
//! while apt.main_loop() {
//!     text_buf.clear();
//!     let text = text_buf.parse("Hello, 3DS!").unwrap();
//!
//!     c2d.frame(|frame| {
//!         frame.scene(c2d.top_screen(), black, |scene| {
//!             scene.draw_rect(0.0, 0.0, 400.0, 240.0, red);
//!             scene.draw_text(&text, 10.0, 10.0, 1.0, Color::rgb(255, 255, 255));
//!         });
//!     });
//! }
//! ```
//!
//! # Architecture
//!
//! The API is structured around:
//! - [`Citro2d`] - owns the graphic context and screen render
//! - [`Frame`] - repressents an active frame, only existing inside a [`Citro2d::frame`] closure
//! - [`Scene`] - repressents drawing to a specific screen, only exists inside a [`Frame::scene`] closure
//!
//! Closure-based design makes it impossible to draw outside a frame or forget to end a frame, since the compiler enforces correct usage.

#![no_std]
#![doc(test(attr(no_run)))]

use citro2d_sys::*;
use ctru::services::gfx::Gfx;

pub mod color;
pub mod error;
pub mod sprite;
pub mod text;
pub use color::Color;
pub use error::Error;
pub use sprite::{Sprite, SpriteSheet};
pub use text::{Text, TextBuf};

const C3D_DEFAULT_CMDBUF_SIZE: usize = 0x40000;
const C3D_FRAME_SYNCDRAW: u8 = 0x01;

/// The main citro2d context.
///
/// Owns the citro3d and citro2d lifecycle and both screen render targets.
/// Must not outlive the [`Gfx`] it was created with.
///
/// # Examples
///
/// ```
/// let gfx = Gfx::new().expect("Failed to init gfx");
/// let c2d = Citro2d::new(&gfx).expect("Failed to init citro2d");
/// ```
pub struct Citro2d<'gfx> {
    _gfx: &'gfx Gfx,
    top: RenderTarget,
    bot: RenderTarget,
}

/// A render target repressenting a single 3DS screen.
///
/// Obtained via [`Citro2d::top_screen`] or [`Citro2d::bottom_screen`].
/// Passed to [`Frame::scene`] to begin drawing to that screen.
pub struct RenderTarget {
    ptr: *mut C3D_RenderTarget,
}

/// Represents an active frame.
///
/// Only exists inside a [`Citro2d::frame`] closure.
/// `C3D_FrameBegin` is called before the closure and `C3D_FrameEnd` is called after.
/// This makes it impossible to forget to end a frame.
pub struct Frame;

/// Represents an active drawin scene on a render target.
///
/// Only exists inside a [`Frame::scene`] closure.
/// The target is cleared and set as the active render target before the closure runs.
pub struct Scene;

impl<'gfx> Citro2d<'gfx> {
    /// Initializes citro3d and citro2d, creating the render targets for both screens.
    ///
    /// # Errors
    ///
    /// Returns [`Error::C3dInitFailed`] if citro3d fails to initialize.
    /// Returns [`Error::C2dInitFailed`] if citro2d fails to initialize.
    ///
    /// # Examples
    ///
    /// ```
    /// let gfx = Gfx::new().expect("Failed to init gfx");
    /// let c2d = Citro2d::new(&gfx)?;
    /// ```
    pub fn new(gfx: &'gfx Gfx) -> Result<Self, Error> {
        unsafe {
            if !C3D_Init(C3D_DEFAULT_CMDBUF_SIZE) {
                return Err(Error::C3dInitFailed);
            }

            if !C2D_Init(C2D_DEFAULT_MAX_OBJECTS as usize) {
                C3D_Fini();
                return Err(Error::C2dInitFailed);
            }

            C2D_Prepare();

            let top = RenderTarget {
                ptr: C2D_CreateScreenTarget(GFX_TOP, GFX_LEFT),
            };
            let bot = RenderTarget {
                ptr: C2D_CreateScreenTarget(GFX_BOTTOM, GFX_LEFT),
            };

            Ok(Self {
                _gfx: gfx,
                top,
                bot,
            })
        }
    }

    /// Returns a reference to the top screen render target.
    pub fn top_screen(&self) -> &RenderTarget {
        &self.top
    }

    /// Returns a reference to the bottom screen render target.
    pub fn bottom_screen(&self) -> &RenderTarget {
        &self.bot
    }

    /// Renders a frame.
    ///
    /// Calls `C3D_FrameBegin` before the closure and `C3D_FrameEnd` after.
    /// All drawing must happen inside this closure via [`Frame::scene`].
    ///
    /// # Examples
    ///
    /// ```
    /// c2d.frame(|frame| {
    ///     frame.scene(c2d.top_screen(), Color::rgb(0, 0, 0), |scene| {
    ///         scene.draw_rect(0.0, 0.0, 400.0, 240.0, Color::rgb(255, 0, 0));
    ///     });
    /// });
    /// ```
    pub fn frame<F: FnOnce(&mut Frame)>(&self, f: F) {
        unsafe {
            C3D_FrameBegin(C3D_FRAME_SYNCDRAW);
        }
        let mut frame = Frame;
        f(&mut frame);
        unsafe {
            C3D_FrameEnd(0);
        }
    }
}

impl Frame {
    /// Begins drawing to the given render target.
    ///
    /// Clears the target with `clear_color` and sets it as active before calling the closure
    /// All draw calls inside the closure go to this target.
    ///
    /// # Examples
    ///
    /// ```
    /// frame.scene(c2d.top_screen(), Color::rgb(0, 0, 0), |scene| {
    ///     scene.draw_rect(0.0, 0.0, 400.0, 240.0, Color::rgb(255, 0, 0));
    /// });
    /// ```
    pub fn scene<F: FnOnce(&mut Scene)>(
        &mut self,
        target: &RenderTarget,
        clear_color: Color,
        f: F,
    ) {
        unsafe {
            C2D_TargetClear(target.ptr, clear_color.value);
            C2D_SceneBegin(target.ptr);
        }
        let mut scene = Scene;
        f(&mut scene);
    }
}

impl Scene {
    /// Draws a solid rectangle.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinates of the top-left corner
    /// * `y` - Y coordinates of the top-left corner
    /// * `w` - Width in pixels
    /// * `h` - Height in pixels
    /// * `color` - Fill color
    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        unsafe {
            C2D_DrawRectSolid(x, y, 0.0, w, h, color.value);
        }
    }

    /// Draws a triangle with colors per vertex.
    ///
    /// # Arguments
    ///
    /// * `x0` - X coordinates of the first vertex
    /// * `y0` - Y coordinates of the first vertex
    /// * `x1` - X coordinates of the second vertex
    /// * `y1` - Y coordinates of the second vertex
    /// * `x2` - X coordinates of the third vertex
    /// * `y2` - Y coordinates of the third vertex
    /// * `color0` - First vertex color
    /// * `color1` - Second vertex color
    /// * `color2` - Third vertex color
    pub fn draw_triangle(
        &mut self,
        x0: f32,
        y0: f32,
        color0: Color,
        x1: f32,
        y1: f32,
        color1: Color,
        x2: f32,
        y2: f32,
        color2: Color,
    ) {
        unsafe {
            C2D_DrawTriangle(
                x0,
                y0,
                color0.value,
                x1,
                y1,
                color1.value,
                x2,
                y2,
                color2.value,
                0.0,
            );
        }
    }

    /// Draws a solid triangle.
    ///
    /// # Arguments
    ///
    /// * `x0` - X coordinates of the first vertex
    /// * `y0` - Y coordinates of the first vertex
    /// * `x1` - X coordinates of the second vertex
    /// * `y1` - Y coordinates of the second vertex
    /// * `x2` - X coordinates of the third vertex
    /// * `y2` - Y coordinates of the third vertex
    /// * `color` - Fill color
    pub fn draw_triangle_solid(
        &mut self,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        color: Color,
    ) {
        self.draw_triangle(x0, y0, color, x1, y1, color, x2, y2, color);
    }

    /// Draws a solid circle.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinates of the top-left corner
    /// * `y` - Y coordinates of the top-left corner
    /// * `radius` - Radius in pixels
    /// * `color` - Fill color
    pub fn draw_circle(&mut self, x: f32, y: f32, radius: f32, color: Color) {
        unsafe {
            C2D_DrawCircleSolid(x, y, 0.0, radius, color.value);
        }
    }

    /// Draws a line between two points.
    ///
    /// # Arguments
    ///
    /// * `x0` - X coordinates of the start point
    /// * `y0` - Y coordinates of the start point
    /// * `x1` - X coordinates of the end point
    /// * `y1` - Y coordinates of the end point
    /// * `thickness` - Line thickness in pixels
    /// * `color` - Line color
    pub fn draw_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, thickness: f32, color: Color) {
        unsafe {
            C2D_DrawLine(x0, y0, color.value, x1, y1, color.value, thickness, 0.0);
        }
    }

    /// Draws a [`Text`] object.
    ///
    /// # Arguments
    ///
    /// * `text` - Parsed text object from [`TextBuf::parse`]
    /// * `x` - X coordinates of the top-left corner
    /// * `y` - Y coordinates of the top-left corner
    /// * `scale` - Font size multiplier, `1.0` is original size
    /// * `color` - Text color
    pub fn draw_text(&mut self, text: &Text, x: f32, y: f32, scale: f32, color: Color) {
        text.draw(x, y, 0.0, scale, color);
    }

    /// Draws a [`Sprite`].
    pub fn draw_sprite(&mut self, sprite: &Sprite) {
        sprite.draw();
    }
}

impl Drop for Citro2d<'_> {
    fn drop(&mut self) {
        unsafe {
            C2D_Fini();
            C3D_Fini();
        }
    }
}
