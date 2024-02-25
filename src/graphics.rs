use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::WindowCanvas,
    video::Window,
};
pub struct Renderer {
    pub canvas: WindowCanvas,
    pub point_size: u32,
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

    pub fn draw(&mut self) -> Result<(), String> {
        self.canvas.clear();

        self.draw_dot(&Point::new(3, 3), Color::RED)?;
        self.draw_rect(&Point::new(3, 1), 200, 300, Color::BLACK)?;

        self.canvas.present();
        self.canvas.clear();

        Ok(())
    }
}
