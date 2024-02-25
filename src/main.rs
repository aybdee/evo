mod environment;
mod graphics;
mod types;

extern crate sdl2;
use std::time::Duration;

use graphics::Renderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Evo - natural selection", 500, 500)
        // .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut renderer = Renderer::new(window, 5)?;
    renderer.draw()?;
    std::thread::sleep(Duration::from_secs(5));

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                _ => {}
            }
        }

        renderer.canvas.clear();
        renderer.canvas.present();
    }

    Ok(())
}

