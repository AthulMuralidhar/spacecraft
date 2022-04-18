
// the project closly follows https://sunjay.dev/learn-game-dev/intro.html

// for compeltion reference
// https://sunjay.dev/learn-game-dev/opening-a-window.html
// https://sunjay.dev/learn-game-dev/rendering-an-image.html


use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::render::Texture;

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
    let texture = texture_creator.load_texture("assets/box.png")?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        i = (i + 1) % 255;

        render( &mut canvas, Color::RGB(i, 255, 255 - i), &texture)?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}


fn render(canvas: &mut Canvas<Window>,color: Color, texture: &Texture) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();
    canvas.copy(texture, None, None)?;
    canvas.present();
    Ok(())
}