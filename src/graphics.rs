use core::fmt;

use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::{TextureQuery, WindowCanvas},
    ttf::Font,
    video::Window,
};
pub struct Renderer {
    pub canvas: WindowCanvas,
    pub point_size: u32,
}

impl fmt::Debug for Renderer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Renderer")
            .field("point_size", &self.point_size)
            .finish()
    }
}

impl Renderer {
    pub fn new(window: Window, point_size: u32) -> Result<Self, String> {
        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        canvas.set_draw_color(Color::WHITE);
        return Ok(Renderer { canvas, point_size });
    }

    pub fn draw_dot(&mut self, point: &Point, color: Color) -> Result<(), String> {
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(Rect::new(
            point.x() * (self.point_size as i32),
            point.y() * (self.point_size as i32),
            self.point_size,
            self.point_size,
        ))
    }

    pub fn draw_rect(
        &mut self,
        point: &Point,
        width: u32,
        height: u32,
        color: Color,
    ) -> Result<(), String> {
        self.canvas.set_draw_color(color);
        self.canvas
            .draw_rect(Rect::new(point.x(), point.y(), width, height))
    }

    pub fn get_size(&self) -> (u32, u32) {
        self.canvas.window().size()
    }
    pub fn clear(&mut self) {
        self.canvas.clear();
    }
    pub fn present(&mut self) {
        self.canvas.present();
        self.canvas.clear();
    }

    fn get_rect(&self, rect_width: u32, rect_height: u32, x: i32, y: i32) -> Rect {
        let padding = 64;
        let cons_width = self.canvas.window().size().0 - padding;
        let cons_height = self.canvas.window().size().1 - padding;
        let wr = rect_width as f32 / cons_width as f32;
        let hr = rect_height as f32 / cons_height as f32;

        let (w, h) = if wr > 1f32 || hr > 1f32 {
            if wr > hr {
                println!("Scaling down! The text will look worse!");
                let h = (rect_height as f32 / wr) as i32;
                (cons_width as i32, h)
            } else {
                println!("Scaling down! The text will look worse!");
                let w = (rect_width as f32 / hr) as i32;
                (w, cons_height as i32)
            }
        } else {
            (rect_width as i32, rect_height as i32)
        };

        // let cx = (SCREEN_WIDTH as i32 - w) / 2;
        // let cy = (SCREEN_HEIGHT as i32 - h) / 2;
        Rect::new(x, y, w as u32, h as u32)
    }

    pub fn render_text(&mut self, text: &str, font: Font, position: Point) -> Result<(), String> {
        let texture_creator = self.canvas.texture_creator();
        let surface = font
            .render(&text)
            .blended(Color::RGBA(255, 0, 0, 255))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();

        // If the example text is too big for the screen, downscale it (and center irregardless)
        let padding = 64;
        let target = self.get_rect(width, height, position.x(), position.y());

        self.canvas.copy(&texture, None, Some(target))?;

        return Ok(());
    }
}
