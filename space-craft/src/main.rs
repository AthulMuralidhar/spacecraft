use std::io;

fn init_game_window() -> Result<i32, io::Error> {
    return Ok(0);
}
fn main() {
    init_game_window();
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
