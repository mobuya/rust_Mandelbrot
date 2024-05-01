use std::ops::Add;
use std::ops::Mul;

#[derive(Copy, Clone)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}
impl Mul for Complex {
    type Output = Self;
    // (a + bi)*(c + di) = (ac - bd) + (ad + bc)i
    fn mul(self, other: Complex) -> Complex {
        Complex {
            re: (self.re * other.re) - (self.im * other.im),
            im: (self.re * other.im) + (self.im * other.re),
        }
    }
}
impl Complex {
    pub fn mag(&self) -> f64 {
        //(a + bi) = (a^2 + b^2)
        (self.im * self.im) + (self.re * self.re)
    }
}
