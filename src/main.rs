mod environment;
mod graphics;
mod types;

extern crate sdl2;
use environment::State;
use graphics::Renderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

const SCREEN_WIDTH: u32 = 800; //num columns(x)
const SCREEN_HEIGHT: u32 = 600; //num rows(y)

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Evo - natural selection", SCREEN_WIDTH, SCREEN_HEIGHT)
        // .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let point_size = 5;
    let mut renderer = Renderer::new(window, point_size)?;

    let mut environment = State::new(
        600 / (point_size as usize),
        800 / (point_size as usize),
        Some(&mut renderer),
    );

    environment.initialize_organism((16, 0));
    // environment.initialize_organisms_random(500);
    environment.display()?;

    // renderer.draw_rect(Point::new(3, 3), 750, 550, Color::BLACK)?;

    // let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    // let texture_creator = renderer.canvas.texture_creator();
    //
    // // Load a font
    // let mut font = ttf_context.load_font("./c.ttf", 20)?;
    // font.set_style(sdl2::ttf::FontStyle::NORMAL);

    // render a surface, and convert it to a texture bound to the canvas
    // renderer.present();

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
    }

    Ok(())
}
