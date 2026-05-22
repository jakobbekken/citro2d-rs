use citro2d::{Citro2d, Color, SpriteSheet, TextBuf};
use ctru::prelude::*;

fn main() {
    let gfx = Gfx::new().expect("Failed to init gfx");
    let mut hid = Hid::new().expect("Failed to init HID");
    let apt = Apt::new().expect("Failed to init APT");

    let c2d = Citro2d::new(&gfx).expect("Failed to init citro2d");

    let black = Color::rgb(0, 0, 0);
    let white = Color::rgb(255, 255, 255);
    let blue = Color::rgb(50, 100, 200);
    let green = Color::rgb(50, 180, 50);

    let text_buf = TextBuf::new(256).expect("Failed to create text buffer");

    let sheet = SpriteSheet::from_mem(include_bytes!("../assets/test.t3x"))
        .expect("Failed to load sprite sheet");

    let mut sprite = sheet.sprite(0).expect("Failed to get sprite");
    sprite.set_scale(0.5, 0.5);
    sprite.set_center(0.5, 0.5);
    sprite.set_pos(200.0, 120.0);

    while apt.main_loop() {
        hid.scan_input();
        if hid.keys_down().intersects(KeyPad::START) {
            break;
        }

        c2d.frame(|frame| {
            text_buf.clear();
            let life = text_buf.parse("20").unwrap();

            frame.scene(c2d.top_screen(), black, |scene| {
                scene.draw_rect(0.0, 0.0, 400.0, 240.0, blue);
                scene.draw_text(&life, 30.0, 100.0, 2.0, white);
                scene.draw_sprite(&sprite);
            });
            frame.scene(c2d.bottom_screen(), black, |scene| {
                scene.draw_rect(0.0, 0.0, 320.0, 240.0, green);
                scene.draw_line(0.0, 120.0, 320.0, 120.0, 3.0, white);
            });
        });
    }
}
