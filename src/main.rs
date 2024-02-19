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

// fn pause_screen(x: &Window, mut y: bool) {
//     y = y;
//     if x.is_key_pressed(Key::P) {
//         y = !y;
//     }
//     x.begin_drawing();
//     x.clear_background(BLANK);
//     x.draw_text("GAME PAUSED", 200, 200, 25, BLACK);
// }

fn main() {
    // Global variables for the game window and character
    let w = 1280;
    let h = 720;
    let title = "Game Window";
    let win = Window::init(w, h, title);
    let mut currentscreen = GameScreen::Logo;
    win.set_target_fps(60);

    // Character variables
    let mut box_a = Rectangle {
        x: 10.,
        y: (win.get_screen_height() as f32 / 2. - 50.) as f32,
        width: w as f32 / 3.2,
        height: h as f32 / 3.6,
    };

    //Main game loop
    while !win.should_close() {
        win.begin_drawing();

        // Checks for when the screen should change.
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
                // Implements movement and toggles sprinting
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

                // Implements the character rectangle
                win.draw_rectangle_rec(box_a, BLACK);
                if win.is_key_pressed(Key::Enter) {
                    currentscreen = GameScreen::Ending
                }
                // TODO: Make this its own screen? Not sure how to get
                // it to stay on the screen. It just pops on and back
                // off again.
                if win.is_key_pressed(Key::P) {
                    win.draw_text("GAME PAUSED", 200, 200, 50, BLACK);
                };
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
