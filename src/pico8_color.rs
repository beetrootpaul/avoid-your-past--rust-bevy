// use bevy::prelude::Color;

#[allow(dead_code)]
pub enum Pico8Color {
    Black,
    DarkBlue,
    DarkPurple,
    DarkGreen,
    Brown,
    DarkGrey,
    LightGrey,
    White,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Levender,
    Pink,
    LightPeach,
}

impl Pico8Color {
    // pub fn as_bevy_color(&self) -> Color {
    //     Color::hex(self.hex()).expect("should convert from hex")
    // }

    fn hex(&self) -> &str {
        match *self {
            // Hex value here are taken from https://pico-8.fandom.com/wiki/Palette#The_system_palette
            Pico8Color::Black => "000000",
            Pico8Color::DarkBlue => "1d2b53",
            Pico8Color::DarkPurple => "7e2553",
            Pico8Color::DarkGreen => "008751",
            Pico8Color::Brown => "ab5236",
            Pico8Color::DarkGrey => "5f574f",
            Pico8Color::LightGrey => "c2c3c7",
            Pico8Color::White => "fff1e8",
            Pico8Color::Red => "ff004d",
            Pico8Color::Orange => "ffa300",
            Pico8Color::Yellow => "ffec27",
            Pico8Color::Green => "00e436",
            Pico8Color::Blue => "29adff",
            Pico8Color::Levender => "83769c",
            Pico8Color::Pink => "ff77a8",
            Pico8Color::LightPeach => "ffccaa",
        }
    }
}
