#![allow(unused)]

use std::ops::{Deref, DerefMut};

use rand::Rng;

pub struct Colors;

impl Colors {
    pub const BLACK: u32 = 0;
    pub const WHITE: u32 = 16777215;
    pub const GREEN: u32 = 65280;
    pub const RED: u32 = 16711680;
    pub const BLUE: u32 = 255;

    pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }

    pub fn random() -> u32 {
        rand::random()
    }
}