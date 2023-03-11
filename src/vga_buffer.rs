use volatile::Volatile;

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


//Declaring a custom type in rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

//Adding an action for our ColorCode type
impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode( (background as u8) << 4 | (foreground as u8))
    }
 }

 //C style struct
 #[derive(Debug, Clone, Copy, PartialEq, Eq)]
 #[repr(C)]
 struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
 }

 const BUFFER_HEIGHT: usize = 25;
 const BUFFER_WIDTH: usize = 80;

 //repr(transparent) makes the single field struct
 //to be layed out in memory in the same way as its 
 //internal type. Meaning we can transmute between 
 //the field and the struct seamesly.
 #[repr(transparent)]
 struct Buffer {
    //The array type is written as [T; N]
    //1.
    //[ScreenChar; BUFFER_WIDTH] -> Array of ScreenChars
    //(lets say a row)
    // -> (ScreenChar0, ScreenChar1, ...ScreenCharN)
    //2.
    // [[ScreenChar;BUFFER_WIDTH]; N], Array size N of rows ->
    // [(ScreenChar0, ScreenChar1, ...ScreenCharN)0,
    // (ScreenChar0, ScreenChar1, ...ScreenCharN)1,
    // (ScreenChar0, ScreenChar1, ...ScreenCharN)N]

    //Volatile<T> tells the compiler that the write 
    //has side effects and should not be optimized away.
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
 }

 pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
 }

 impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte { 
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                
                let row = BUFFER_HEIGHT  - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write( ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }

        }

    }

    fn new_line(&mut self){ 
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row -1][col].write(character);
            }
        }
        self.clean_row(BUFFER_HEIGHT - 1); //Clean bottom line
        self.column_position = 0;
    }

    fn clean_row(&mut self, row: usize){
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(
                ScreenChar {
                    ascii_character: b' ',
                    color_code: self.color_code,
                }
            )
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe)
            }
        }
    
    }
 }

//  pub fn print_someting() {
//         use core::fmt::Write;
//         let mut writer = Writer {
//             column_position: 0,
//             color_code: ColorCode::new(Color::Yellow, Color::Black),
//             /* 1. Cast the integer 0xb8000 as a mutable raw pointer
//                 (0xb8000 as *mut Buffer) now the memory at 0xb8000 
//                 has the shape of the struct Buffer.
            
//             2.hen we convert it to a mutable reference by dereferencing it
//             *(0xb8000 as *mut Buffer)
            
//             3. borrowing it again (through &mut)
//                 &mut *(0xb8000 as *mut Buffer)

//             In the end  Writer.buffer points to a raw memory area
//             with the shape of struct Buffer that we can manipulate
//             to print stuff.
//             */
//             buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
//         };
//         writer.write_string("Hellö öööRLD! ö ");
//         write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
//     }

use core::fmt;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! { 
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer) },
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
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}