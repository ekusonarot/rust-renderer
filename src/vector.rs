use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[allow(dead_code)]
impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 {
            x,
            y,
            z,
        }
    }
    pub fn normalize(&self) -> Vector3 {
        let denom = (self.x*self.x+self.y*self.y+self.z*self.z).sqrt();
        *self/denom
    }
    pub fn refraction(&self, norm: &Vector3, optical_density: f32) -> Vector3 {
        let mut norm = *norm*-1.;
        let mut ni = 1.;
        let mut no = optical_density;
        if self.inner(&norm) < 0.{
            ni = optical_density;
            no = 1.;
            norm = norm*-1.;
        }
        
        *self + norm*(no*no-ni*ni+(self.inner(&norm).powf(2.)).sqrt()-self.inner(&norm))
    }
    pub fn reflection(&self, norm: &Vector3) -> Vector3 {
        *self+*norm*2.*(-(*self).inner(norm))
    }
    #[allow(dead_code)]
    pub fn rotate(&self, r: &Vector3) -> Vector3 {
        let mut r = (*r).clone();
        r = r/180.*std::f32::consts::PI;
        let x = self.x*(r.x.cos()*r.y.cos()*r.z.cos()-r.x.sin()*r.z.sin()) 
            + self.y*(-r.x.cos()*r.y.cos()*r.z.sin()+r.x.sin()*r.z.cos()) 
            + self.z*r.x.cos()*r.y.sin();
        let y = self.x*(r.x.sin()*r.y.cos()*r.z.cos()+r.x.cos()*r.z.sin())
            + self.y*(-r.x.sin()*r.y.cos()*r.z.sin()+r.x.cos()*r.z.cos())
            + self.z*r.x.sin()*r.y.sin();
        let z = self.x*(-r.y.sin()*r.z.cos())
            + self.y*(r.y.sin()*r.z.sin())
            + self.z*r.y.cos();
        Vector3::new(x,y,z)
    }
    
    #[allow(dead_code)]
    pub fn cross(&self, b: &Vector3) -> Vector3 {
        Vector3{
            x: self.y*b.z - self.z*b.y,
            y: self.z*b.x - self.x*b.z,
            z: self.x*b.y - self.y*b.x,
        }
    }
    
    #[allow(dead_code)]
    pub fn inner(&self, b: &Vector3) -> f32 {
        self.x*b.x + self.y*b.y + self.z*b.z
    }
    
    #[allow(dead_code)]
    pub fn scale(&self, v: &Vector3) -> Vector3 {
        let x = self.x*v.x;
        let y = self.y*v.y;
        let z = self.z*v.z;
        Vector3::new(x,y,z)
    }
    
    #[allow(dead_code)]
    pub fn translate(&self, t:& Vector3) -> Vector3 {
        let x = self.x+t.x;
        let y = self.y+t.y;
        let z = self.z+t.z;
        Vector3::new(x,y,z)
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Add<f32> for Vector3 {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Sub<f32> for Vector3 {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div for Vector3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}
impl Div<f32> for Vector3 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
#[allow(dead_code)]
impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 {
            x,
            y,
        }
    }
    pub fn normalize(&self) -> Vector2 {
        let denom = (self.x*self.x+self.y*self.y).sqrt();
        *self/denom
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Add<f32> for Vector2 {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Sub<f32> for Vector2 {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

impl Mul for Vector2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}
impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div for Vector2 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}
impl Div<f32> for Vector2 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}
