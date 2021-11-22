use std::ops::{Add, Sub, Mul, Div};
use image::Rgb;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
impl Color {
    #[allow(dead_code)]
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color {
            r,
            g,
            b,
        }
    }
    pub fn from_pixel(pixel: &Rgb<u8>) -> Color {
        Color {
            r: pixel[0] as f32,
            g: pixel[1] as f32,
            b: pixel[2] as f32,
        }
    }
    pub fn from_vector(v: &Vec<f32>) -> Color {
        assert_eq!(v.len(),3);
        Color {
            r: v[0],
            g: v[1],
            b: v[2],
        }
    }
    pub fn from_list(l: [f32; 3]) -> Color {
        assert_eq!(l.len(),3);
        Color {
            r: l[0],
            g: l[1],
            b: l[2],
        }
    }
    pub fn reform(&self) -> Color {
        let r = if 255. < self.r {
            255.
        } else if self.r < 0. {
            0.
        } else {
            self.r
        };
        let g = if 255. < self.g {
            255.
        } else if self.g < 0. {
            0.
        } else {
            self.g
        };
        let b = if 255. < self.b {
            255.
        } else if self.b < 0. {
            0.
        } else {
            self.b
        };
        Color{r,g,b}
    }
    pub fn zeros() -> Color {
        Color{
            r: 0.,
            g: 0.,
            b: 0.,
        }
    }
    pub fn ones() -> Color {
        Color{
            r: 1.,
            g: 1.,
            b: 1.,
        }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r+other.r,
            g: self.g+other.g,
            b: self.b+other.b,
        }
    }
}
impl Add<f32> for Color {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        Self {
            r: self.r+other,
            g: self.g+other,
            b: self.b+other,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            r: self.r-other.r,
            g: self.g-other.g,
            b: self.b-other.b,
        }
    }
}
impl Sub<f32> for Color {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        Self {
            r: self.r-other,
            g: self.g-other,
            b: self.b-other,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}
impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl Div for Color {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            r: self.r / other.r,
            g: self.g / other.g,
            b: self.b / other.b,
        }
    }
}
impl Div<f32> for Color {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other,
        }
    }
}