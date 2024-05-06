#![allow(unused)]
use crate::Colors;
use std::ops::{Deref, DerefMut, Index, IndexMut};

pub struct Buffer {
    buf: Vec<u32>,
    width: usize,
    height: usize,
    color: u32,
}

impl Deref for Buffer {
    type Target = [u32];

    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buf
    }
}

impl Index<(usize, usize)> for Buffer {
    type Output = u32;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        assert!(
            x < self.width && y < self.height,
            "({x}, {y}) is out of bounds"
        );
        &self.buf[(y * self.width) + x]
    }
}

impl IndexMut<(usize, usize)> for Buffer {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        assert!(
            x < self.width && y < self.height,
            "({x}, {y}) is out of bounds"
        );
        &mut self.buf[(y * self.width) + x]
    }
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buf: vec![0; width * height],
            width,
            height,
            color: Colors::WHITE,
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn color(&self) -> u32 {
        self.color
    }

    pub fn set_color(&mut self, color: u32) {
        self.color = color;
    }

    pub fn clear(&mut self) {
        self.iter_mut().for_each(|x| *x = 0);
    }

    pub fn fill(&mut self) {
        let color = self.color;
        self.iter_mut().for_each(|x| *x = color);
    }

    pub fn pixel(&mut self, index: (usize, usize)) {
        self[index] = self.color;
    }

    pub fn line(&mut self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) {
        let (x1, y1, x2, y2) = (x1 as i32, y1 as i32, x2 as i32, y2 as i32);
        let dx = (x2 - x1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let dy = -(y2 - y1).abs();
        let sy = if y1 < y2 { 1 } else { -1 };
        let (mut x, mut y) = (x1, y1);
        let mut error = dx + dy;
        loop {
            self.pixel((x as usize, y as usize));
            if x == x2 && y == y2 {
                break;
            }
            let e2 = 2 * error;
            if e2 >= dy {
                if x == x2 {
                    break;
                }
                error += dy;
                x += sx;
            }
            if e2 <= dx {
                if y == y2 {
                    break;
                }
                error += dx;
                y += sy;
            }
        }
    }

    pub fn tri(&mut self, p1: (usize, usize), p2: (usize, usize), p3: (usize, usize)) {
        self.line(p1, p2);
        self.line(p2, p3);
        self.line(p3, p1);
    }

    pub fn sqr(&mut self, p1: (usize, usize), p2: (usize, usize), p3: (usize, usize), p4: (usize, usize)) {
        self.line(p1, p2);
        self.line(p2, p3);
        self.line(p3, p4);
        self.line(p4, p1);
    }
}