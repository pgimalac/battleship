use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[macro_export]
macro_rules! result_map {
    ($x : expr, $g : expr, $h : expr) => {
        match $x {
            Ok(o) => Ok($g(o)),
            Err(e) => Err($h(e)),
        }
    };
}

#[macro_export]
macro_rules! try_string {
    ($x : expr) => {
        match $x {
            Ok(o) => o,
            Err(e) => return Err(e.to_string()),
        }
    };
}

pub fn fill_circle(
    canvas: &mut Canvas<Window>,
    color: Color,
    x: i32,
    y: i32,
    r: i32,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    for i in 0..r {
        let mut j = 0;
        while i >= j && (i * i) + (j * j) <= (r * r) {
            canvas.draw_point(Point::new(x + i, y + j))?;
            canvas.draw_point(Point::new(x + i, y - j))?;
            canvas.draw_point(Point::new(x - i, y + j))?;
            canvas.draw_point(Point::new(x - i, y - j))?;
            canvas.draw_point(Point::new(x + j, y + i))?;
            canvas.draw_point(Point::new(x + j, y - i))?;
            canvas.draw_point(Point::new(x - j, y + i))?;
            canvas.draw_point(Point::new(x - j, y - i))?;

            j += 1;
        }
    }
    Ok(())
}

pub const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
};
pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 0,
};
pub const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 0,
};
pub const BLUE: Color = Color {
    r: 0,
    g: 0,
    b: 255,
    a: 0,
};
pub const GREEN: Color = Color {
    r: 0,
    g: 255,
    b: 0,
    a: 0,
};
pub const MAGENTA: Color = Color {
    r: 255,
    g: 0,
    b: 255,
    a: 0,
};
pub const _CYAN: Color = Color {
    r: 0,
    g: 255,
    b: 255,
    a: 0,
};
pub const YELLOW: Color = Color {
    r: 255,
    g: 255,
    b: 0,
    a: 0,
};
