use raylib::prelude::*;

pub fn run() {
    const SCREEN_WIDTH: i32 = 800;
    const SCREEN_HEIGHT: i32 = 450;
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Move the ball with arrow keys")
        .build();
    rl.set_target_fps(60);
    let mut ball_position = Vector2{
        x: (SCREEN_WIDTH / 2) as f32,
        y: (SCREEN_HEIGHT / 2) as f32,
    };
    while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_LEFT) { ball_position.x -= 2.0; }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) { ball_position.x += 2.0; }
        if rl.is_key_down(KeyboardKey::KEY_UP) { ball_position.y -= 2.0; }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) { ball_position.y += 2.0; }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_text("Move the ball with arrow keys", 10, 10, 20, Color::DARKGRAY);
        d.draw_circle_v(ball_position, 50.0, Color::MAROON);
    }
}