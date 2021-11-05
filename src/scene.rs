use image::{ImageBuffer, RgbImage};
use rayon::prelude::*;
#[derive(Debug)]
pub struct Camera {
    position: Vec<f32>,
    top: Vec<f32>,
    forward: Vec<f32>,
    right: Vec<f32>,
    fov: f32,
    image_size: (u32, u32),
}
impl Camera {
    pub fn new(position: Vec<f32>, top: Vec<f32>, forward: Vec<f32>,
        right: Vec<f32>, fov: f32, image_size: (u32, u32)) -> Camera {
        assert_eq!(forward.len(), 3);
        assert_eq!(top.len(), 3);
        assert_eq!(position.len(), 3);
        Camera{
            position,
            top,
            forward,
            right,
            fov,
            image_size,
        }
    }
}
#[derive(Debug)]
struct Ray {
    origin: Vec<f32>,
    direction: Vec<f32>,
}
impl Ray {
    pub fn new(origin: Vec<f32>, direction: Vec<f32>) -> Ray {
        Ray{
            origin,
            direction
        }
    }
}
#[derive(Debug)]
pub struct Plane {
    v1: Vec<f32>,
    v2: Vec<f32>,
    v3: Vec<f32>,
}
impl Plane {
    pub fn new(v1: Vec<f32>, v2: Vec<f32>, v3: Vec<f32>) -> Plane {
        Plane{
            v1,
            v2,
            v3,
        }
    }
}
#[derive(Debug)]
pub struct Scene {
    camera: Camera,
    planes: Vec<Plane>
}
impl Scene {
    pub fn new(camera: Camera, planes: Vec<Plane>) -> Scene {
        Scene{
            camera,
            planes,
        }
    }
    pub fn render(&self) {
        let (width, height) = self.camera.image_size;
        let mut img: RgbImage = ImageBuffer::new(width, height);
        let fov = self.camera.fov;
        let position = self.camera.position;
        let forward = self.camera.forward;
        let top = self.camera.top;
        let right = self.camera.right;
        img.enumerate_pixels_mut()
            .collect::<Vec<(u32, u32, &mut image::Rgb<u8>)>>()
            .par_iter_mut()
            .for_each(|(x, y, pixel)|{
                let x = (*x - height/2) as f32/(width/2) as f32;
                let y = (*y - width/2) as f32/(width/2) as f32;
                let direction = forward/(fov/2.).tan()+right*y-top*x;
                let ray = Ray::new(position,direction);
                pixel[0] = 0;
                pixel[1] = 255;
                pixel[2] = 0;
            });
        img.save("save.png").unwrap();
    }
}
