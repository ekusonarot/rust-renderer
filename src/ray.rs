use crate::vector::Vector3;
use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}
impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray{
            origin,
            direction: direction.normalize(),
        }
    }
    pub fn rnd_dirgen(origin: &Vector3, norm: &Vector3, num: usize, transparent: bool) -> Vec<Ray> {
        let mut rays = Vec::new();
        for _ in 0..num {
            let s: f32 = rand::thread_rng().gen_range(0.,std::f32::consts::PI);
            let p: f32 = rand::thread_rng().gen_range(0.,std::f32::consts::PI);
            let x = s.sin()*p.cos();
            let y = s.sin()*p.sin();
            let z = s.cos();
            let mut vec = Vector3::new(x,y,z);
            if transparent {
                rays.push(Ray::new(*origin,vec));
            } else if 0. < vec.inner(norm) {
                vec = vec + *norm/(89. as f32).tan();
                rays.push(Ray::new(*origin,vec));
            } else {
                vec = vec * -1.;
                vec = vec + *norm/(89. as f32).tan();
                rays.push(Ray::new(*origin,vec));
            }
        }
        rays
    }
}
