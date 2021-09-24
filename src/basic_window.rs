use raylib::prelude::*;

pub fn run() {
    let (mut rl, thread) = raylib::init()
        .size(800, 450)
        .title("My game")
        .build();
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_text("Congrats! You created your first window!", 190, 200, 20, Color::LIGHTGRAY);
    }
}