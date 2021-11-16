use image::{ImageBuffer, RgbImage, Rgb};
use rayon::prelude::*;

use crate::vector::Vector3;
use crate::color::Color;
use crate::object::{Object, Plane};
use crate::ray::Ray;

const RAY_NUM: usize = 20;

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
    fn calc(&self, ray: &Ray, sampling: usize) -> Color {
        let sampling = if 0 < sampling {
            sampling - 1
        } else {
            return Color::new(0.,0.,0.);
        };
        let (vec, plane, obj) = match Object::crossjudge(&self.objs, ray) {
            Some((vec, plane, obj)) => (vec, plane, obj),
            None => return Color::new(0.,0.,0.),
        };
        let material_id = match plane.material_id {
            Some(material_id) => material_id,
            None => return Color::new(0.,0.,0.),
        };
        let material = &obj.materials[material_id];
        let mut pr = Color::new(0.,0.,0.); //ラフネス
        if let Some(map_pr) = material.unknown_param.get("map_Pr") {
            if let Some(pr_color) = self.tex_calc(&vec, &plane, &obj, map_pr) {
                pr = pr_color/255.;
            }
        }
        if let Some(pr_string) = material.unknown_param.get("Pr") {
            pr = Color::from_vector(&pr_string.split(" ").filter_map(|s| s.parse::<f32>().ok()).collect::<Vec<_>>());
        }
        let mut pm = Color::new(0.,0.,0.); //メタリック
        if let Some(map_pm) = material.unknown_param.get("map_Pm") {
            if let Some(pm_color) = self.tex_calc(&vec, &plane, &obj, map_pm) {
                pm = pm_color/255.;
            }
        }
        if let Some(pm_string) = material.unknown_param.get("Pm") {
            pm = Color::from_vector(&pm_string.split(" ").filter_map(|s| s.parse::<f32>().ok()).collect::<Vec<_>>());
        }
        let mut ps = Color::new(0.,0.,0.); //Sheen
        if let Some(map_ps) = material.unknown_param.get("map_Ps") {
            if let Some(ps_color) = self.tex_calc(&vec, &plane, &obj, map_ps) {
                ps = ps_color/255.;
            }
        }
        if let Some(ps_string) = material.unknown_param.get("Ps") {
            ps = Color::from_vector(&ps_string.split(" ").filter_map(|s| s.parse::<f32>().ok()).collect::<Vec<_>>());
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
        
        let mut ec = Color::new(0.,0.,0.); //発光色
        if let Some(ec_string) = material.unknown_param.get("Ec") {
            ec = Color::from_vector(&ec_string.split(" ").filter_map(|s| s.parse::<f32>().ok()).collect::<Vec<_>>());
        }

        let new_origin = ray.direction*vec.x+ray.origin;
        let norm = match &plane.vn {
            Some(vn) => vn.v1,
            None => Vector3::new(1.,0.,0.),
        };
        let rays = Ray::rnd_dirgen(&new_origin, &norm, RAY_NUM, 0. < tr);
        let mut colors: [Color; RAY_NUM] = [Color{r:0.,g:0.,b:0.}; RAY_NUM];
        rays.par_iter().zip(colors.par_iter_mut()).for_each(|(ray, color)| {
            *color = self.calc(ray,sampling);
        });
        let mut tr_color = Color::new(0.,0.,0.);
        if 0. < tr {
            let tr_ray = Ray::new(new_origin, ray.direction.refraction(&norm, ni));
            tr_color = self.calc(&tr_ray, sampling);
        }
        let mut pm_color = Color::new(0.,0.,0.);
        if (Color{r:0.,g:0.,b:0.}) < pm {
            let pm_ray = Ray::new(new_origin, ray.direction.reflection(&norm));
            pm_color = self.calc(&pm_ray, sampling);
        }
        let color = colors.iter().fold(Color{r:0.,g:0.,b:0.}, |acc, x| acc+*x);
        (ec*255. + ps*color + tr_color*tr + pm_color*pm).reform()
    }
    pub fn render(&self, sampling: usize) {
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
                let color = self.calc(&ray, sampling);
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