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
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
 }

 pub struct Writter {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
 }

 impl Writter {
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
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                self.column_position += 1;
            }

        }

    }

    fn new_line(&mut self){ 
        self.column_position = 0;
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