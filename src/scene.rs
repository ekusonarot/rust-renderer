use image::{ImageBuffer, RgbImage, Rgb};
use rayon::prelude::*;

use crate::vector::Vector3;
use crate::color::Color;
use crate::object::{Object, Plane};
use crate::ray::Ray;

#[derive(Debug)]
pub struct Camera {
    position: Vector3,
    top: Vector3,
    forward: Vector3,
    fov: f32,
    right: Vector3,
    image_size: (u32, u32),
}
impl Camera {
    pub fn new(position: Vector3, top: Vector3, forward: Vector3,
        right: Vector3, fov: f32, image_size: (u32, u32)) -> Camera {
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
pub struct Scene {
    camera: Camera,
    objs: Vec<Object>
}
impl Scene {
    pub fn new(camera: Camera, objs: Vec<Object>) -> Scene {
        Scene{
            camera,
            objs,
        }
    }
    fn tex_calc(&self, vec: &Vector3, plane: &Plane, obj: &Object, tex_name: &str) -> Option<Color> {
        if tex_name.is_empty() {
            return None;
        }
        let image = &obj.image[tex_name];
        let width = image.width() as f32;
        let height = image.height() as f32;
        let vt = match plane.vt.as_ref() {
            Some(vt) => vt,
            None => return None,
        };
        let v1 = vt.v2 - vt.v1;
        let v2 = vt.v3 - vt.v1;
        let v = v1 * vec.y + v2 * vec.z;
        let mut x = if 0. <= v.x {
            (v.x * height) as u32
        } else {
            ((1.+v.x) * height) as u32
        };
        let mut y =  if 0. <= v.y {
            (v.y * width) as u32
        } else {
            ((1.+v.y) * width) as u32
        };
        if width as u32 <= x {
            x = width as u32 - 1;
        }
        if height as u32 <= y {
            y = height as u32 - 1;
        }
        let pixel = image.get_pixel(x, y);
        Some(Color::from_pixel(pixel))
    }
    fn calc(&self, ray: &Ray, num_of_bounce: usize, sampling: usize, num_of_diffuse: usize) -> (Color, f32) {
        let num_of_bounce = if 0 < num_of_bounce {
            num_of_bounce - 1
        } else {
            return (Color::zeros(), 1.);
        };
        let (intersect, plane, obj) = match Object::crossjudge(&self.objs, ray) {
            Some((intersect, plane, obj)) => (intersect, plane, obj),
            None => return (Color::zeros(), 1.),
        };
        let material_id = match plane.material_id {
            Some(material_id) => material_id,
            None => return (Color::zeros(), 1.),
        };
        let material = &obj.materials[material_id];
        let mut diffuse = Color::ones();
        if !material.diffuse_texture.is_empty() {
            if let Some(map_kd) = self.tex_calc(&intersect, &plane, &obj, &material.diffuse_texture) {
                diffuse = map_kd;
            }
        } else {
            diffuse = Color::from_list(material.diffuse);
        }
        let mut pr = 0.; //ラフネス
        if let Some(map_pr) = material.unknown_param.get("map_Pr") {
            if let Some(pr_color) = self.tex_calc(&intersect, &plane, &obj, map_pr) {
                pr = pr_color.r/255.;
            }
        }
        if let Some(pr_string) = material.unknown_param.get("Pr") {
            pr = pr_string.parse::<f32>().unwrap();
        }
        let mut pm = 0.; //メタリック
        if let Some(map_pm) = material.unknown_param.get("map_Pm") {
            if let Some(pm_color) = self.tex_calc(&intersect, &plane, &obj, map_pm) {
                pm = pm_color.r/255.;
            }
        }
        if let Some(pm_string) = material.unknown_param.get("Pm") {
            pm = pm_string.parse::<f32>().unwrap();
        }
        let mut tr: f32 = 0.; //透明度
        if let Some(tr_string) = material.unknown_param.get("Tr") {
            tr = match tr_string.parse::<f32>() {
                Ok(tr) => tr,
                Err(_) => 0.,
            };
        }
        if let Some(d_string) = material.unknown_param.get("d") {
            tr = match d_string.parse::<f32>() {
                Ok(d) => 1.-d,
                Err(_) => 0.,
            }
        }
        let ni = material.optical_density; //屈折率
        
        let mut ec = Color::zeros(); //発光
        if let Some(ec_string) = material.unknown_param.get("Ec") {
            ec = Color::from_vector(&ec_string.split(" ").filter_map(|s| s.parse::<f32>().ok()).collect::<Vec<_>>());
        }

        let new_origin = ray.direction*intersect.x+ray.origin;
        let norm = match &plane.vn {
            Some(vn) => vn.v1,
            None => Vector3::new(1.,0.,0.),
        };
        let mut tr_color = Color::zeros();
        if 0. < tr {
            let tr_ray = Ray::new(new_origin, ray.direction.refraction(&norm, ni));
            let (ttr_color, d) = self.calc(&tr_ray, num_of_bounce, sampling, num_of_diffuse);
            tr_color=ttr_color/d;
        }
        let mut pm_color = Color::zeros();
        let mut diffuse_color = Color::zeros();
        if 0 < num_of_diffuse {
            if 0. < pm {
                let rays = Ray::rnd_dirgen(&new_origin, &norm, &ray.direction.reflection(&norm), sampling, pr);
                let mut colors = vec![Color::zeros(); sampling as usize];
                rays.par_iter().zip(colors.par_iter_mut()).for_each(|(ray, color)| {
                    let (t_color, d) = self.calc(ray, num_of_bounce, sampling, num_of_diffuse-1);
                    *color = t_color/d;
                });
                pm_color = colors.iter().fold(Color{r:0.,g:0.,b:0.}, |acc, x| acc+*x)/sampling as f32;
            }

            let nee_rays = self.next_event_estimation(&new_origin, &norm);
            let mut colors = vec![Color::zeros(); nee_rays.len()];
            nee_rays.par_iter().zip(colors.par_iter_mut()).for_each(|(ray, color)| {
                let (t_color, d) = self.calc(ray, num_of_bounce, sampling, 0);
                *color = t_color/d;
            });
            let nee_color = colors.iter().fold(Color{r:0.,g:0.,b:0.}, |acc, x| acc+*x);

            let rays = Ray::rnd_dirgen(&new_origin, &norm, &norm, sampling-nee_rays.len(), 1.);
            let mut colors = vec![Color::zeros(); sampling-nee_rays.len() as usize];
            rays.par_iter().zip(colors.par_iter_mut()).for_each(|(ray, color)| {
                let (t_color, d) = self.calc(ray, num_of_bounce, sampling, num_of_diffuse-1);
                *color = t_color/d;
            });
            diffuse_color = (colors.iter().fold(Color{r:0.,g:0.,b:0.}, |acc, x| acc+*x)+nee_color)/sampling as f32;
        }

        (diffuse*diffuse_color*(1.-pm)*(1.-tr)/2. + tr_color*tr*(1.-pm) + pm_color*pm + ec, intersect.x)
    }
    fn next_event_estimation(&self, origin: &Vector3, norm: &Vector3) -> Vec<Ray> {
        let mut rays = Vec::new();
        self.objs.iter().for_each(|obj| {
            if !obj.is_ec {
                return;
            }
            if 0. < norm.inner(&(obj.origin-*origin)) {
                rays.push(Ray::new(*origin,obj.origin-*origin));
            }
        });
        rays
    }
    pub fn render(&self, num_of_bounce: usize, sampling: usize, num_of_diffuse: usize) {
        let (width, height) = self.camera.image_size;
        let mut img: RgbImage = ImageBuffer::new(width, height);
        let fov = self.camera.fov/180. * std::f32::consts::PI;
        let position = self.camera.position;
        let forward = self.camera.forward;
        let top = self.camera.top;
        let right = self.camera.right;
        img.enumerate_pixels_mut()
            .collect::<Vec<(u32, u32, &mut Rgb<u8>)>>()
            .par_iter_mut()
            .for_each(|(w, h, pixel)|{
                let mut w = (*w) as f32;
                let mut h = (*h) as f32;
                let height = height as f32;
                let width = width as f32;
                w = (w - width/2.)/(width/2.);
                h = (h - height/2.)/(width/2.);
                let direction = forward/(fov/2.).tan()+right*w-top*h;
                let ray = Ray::new(position,direction);
                let (color, _) = self.calc(&ray, num_of_bounce, sampling, num_of_diffuse);
                let color = color.reform();
                pixel[0] = color.r as u8;
                pixel[1] = color.g as u8;
                pixel[2] = color.b as u8;
            });
        img.save("save.png").unwrap();
    }
}
#[cfg(test)]
mod tests {
    use crate::vector::Vector3;
    use crate::scene::Scene;
    #[test]
    fn it_works() {
        let origin = Vector3::new(0.,0.,0.);
        let norm = Vector3::new(1.,0.,0.);
        let rays = Ray::rnd_dirgen(&origin, &norm, 100, false);
        for ray in rays {
            if norm.inner(&ray.direction) < 0. {
                panic!("STOP");
            }
        }
    }
    #[test]
    fn refract() {
        let norm = Vector3::new(1.,0.,0.);
        let vec = Vector3::new(1.,1.,1.);
        panic!("{:?}",vec.refraction(&norm, 1.2));
    }
    #[test]
    fn reflect() {
        let norm = Vector3::new(1.,1.,1.);
        let vec = Vector3::new(1.,1.,1.);
        panic!("{:?}", vec.reflection(&norm));
    }
}