
use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static!{
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            0 => Color::Black,
            1 => Color::Blue,
            2 => Color::Green,
            3 => Color::Cyan,
            4 => Color::Red,
            5 => Color::Magenta,
            6 => Color::Brown,
            7 => Color::LightGray,
            8 => Color::DarkGray,
            9 => Color::LightBlue,
            10 => Color::LightGreen,
            11 => Color::LightCyan,
            12 => Color::LightRed,
            13 => Color::Pink,
            14 => Color::Yellow,
            15 => Color::White,
            _ => Color::Black
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }

    fn change_color(&mut self, foreground: Color, background: Color) {
        self.0 = (background as u8) << 4 | (foreground as u8);
    }

    fn get_background(&self) -> Color {
        Color::from(self.0 >> 4 & 0x0F)
    }

    fn get_foreground(&self) -> Color {
        Color::from(self.0 & 0x0F)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    character: u8,
    color_code: ColorCode
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT]
}


/* WRITER */

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
    escape_bit: bool
}

impl Writer {
    pub fn new() -> Writer {
        Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
            escape_bit: false
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        if self.escape_bit {
            match byte {
                00..=15 => self.color_code = ColorCode::new(Color::White, Color::Black),
                16..=31 => self.color_code.change_color(
                    Color::from(byte % 16),
                    self.color_code.get_background()
                ),
                32..=47 => self.color_code.change_color(
                    self.color_code.get_foreground(),
                    Color::from(byte % 16)
                ),
                b'm' => self.escape_bit = false,
                _ => {}
            }
        }
        else {
            match byte {
                b'\n' => self.new_line(),
                0x08 => self.backspace(),
                0x1B => self.escape_bit = true,
                byte => {
                    if self.column_position >= BUFFER_WIDTH {
                        self.new_line();
                    }
    
                    let row = BUFFER_HEIGHT - 1;
                    let col = self.column_position;
    
                    let color_code = self.color_code;
                    self.buffer.chars[row][col] = ScreenChar {
                        character: byte,
                        color_code
                    };
                    self.column_position += 1;
                }
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' | 0x08 => self.write_byte(byte),
                _ => self.write_byte(0xfe)
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col];
                self.buffer.chars[row - 1][col] = character;
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn backspace(&mut self) {
        if self.column_position > 0 {
            self.column_position -= 1;
            self.buffer.chars[BUFFER_HEIGHT - 1][self.column_position] =
                ScreenChar { character: b' ', color_code: self.color_code };
        }
    }

    fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col] = ScreenChar {
                character: b' ',
                color_code: self.color_code
            };
        }
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}


/* PRINT MACROS/FUNCS */

pub fn print_change_color(foreground: Color, background: Color) {
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        let mut lock = WRITER.lock();
        lock.write_byte(0x1B);
        lock.write_byte(foreground as u8 + 16);
        lock.write_byte(background as u8 + 32);
        lock.write_byte(b'm');
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
#[allow(unused_imports)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

/* TESTS */

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}


#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    use x86_64::instructions::interrupts;

    let s = "Some test string that fits on a single line";
    
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln! failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i];
            assert_eq!(char::from(screen_char.character), c);
        }
    });
}

#[test_case]
fn test_println_color() {
    serial_print!("\n");
    let mut i = 0;
    while i < 16 {
        let mut j = 0;
        while j < 16 {
            print_change_color(Color::from(i as u8), Color::from(j as u8));
            print!("A");
            print_change_color(Color::White, Color::Black);
            print!(" ");
            j += 1;
        }
        print!("\n");
        i += 1;
    }

    let lock = WRITER.lock();

    let mut i = 0;
    while i < 16 {
        let mut j = 0;
        while j < 16 {
            serial_print!("  -> Test {}, {}\t", i, j);
            assert_eq!(
                lock.buffer.chars[BUFFER_HEIGHT - (17 - i)][j * 2].color_code.0,
                (i as u8) | ((j as u8) << 4)
            );
            serial_println!("[ok]");
            j += 1;
        }
        i += 1;
    }

    serial_print!("-> test_println_color: \t");
}