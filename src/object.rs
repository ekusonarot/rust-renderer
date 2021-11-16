use tobj;
use image::RgbImage;
use image::io::Reader as ImageReader;
use std::collections::HashMap;

use crate::vector::{Vector3, Vector2};
use crate::ray::Ray;

pub const MAX_RANGE: f32 = 10000.;

#[derive(Debug)]
pub struct Texcoord {
    pub v1: Vector2,
    pub v2: Vector2,
    pub v3: Vector2,
}
impl Texcoord {
    pub fn new(v1: Vector2, v2: Vector2, v3: Vector2) -> Texcoord{
        Texcoord {
            v1,
            v2,
            v3,
        }
    }
}

#[derive(Debug)]
pub struct Normcoord {
    pub v1: Vector3,
    pub v2: Vector3,
    pub v3: Vector3,
}
impl Normcoord {
    pub fn new(v1: Vector3, v2: Vector3, v3: Vector3) -> Normcoord{
        Normcoord {
            v1,
            v2,
            v3,
        }
    }
}

#[derive(Debug)]
pub struct Plane {
    v1: Vector3,
    v2: Vector3,
    v3: Vector3,
    pub vn: Option<Normcoord>,
    pub vt: Option<Texcoord>,
    pub material_id: Option<usize>,

}
impl Plane {
    pub fn new(v1: Vector3, v2: Vector3, v3: Vector3, vn: Option<Normcoord>, vt: Option<Texcoord>, material_id: Option<usize>) -> Plane {
        Plane{
            v1,
            v2,
            v3,
            vn,
            vt,
            material_id,
        }
    }
    pub fn intersection(&self, ray: &Ray) -> Option<Vector3> {
        let e1 = self.v2 - self.v1;
        let e2 = self.v3 - self.v1;
        let r = ray.origin - self.v1;
        let vec = Vector3::new(
            r.cross(&e1).inner(&e2),
            ray.direction.cross(&e2).inner(&r),
            r.cross(&e1).inner(&ray.direction),
        );
        let t = vec/ray.direction.cross(&e2).inner(&e1);
        if 0.001 < t.x && 0. <= t.y && 0. <= t.z && t.y + t.z <= 1. && t.x < MAX_RANGE {
            Some(t)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Object {
    pub planes: Vec<Plane>,
    pub origin: Vector3,
    pub radius: f32,
    pub materials: Vec<tobj::Material>,
    pub image: HashMap<String, RgbImage>,
}
impl Object {
    pub fn import(obj_file: &str, r: Vector3, s: Vector3, t: Vector3) -> Object {
        let cornell_box = tobj::load_obj(
            obj_file,
            &tobj::LoadOptions {
                single_index: false,
                triangulate: true,
                ..Default::default()
            },
        );
        let (models, materials) = cornell_box.expect("CAN NOT OPEN OBJ FILE");
        let materials = materials.expect("CAN NOT OPEN MTL FILE");
        let mut max_radius: f32 = 0.;
        let mut planes = Vec::new();
        let mut image = HashMap::new();
        for model in models.iter() {
            for i in 0..model.mesh.indices.len() / 3 {
                let j = model.mesh.indices[3*i] as usize;
                let x = model.mesh.positions[3*j];
                let y = model.mesh.positions[3*j+1];
                let z = model.mesh.positions[3*j+2];
                let mut v1 = Vector3::new(x,y,z);
                v1 = v1.rotate(&r);
                v1 = v1.scale(&s);
                v1 = v1.translate(&t);
                let j = model.mesh.indices[3*i+1] as usize;
                let x = model.mesh.positions[3*j];
                let y = model.mesh.positions[3*j+1];
                let z = model.mesh.positions[3*j+2];
                let mut v2 = Vector3::new(x,y,z);
                v2 = v2.rotate(&r);
                v2 = v2.scale(&s);
                v2 = v2.translate(&t);
                let j = model.mesh.indices[3*i+2] as usize;
                let x = model.mesh.positions[3*j];
                let y = model.mesh.positions[3*j+1];
                let z = model.mesh.positions[3*j+2];
                let mut v3 = Vector3::new(x,y,z);
                v3 = v3.rotate(&r);
                v3 = v3.scale(&s);
                v3 = v3.translate(&t);
                if max_radius < (v1.x*v1.x + v1.y*v1.y + v1.z*v1.z).sqrt() {
                    max_radius = (v1.x*v1.x + v1.y*v1.y + v1.z*v1.z).sqrt();
                }
                if max_radius < (v2.x*v2.x + v2.y*v2.y + v2.z*v2.z).sqrt() {
                    max_radius = (v2.x*v2.x + v2.y*v2.y + v2.z*v2.z).sqrt();
                }
                if max_radius < (v3.x*v3.x + v3.y*v3.y + v3.z*v3.z).sqrt() {
                    max_radius = (v3.x*v3.x + v3.y*v3.y + v3.z*v3.z).sqrt();
                }
                let mut vn: Option<Normcoord> = None;
                if !model.mesh.normals.is_empty() {
                    let j = model.mesh.normal_indices[3*i] as usize;
                    let x = model.mesh.normals[3*j];
                    let y = model.mesh.normals[3*j+1];
                    let z = model.mesh.normals[3*j+2];
                    let mut vn1 = Vector3::new(x, y, z);
                    vn1 = vn1.rotate(&r);
                    let j = model.mesh.normal_indices[3*i+1] as usize;
                    let x = model.mesh.normals[3*j];
                    let y = model.mesh.normals[3*j+1];
                    let z = model.mesh.normals[3*j+2];
                    let mut vn2 = Vector3::new(x, y, z);
                    vn2 = vn2.rotate(&r);
                    let j = model.mesh.normal_indices[3*i+2] as usize;
                    let x = model.mesh.normals[3*j];
                    let y = model.mesh.normals[3*j+1];
                    let z = model.mesh.normals[3*j+2];
                    let mut vn3 = Vector3::new(x, y, z);
                    vn3 = vn3.rotate(&r);
                    vn = Some(Normcoord::new(vn1, vn2, vn3));
                }
                let mut vt: Option<Texcoord> = None;
                if !model.mesh.texcoords.is_empty() {
                    let j = model.mesh.texcoord_indices[3*i] as usize;
                    let x = model.mesh.texcoords[2*j];
                    let y = model.mesh.texcoords[2*j+1];
                    let vt1 = Vector2::new(x, y);
                    let j = model.mesh.texcoord_indices[3*i+1] as usize;
                    let x = model.mesh.texcoords[2*j];
                    let y = model.mesh.texcoords[2*j+1];
                    let vt2 = Vector2::new(x, y);
                    let j = model.mesh.texcoord_indices[3*i+2] as usize;
                    let x = model.mesh.texcoords[2*j];
                    let y = model.mesh.texcoords[2*j+1];
                    let vt3 = Vector2::new(x, y);
                    vt = Some(Texcoord::new(vt1, vt2, vt3));
                }
                planes.push(Plane::new(v1,v2,v3,vn,vt,model.mesh.material_id));
            }
            for material in materials.iter() {
                println!("{:?}", material);
                if material.ambient_texture != "" {
                    let t_image = ImageReader::open(&material.ambient_texture).unwrap().decode().unwrap();
                    image.insert(String::from(&material.ambient_texture), t_image.to_rgb8());
                }
                if material.diffuse_texture != "" {
                    let t_image = ImageReader::open(&material.diffuse_texture).unwrap().decode().unwrap();
                    image.insert(String::from(&material.diffuse_texture), t_image.to_rgb8());
                }
                if material.specular_texture != "" {
                    let t_image = ImageReader::open(&material.specular_texture).unwrap().decode().unwrap();
                    image.insert(String::from(&material.specular_texture), t_image.to_rgb8());
                }
                if material.normal_texture != "" {
                    let t_image = ImageReader::open(&material.normal_texture).unwrap().decode().unwrap();
                    image.insert(String::from(&material.normal_texture), t_image.to_rgb8());
                }
                if material.shininess_texture != "" {
                    let t_image = ImageReader::open(&material.shininess_texture).unwrap().decode().unwrap();
                    image.insert(String::from(&material.shininess_texture), t_image.to_rgb8());
                }
                if material.dissolve_texture != "" {
                    let t_image = ImageReader::open(&material.dissolve_texture).unwrap().decode().unwrap();
                    image.insert(String::from(&material.dissolve_texture), t_image.to_rgb8());
                }
                if let Some(map_pr) = material.unknown_param.get("map_Pr") {
                    let t_image = ImageReader::open(map_pr).unwrap().decode().unwrap();
                    image.insert(String::from(map_pr), t_image.to_rgb8());
                }
                if let Some(map_pm) = material.unknown_param.get("map_Pm") {
                    let t_image = ImageReader::open(map_pm).unwrap().decode().unwrap();
                    image.insert(String::from(map_pm), t_image.to_rgb8());
                }
                if let Some(map_ps) = material.unknown_param.get("map_Ps") {
                    let t_image = ImageReader::open(map_ps).unwrap().decode().unwrap();
                    image.insert(String::from(map_ps), t_image.to_rgb8());
                }
            }
        }
        Object {
            planes,
            origin: t,
            radius: max_radius,
            materials,
            image,
        }
    }
    fn intersection(&self, ray: &Ray) -> Option<(Vector3, usize)> {
        let b = ray.direction.inner(&(ray.origin- self.origin));
        let a = ray.direction.inner(&(ray.direction));
        let c = (ray.origin-self.origin).inner(&(ray.origin-self.origin)) - self.radius*self.radius;
        let mut min_d = Vector3::new(MAX_RANGE,0.,0.);
        let mut is_hit = false;
        let mut plane_num: usize = 0;
        if b*b-a*c < 0. {
            None
        } else {
            self.planes.iter().enumerate().for_each(|(i, plane)| {
                if let Some(v) = plane.intersection(ray) {
                    if v.x < min_d.x {
                        is_hit = true;
                        min_d = v;
                        plane_num = i;
                    }
                }
            });
            if is_hit {
                Some((min_d, plane_num))
            } else {
                None
            }
        }
    }
    pub fn crossjudge<'a>(objs: &'a Vec<Object>, ray: &'a Ray) -> Option<(Vector3, &'a Plane, &'a Object)>{
        let mut is_hit = false;
        let mut min_d = Vector3::new(MAX_RANGE,0.,0.);
        let mut plane_num: usize = 0;
        let mut obj_num: usize = 0;
        objs.iter().enumerate().for_each(|(i, obj)| {
            if let Some((v, t_plane_num)) = obj.intersection(ray) {
                if v.x < min_d.x {
                    min_d = v;
                    is_hit = true;
                    plane_num = t_plane_num;
                    obj_num = i;
                }
            }
        });
        if is_hit {
            Some((
                min_d,
                &(objs[obj_num].planes[plane_num]),
                &objs[obj_num],
            ))
        } else {
            None
        }
    }
}