#![no_std]

use citro2d_sys::*;
use ctru::services::gfx::Gfx;

pub mod color;
pub mod error;
pub mod text;
pub use color::Color;
pub use error::Error;
pub use text::{Text, TextBuf};

const C3D_DEFAULT_CMDBUF_SIZE: usize = 0x40000;
const C3D_FRAME_SYNCDRAW: u8 = 0x01;

pub struct Citro2d<'gfx> {
    _gfx: &'gfx Gfx,
    top: RenderTarget,
    bot: RenderTarget,
}

pub struct RenderTarget {
    ptr: *mut C3D_RenderTarget,
}

pub struct Frame;

pub struct Scene;

impl<'gfx> Citro2d<'gfx> {
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

    pub fn top_screen(&self) -> &RenderTarget {
        &self.top
    }

    pub fn bottom_screen(&self) -> &RenderTarget {
        &self.bot
    }

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
    pub fn scene<F: FnOnce(&mut Scene)>(
        &mut self,
        target: &RenderTarget,
        clear_color: Color,
        f: F,
    ) {
        unsafe {
            C2D_TargetClear(target.ptr, clear_color.into());
            C2D_SceneBegin(target.ptr);
        }
        let mut scene = Scene;
        f(&mut scene);
    }
}

impl Scene {
    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        unsafe {
            C2D_DrawRectSolid(x, y, 0.0, w, h, color.into());
        }
    }

    pub fn draw_circle(&mut self, x: f32, y: f32, radius: f32, color: Color) {
        unsafe {
            C2D_DrawCircleSolid(x, y, 0.0, radius, color.into());
        }
    }

    pub fn draw_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, thickness: f32, color: Color) {
        unsafe {
            C2D_DrawLine(x0, y0, color.into(), x1, y1, color.into(), thickness, 0.0);
        }
    }

    pub fn draw_text(&mut self, text: &Text, x: f32, y: f32, scale: f32, color: Color) {
        text.draw(x, y, 0.0, scale, color);
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
