use sdl2::VideoSubsystem;

/*
The project closely follows https://sunjay.dev/learn-game-dev/intro.html

Extras needed to run on linux:
- sudo apt-get install libsdl2-dev
*/

fn init_game_window() -> Result<VideoSubsystem, String> {
    let video_subsystem = sdl2::init()?.video();
    return video_subsystem;
}
fn main() {
    init_game_window().ok();
}

//  TESTS
#[cfg(test)]
mod tests {
    use crate::init_game_window;

    #[test]
    fn it_renders_the_window() {
        assert!(init_game_window().is_ok())
    }
}
