use citro2d::Citro2d;
use citro2d_sys::C2D_Color32;
use ctru::prelude::*;

fn main() {
    let gfx = Gfx::new().expect("Failed to init gfx");
    let mut hid = Hid::new().expect("Failed to init HID");
    let apt = Apt::new().expect("Failed to init APT");

    let c2d = Citro2d::new(&gfx).expect("Failed to init citro2d");

    let black = unsafe { C2D_Color32(0, 0, 0, 255) };
    let red = unsafe { C2D_Color32(200, 50, 50, 255) };
    let white = unsafe { C2D_Color32(255, 255, 255, 255) };
    let blue = unsafe { C2D_Color32(50, 100, 200, 255) };
    let green = unsafe { C2D_Color32(50, 180, 50, 255) };

    while apt.main_loop() {
        hid.scan_input();
        if hid.keys_down().intersects(KeyPad::START) {
            break;
        }

        c2d.frame(|frame| {
            frame.scene(c2d.top_screen(), black, |scene| {
                scene.draw_rect(0.0, 0.0, 400.0, 240.0, red);
                scene.draw_rect(100.0, 60.0, 200.0, 120.0, white);
                scene.draw_circle(200.0, 120.0, 40.0, blue);
            });
            frame.scene(c2d.bottom_screen(), black, |scene| {
                scene.draw_rect(0.0, 0.0, 320.0, 240.0, green);
                scene.draw_line(0.0, 120.0, 320.0, 120.0, 3.0, white);
            });
        });
    }
}
