use raylib::prelude::*;

pub fn run() {
    const SCREEN_WIDTH: i32 = 800;
    const SCREEN_HEIGHT: i32 = 450;

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Move ball with mouse")
        .build();

    let colors = [Color::RED, Color::GREEN, Color::DARKBLUE];
    let mut color_index = 0;
    let mut ball_color = colors[color_index];
    let mut ball_position;
    while !rl.window_should_close() {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            color_index += 1;
            if color_index >= colors.len() {
                color_index = 0;
            }
            ball_color = colors[color_index];
        }
        ball_position = rl.get_mouse_position();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_circle_v(ball_position, 40.0, ball_color);
    }
}