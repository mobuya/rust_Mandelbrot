use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pixel {
    // the max of u8 is 255
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

// * * * Image * * *

pub struct Image {
    pub width: usize,
    pub height: usize,
    data: Vec<Pixel>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        let data = vec![Pixel { r: 0, b: 0, g: 0 }; width * height];
        Image {
            width,
            height,
            data,
        }
    }

    // Option: Some and None -> requires unwrap !
    pub fn get(&self, x: usize, y: usize) -> Option<&Pixel> {
        let return_pixel = self.data.get(y * self.width + x);
        match return_pixel {
            Some(return_pixel) => Some(return_pixel),
            None => {
                println!("Pixel at coordinates ({}, {}) is out of range.", x, y);
                None
            }
        }
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        let return_pixel = self.data.get_mut(y * self.width + x);
        match return_pixel {
            Some(return_pixel) => Some(return_pixel),
            None => {
                println!("Pixel at coordinates ({}, {}) is out of range.", x, y);
                None
            }
        }
    }

    pub fn get_mandelbrot_pixels(&self) -> usize {
        // black pixels are inside the set
        self.data
            .iter()
            .filter(|&pixel| *pixel == Pixel { r: 0, g: 0, b: 0 })
            .count()
    }
}
