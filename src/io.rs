use sdl2::{
    keyboard::{KeyboardState, Keycode as K, Scancode},
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::{Canvas, Texture},
    video::Window, surface::Surface,
};

type Pixel = [u8; 3];

const ON: Pixel = [0, 0, 0];
const OFF: Pixel = [255, 255, 255];

pub struct Screen {
    changed: bool,
    pub(crate) data: [u8; 512 * 256 * 3],
    //canvas: Canvas<Window>,
    //texture: Texture<'a>,
}
impl Screen {
    pub fn new() -> Self {
        Self { changed: true, data: [255; 512 * 256 * 3]}
    }

    pub fn update(&mut self, addr: usize, value: i16) {
        let addr = addr * 3 * 16;
        let register = &mut self.data[addr..addr + 48];
        register.copy_from_slice(&as_pixels(value));
        self.changed = true;
    }

    pub const fn changed(&self) -> bool {
        self.changed
    }

    pub fn refresh(&mut self, texture: &mut Texture) {
        texture.update(None, &self.data, 3 << 9).unwrap();
        self.changed = false;
    }
}

const fn get_register(addr: i16) -> (i32, i32, u32, u32) {
    let addr = addr as i32 - 0x4000;
    let row = addr >> 5;
    let col = (addr & 31) << 4;
    (col, row, 16, 1)
}

pub fn as_pixels(value: i16) -> [u8; 48] {
    match value {
        0 => [255; 48],
        -1 => [0; 48],
        _ => {
            let mut res: [u8; 48] = [0; 48];
            for (i, p) in res.chunks_mut(3).enumerate() {
                p.copy_from_slice(if (value >> i) & 1 == 1 { &ON } else { &OFF });
            }
            // println!("{res:?}");
            res
        }
    }
        
}

pub(crate) fn get_key(kbd: KeyboardState) -> i16 {
    let mut pressed = kbd.pressed_scancodes().filter_map(K::from_scancode);
    match pressed.next() {
        Some(k) => {
            if kbd.is_scancode_pressed(Scancode::from_keycode(K::LShift).unwrap())
                || kbd.is_scancode_pressed(Scancode::from_keycode(K::RShift).unwrap())
            {
                //pressed.any(|k| k == K::LShift || k == K::RShift) {
                match k {
                    k if (k as u8 as char).is_ascii_alphabetic() => return (k as i16) - 32,
                    K::Num0 => return K::RightParen as i16,
                    K::Num1 => return K::Exclaim as i16,
                    K::Num2 => return K::At as i16,
                    K::Num3 => return K::Hash as i16,
                    K::Num4 => return K::Dollar as i16,
                    K::Num5 => return K::Percent as i16,
                    K::Num6 => return K::Caret as i16,
                    K::Num7 => return K::Ampersand as i16,
                    K::Num8 => return K::Asterisk as i16,
                    K::Num9 => return K::LeftParen as i16,
                    K::Equals => return K::Plus as i16,
                    K::LeftBracket => return 123,
                    K::Backslash => return 124,
                    K::RightBracket => return 125,
                    K::Backquote => return 126,
                    K::Comma => return K::Less as i16,
                    K::Period => return K::Greater as i16,
                    K::Slash => return K::Question as i16,
                    K::Minus => return K::Underscore as i16,
                    _ => {}
                }
            }

            match k {
                // These are all the same as their ascii equivalents
                K::Tab
                | K::Escape
                | K::Space
                | K::Exclaim
                | K::Quotedbl
                | K::Hash
                | K::Dollar
                | K::Percent
                | K::Ampersand
                | K::Quote
                | K::LeftParen
                | K::RightParen
                | K::Asterisk
                | K::Plus
                | K::Comma
                | K::Minus
                | K::Period
                | K::Slash
                | K::Num0
                | K::Num1
                | K::Num2
                | K::Num3
                | K::Num4
                | K::Num5
                | K::Num6
                | K::Num7
                | K::Num8
                | K::Num9
                | K::Colon
                | K::Semicolon
                | K::Less
                | K::Equals
                | K::Greater
                | K::Question
                | K::At
                | K::LeftBracket
                | K::Backslash
                | K::RightBracket
                | K::Caret
                | K::Underscore
                | K::Backquote
                | K::A
                | K::B
                | K::C
                | K::D
                | K::E
                | K::F
                | K::G
                | K::H
                | K::I
                | K::J
                | K::K
                | K::L
                | K::M
                | K::N
                | K::O
                | K::P
                | K::Q
                | K::R
                | K::S
                | K::T
                | K::U
                | K::V
                | K::W
                | K::X
                | K::Y
                | K::Z => k as i16,
                K::KpLeftBrace => 123,
                K::KpVerticalBar => 124,
                K::KpRightBrace => 125,
                K::Return => 128,
                K::Backspace => 129,
                K::Left => 130,
                K::Up => 131,
                K::Right => 132,
                K::Down => 133,
                K::Home => 134,
                K::PageUp => 136,
                K::End => 135,
                K::PageDown => 137,
                K::Insert => 138,
                K::Delete => 139,
                K::F1 => 141,
                K::F2 => 142,
                K::F3 => 143,
                K::F4 => 144,
                K::F5 => 145,
                K::F6 => 146,
                K::F7 => 147,
                K::F8 => 148,
                K::F9 => 149,
                K::F10 => 150,
                K::F11 => 151,
                K::F12 => 152,
                _ => 0,
            }
        }
        None => 0,
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
