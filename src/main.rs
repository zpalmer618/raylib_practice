use donkey::{colors::GRAY, Window};

fn main() {
    let w = 800;
    let h = 600;
    let win = Window::init(w, h, "Window for fun");
    win.set_target_fps(60);
    let color = GRAY;
    while !win.should_close() {
        win.begin_drawing();
        win.clear_background(color);
        win.end_drawing()
    }
}
