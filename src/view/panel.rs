use crate::utils::*;
use crate::view::buttons::Button;
use sdl2::{
    event::{Event, Event::MouseButtonUp},
    mouse::{MouseButton, MouseState},
    pixels::Color,
    render::Canvas,
    video::Window,
};

pub const QUIT_COLOR: Color = RED;
pub const TEXT_COLOR: Color = BLACK;

pub trait Panel {
    fn button_vec_mut(&mut self) -> &mut Vec<Button>;
    fn button_vec(&self) -> &Vec<Button>;

    fn render(&self, canvas: &mut Canvas<Window>, _mouse_state: MouseState) -> Result<(), String> {
        canvas.set_draw_color(WHITE);
        canvas.fill_rect(None)?;
        for button in self.button_vec() {
            button.render(canvas)?;
        }
        Ok(())
    }

    // the Ok part is true to 'continue' the main loop (go back to the beginning) and false otherwise
    fn manage_event(&mut self, event: Event) -> Result<bool, String> {
        if let MouseButtonUp {
            mouse_btn: MouseButton::Left,
            x,
            y,
            ..
        } = event
        {
            for button in self.button_vec_mut() {
                if button.contains_point((x, y)) {
                    button.execute();
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    // called each loop turn
    // does nothing by default
    // the Ok part is true to 'continue' the main loop (go back to the beginning) and false otherwise
    fn do_loop(&mut self) -> Result<bool, String> {
        Ok(false)
    }
}
