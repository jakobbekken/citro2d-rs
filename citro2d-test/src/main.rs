use citro2d_sys::*;
use ctru::prelude::*;
use ctru_sys::{GFX_BOTTOM, GFX_LEFT, GFX_TOP};

fn main() {
    let _gfx = Gfx::new().expect("Failed to init gfx");
    let mut hid = Hid::new().expect("Failed to init HID");
    let apt = Apt::new().expect("Failed to init APT");

    unsafe {
        C3D_Init(C3D_DEFAULT_CMDBUF_SIZE as usize);
        C2D_Init(C2D_DEFAULT_MAX_OBJECTS as usize);
        C2D_Prepare();
    }

    let top = unsafe { C2D_CreateScreenTarget(GFX_TOP, GFX_LEFT) };
    let bot = unsafe { C2D_CreateScreenTarget(GFX_BOTTOM, GFX_LEFT) };

    let white = unsafe { C2D_Color32(255, 255, 255, 255) };
    let red = unsafe { C2D_Color32(200, 50, 50, 255) };
    let green = unsafe { C2D_Color32(50, 180, 50, 255) };
    let blue = unsafe { C2D_Color32(50, 100, 200, 255) };
    let black = unsafe { C2D_Color32(0, 0, 0, 255) };

    while apt.main_loop() {
        hid.scan_input();
        if hid.keys_down().intersects(KeyPad::START) {
            break;
        }

        unsafe {
            C3D_FrameBegin(C3D_FRAME_SYNCDRAW as u8);

            C2D_TargetClear(top, black);
            C2D_SceneBegin(top);
            C2D_DrawRectSolid(0.0, 0.0, 0.0, 400.0, 240.0, red);
            C2D_DrawRectSolid(100.0, 60.0, 0.0, 200.0, 120.0, white);
            C2D_DrawCircleSolid(200.0, 120.0, 0.0, 40.0, blue);

            C2D_TargetClear(bot, black);
            C2D_SceneBegin(bot);
            C2D_DrawRectSolid(0.0, 0.0, 0.0, 320.0, 240.0, green);
            C2D_DrawLine(0.0, 120.0, green, 320.0, 120.0, white, 3.0, 0.0);
            C2D_DrawRectangle(60.0, 60.0, 0.0, 200.0, 80.0, red, blue, green, white);

            C3D_FrameEnd(0);
        }
    }

    unsafe {
        C2D_Fini();
        C3D_Fini();
    }
}
