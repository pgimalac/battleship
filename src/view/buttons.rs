use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub struct Button {
    background: Color,
    position: Rect,
    _text: String,
    _text_color: Color,
    action: *mut Box<FnMut() -> bool>,
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
        action: Box<FnMut() -> bool>,
    ) -> Button {
        Button {
            position: Rect::new(x, y, w as u32, h as u32),
            background,
            _text,
            _text_color,
            action: Box::into_raw(Box::new(action)),
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(self.background);
        canvas.fill_rect(self.position)
    }

    pub fn contains_point<P: Into<(i32, i32)>>(&self, point: P) -> bool {
        self.position.contains_point(point)
    }

    pub fn execute(&mut self) -> bool {
        unsafe { (*self.action)() }
    }
}

// impl Drop for Button {
//     fn drop(&mut self) {
//         let mut f: *mut Box<FnMut() -> bool> = std::ptr::null_mut();
//         std::mem::swap(&mut f, &mut self.action);
//         unsafe {
//             std::ptr::drop_in_place(f);
//         }
//     }
// }
