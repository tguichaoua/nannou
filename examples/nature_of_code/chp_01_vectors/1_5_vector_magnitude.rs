// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Example 1-5: Vector Magnitude
extern crate nannou;

use nannou::prelude::*;

fn main() {
    nannou::view(view);
}

fn view(app: &App, frame: Frame) -> Frame {
    app.main_window().set_inner_size_points(640.0, 360.0);

    // Begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);

    let mut mouse = vec2(app.mouse.x, app.mouse.y);
    let center = vec2(0.0, 0.0);
    mouse -= center;

    let m = mouse.magnitude();

    draw.rect()
        .xy(app.window_rect().top_left())
        .w_h(m, 10.0)
        .color(BLACK);

    draw.line()
        .start(pt2(0.0, 0.0))
        .end(pt2(mouse.x, mouse.y))
        .thickness(2.0)
        .color(BLACK);

    // Write the result of our drawing to the window's OpenGL frame.
    draw.to_frame(app, &frame).unwrap();

    // Return the drawn frame.
    frame
}
