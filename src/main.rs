use raylib::prelude::*;

fn main() {
    const SCREEN_WIDTH: i32 = 640;
    const SCREEN_HEIGHT: i32 = 480;
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hello, World")
        .build();

    let num = 50;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

        if num > 50 {
            d.draw_text(
                &*format!("The num {} is greater than 50", num),
                12, 24, 20, Color::RED);
        } else {
            d.draw_text(
                &*format!("The num {} is not greater than 50", num),
                12, 24, 20, Color::BLUE);
        }

        d.draw_circle(50, SCREEN_HEIGHT - 50, 45.0, Color::DARKBLUE);
        d.draw_rectangle(SCREEN_WIDTH - 50, SCREEN_HEIGHT - 50, 45, 45, Color::DARKPURPLE);
    }
}