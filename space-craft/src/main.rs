
// the project closly follows https://sunjay.dev/learn-game-dev/intro.html

// for compeltion reference
// https://sunjay.dev/learn-game-dev/opening-a-window.html
// https://sunjay.dev/learn-game-dev/rendering-an-image.html
// https://sunjay.dev/learn-game-dev/single-sprite.html
// https://sunjay.dev/learn-game-dev/refactor-player-struct.html
// https://sunjay.dev/learn-game-dev/simple-movement.html

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::render::Texture;
use sdl2::rect::Rect;
use sdl2::rect::Point;


struct Player {
    position: Point,
    sprite: Rect,
    speed: i32,
}


fn render(canvas: &mut Canvas<Window>,
    color: Color, 
    texture: &Texture,
    player: &Player,
) -> Result<(), String> {

    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let screen_position = player.position + Point::new(width as i32 / 2, height as i32/ 2);
    let screen_rect = Rect::from_center(screen_position, player.sprite.width(), player. sprite.height());
    

    canvas.copy(texture, player.sprite, screen_rect)?;
    canvas.present();
    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG)?;


    let window = video_subsystem.window("simple box demo", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    // image size in px:  312px by 288px
    let texture = texture_creator.load_texture("assets/reaper.png")?;

    // init player position
    let mut player = Player {
        // world co-ordinates - normalized cartician at orgin == centre of screen
        position: Point::new(0,0),
        //  each sprite fits into a  26px by 36px rectangle.
        sprite: Rect::new(0,0,26,36),
        // 0-10,
        speed: 5,
    };

    // main game loop
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();
        for event in event_pump.poll_iter() {
            // A nuanced thing about this is that we're handling all the queued events at once. 
            // ie. parallelly processed
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown {keycode: Some(Keycode::Left), ..} => {
                    // move from centre
                    player.position = player.position.offset(-player.speed,0);

                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    player.position = player.position.offset(player.speed, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    player.position = player.position.offset(0, -player.speed);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    player.position = player.position.offset(0, player.speed);
                },
                _ => {}
            }
        }
        // the update
        i = (i + 1) % 255;

        // the renderr
        render( &mut canvas, Color::RGB(i, 255, 255 - i), &texture, &player)?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}


