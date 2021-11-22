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
    pub fn rnd_dirgen(origin: &Vector3, norm: &Vector3, dir: &Vector3, num: usize, roughness: f32) -> Vec<Ray> {
        let mut rays = Vec::new();
        let norm = norm.normalize();
        let dir = dir.normalize();
        for _ in 0..num {
            let s: f32 = rand::thread_rng().gen_range(0.,std::f32::consts::PI);
            let p: f32 = rand::thread_rng().gen_range(0.,std::f32::consts::PI);
            let x = s.sin()*p.cos();
            let y = s.sin()*p.sin();
            let z = s.cos();
            let mut vec = Vector3::new(x,y,z);
            let angle = 89.9 * roughness;
            if 0. < vec.inner(&norm) {
                vec = vec + dir/(angle.tan()+f32::EPSILON);
                rays.push(Ray::new(*origin,vec));
            } else {
                vec = vec * -1.;
                vec = vec + dir/(angle.tan()+f32::EPSILON);
                rays.push(Ray::new(*origin,vec));
            }
        }
        rays
    }
}
