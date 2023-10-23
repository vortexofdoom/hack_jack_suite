use sdl2::{keyboard::Keycode, pixels::{PixelFormatEnum, Color}, render::{Texture, UpdateTextureError, Canvas}, rect::Rect, video::Window};

type Pixel = [u8; 3];

const ON: Pixel = [0, 0, 0];
const OFF: Pixel = [0, 0, 0];

pub struct Screen<'a> {
    canvas: Canvas<Window>,
    texture: Texture<'a>,
}
impl<'a> Screen<'a> {
    pub fn new(canvas: Canvas<Window>, texture: Texture<'a>) -> Self {
        let (mut canvas, mut texture) = (canvas, texture);
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        texture.update(None, &[255; 3 * 512 * 256], 3 * 512).unwrap();
        canvas.present();
        Self { canvas, texture }
    }

    pub fn update(&mut self, addr: i16, value: i16) {
        let rect = Rect::from(get_register(addr));
        let pixels = as_pixels(value);
        self.texture.update(Some(rect), &pixels, 3 << 9).unwrap();
        self.canvas.copy(&self.texture, None, None).unwrap();
        self.canvas.present();
    }
}

const fn get_register(addr: i16) -> (i32, i32, u32, u32) {
    let addr = addr as i32 - 0x4000;
    let row = addr >> 5;
    let col = (addr & 31) << 4;
    (col, row, 16, 1)
}

pub fn as_pixels(value: i16) -> [u8; 48] {
    let mut res = [0; 48];
    for (i, p) in res.chunks_mut(3).enumerate() {
        p.copy_from_slice(if value >> i == 0 {
            &OFF
        } else {
            &ON
        });
    }
    res
}

pub(crate) fn get_key(code: Keycode) -> i16 {
    match code {
        // These are all the same as their ascii equivalents
        Keycode::Tab
        | Keycode::Escape
        | Keycode::Space
        | Keycode::Exclaim
        | Keycode::Quotedbl
        | Keycode::Hash
        | Keycode::Dollar
        | Keycode::Percent
        | Keycode::Ampersand
        | Keycode::Quote
        | Keycode::LeftParen
        | Keycode::RightParen
        | Keycode::Asterisk
        | Keycode::Plus
        | Keycode::Comma
        | Keycode::Minus
        | Keycode::Period
        | Keycode::Slash
        | Keycode::Num0
        | Keycode::Num1
        | Keycode::Num2
        | Keycode::Num3
        | Keycode::Num4
        | Keycode::Num5
        | Keycode::Num6
        | Keycode::Num7
        | Keycode::Num8
        | Keycode::Num9
        | Keycode::Colon
        | Keycode::Semicolon
        | Keycode::Less
        | Keycode::Equals
        | Keycode::Greater
        | Keycode::Question
        | Keycode::At
        | Keycode::LeftBracket
        | Keycode::Backslash
        | Keycode::RightBracket
        | Keycode::Caret
        | Keycode::Underscore
        | Keycode::Backquote
        | Keycode::A
        | Keycode::B
        | Keycode::C
        | Keycode::D
        | Keycode::E
        | Keycode::F
        | Keycode::G
        | Keycode::H
        | Keycode::I
        | Keycode::J
        | Keycode::K
        | Keycode::L
        | Keycode::M
        | Keycode::N
        | Keycode::O
        | Keycode::P
        | Keycode::Q
        | Keycode::R
        | Keycode::S
        | Keycode::T
        | Keycode::U
        | Keycode::V
        | Keycode::W
        | Keycode::X
        | Keycode::Y
        | Keycode::Z => code as i16,
        Keycode::Return => 128,
        Keycode::Backspace => 129,
        Keycode::Left => 130,
        Keycode::Up => 131,
        Keycode::Right => 132,
        Keycode::Down => 133,
        Keycode::Home => 134,
        Keycode::PageUp => 136,
        Keycode::End => 135,
        Keycode::PageDown => 137,
        Keycode::Insert => 138,
        Keycode::Delete => 139,
        Keycode::F1 => 141,
        Keycode::F2 => 142,
        Keycode::F3 => 143,
        Keycode::F4 => 144,
        Keycode::F5 => 145,
        Keycode::F6 => 146,
        Keycode::F7 => 147,
        Keycode::F8 => 148,
        Keycode::F9 => 149,
        Keycode::F10 => 150,
        Keycode::F11 => 151,
        Keycode::F12 => 152,
        _ => 0,
    }
}

#[cfg(test)]
pub mod tests {
    use sdl2::pixels::PixelFormatEnum;

    use crate::io::get_register;

    #[test]
    fn test_registers() {
        assert_eq!(PixelFormatEnum::RGB24.byte_size_per_pixel(), 3);
        assert_eq!(get_register(0x4000), (0, 0, 16, 1));
        assert_eq!(get_register(0x4001), (16, 0, 16, 1));
        assert_eq!(get_register(0x5FFF), (512 - 16, 255, 16, 1));
    }
}
