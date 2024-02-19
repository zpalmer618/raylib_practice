use donkey::{
    colors::{BLACK, GRAY},
    keys::Key,
    Rectangle, Window,
};

// Define Screen Management
enum GameScreen {
    Logo,
    Title,
    Gameplay,
    Ending,
}

fn main() {
    let w = 1280;
    let h = 720;
    let title = "Game Window";
    let win = Window::init(w, h, title);
    win.set_target_fps(60);
    let mut box_a = Rectangle {
        x: 10.,
        y: (win.get_screen_height() as f32 / 2. - 50.) as f32,
        width: w as f32 / 3.2,
        height: h as f32 / 3.6,
    };
    let mut currentscreen = GameScreen::Logo;
    while !win.should_close() {
        win.begin_drawing();
        match currentscreen {
            GameScreen::Logo => {
                win.clear_background(GRAY);
                win.draw_text(
                    "Here is where the logo for the game will go",
                    100,
                    100,
                    25,
                    BLACK,
                );
                if win.is_key_pressed(Key::Enter) {
                    currentscreen = GameScreen::Title
                }
            }
            GameScreen::Title => {
                win.clear_background(GRAY);
                win.draw_text(
                    "Here is where the title screen will go",
                    100,
                    100,
                    25,
                    BLACK,
                );
                if win.is_key_pressed(Key::Enter) {
                    currentscreen = GameScreen::Gameplay
                }
            }
            GameScreen::Gameplay => {
                let fac = if win.is_key_down(Key::LShift) {
                    2.0
                } else {
                    1.0
                };
                if win.is_key_down(Key::W) {
                    box_a.y -= 2.0 * fac;
                }
                if win.is_key_down(Key::A) {
                    box_a.x -= 2.0 * fac;
                }
                if win.is_key_down(Key::S) {
                    box_a.y += 2.0 * fac;
                }
                if win.is_key_down(Key::D) {
                    box_a.x += 2.0 * fac;
                }
                win.clear_background(GRAY);
                win.draw_text("Move your rectangle with WASD", 100, 100, 25, BLACK);
                win.draw_rectangle_rec(box_a, BLACK);
                if win.is_key_pressed(Key::Enter) {
                    currentscreen = GameScreen::Ending
                }
            }
            GameScreen::Ending => {
                win.clear_background(GRAY);
                win.draw_text(
                    "Here is where the ending screen will go",
                    100,
                    100,
                    25,
                    BLACK,
                );
                if win.is_key_pressed(Key::Enter) {
                    currentscreen = GameScreen::Title
                }
            }
        }
        win.end_drawing()
    }
}

// The below is another way of manual making a movable rectangle.

// This would go at the top of fn main()
// let mut rectpos = donkey::vector2!((w / 2) as f32, (h / 2) as f32);
// let rectsize = donkey::vector2!(200., 100.);

// This goes where you begin drawing the window.

// win.draw_rectangle_v(rectpos, rectsize, BLACK);
// win.draw_text(
//     "Player 1",
//     (rectpos.x + win.measure_text("Player 1", 20) as f32 * 0.7) as usize,
//     rectpos.y as usize,
//     20,
//     WHITE,
// );
