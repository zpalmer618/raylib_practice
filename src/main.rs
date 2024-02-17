use donkey::{colors::{BLACK, BLUE, GRAY, GREEN, ORANGE, RED, WHITE}, keys::Key, Window};

enum GameScreen {
    Logo,
    Title,
    Gameplay,
    Ending
}

fn main() {
    let w = 800;
    let h = 600;
    let title = "Game Window";
    let win = Window::init(w, h, title);
    win.set_target_fps(60);
    let mut currentscreen = GameScreen::Logo;
    while !win.should_close() {
        win.begin_drawing();
        match currentscreen {
            GameScreen::Logo => {
                win.clear_background(GRAY);
                win.draw_text("Here is where the logo for the game will go",
                    100,
                    100,
                    25,
                    BLACK
                );
                if win.is_key_pressed(Key::Enter) {
                    currentscreen = GameScreen::Title
                }
            }
            GameScreen::Title => {
                win.clear_background(RED);
                win.draw_text("Here is where the title screen will go",
                    100,
                    100,
                    25,
                    WHITE
                );
                if win.is_key_pressed(Key::Enter) {
                    currentscreen = GameScreen::Gameplay
                }
            }
            GameScreen::Gameplay => {
                win.clear_background(BLACK);
                win.draw_text("Move your rectangle with WASD",
                    100,
                    100,
                    25,
                    ORANGE
                );
                if win.is_key_pressed(Key::Enter) {
                    currentscreen = GameScreen::Ending
                }
            }
            GameScreen::Ending => {
                win.clear_background(BLUE);
                win.draw_text("Here is where the ending screen will go",
                    100,
                    100,
                    25,
                    GREEN
                );
                if win.is_key_pressed(Key::Enter) {
                    currentscreen = GameScreen::Title
                }
            }
        }
        win.end_drawing()
    }
}
