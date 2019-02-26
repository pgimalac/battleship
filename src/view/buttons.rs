use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub struct Button {
    background: Color,
    position: Rect,
    _text: String,
    _text_color: Color,
    action: Box<FnMut() -> ()>,
}

impl Button {
    pub fn new(
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        background: Color,
        _text: String,
        _text_color: Color,
        action: Box<FnMut() -> ()>,
    ) -> Button {
        Button {
            position: Rect::new(x, y, w as u32, h as u32),
            background,
            _text,
            _text_color,
            action,
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(self.background);
        canvas.fill_rect(self.position)?;

        Ok(())
    }

    pub fn contains_point<P: Into<(i32, i32)>>(&self, point: P) -> bool {
        self.position.contains_point(point)
    }

    pub fn execute(&mut self) {
        (self.action)();
    }
}
