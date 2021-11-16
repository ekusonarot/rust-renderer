mod scene;
mod vector;
mod object;
mod ray;
mod color;

use serde_json::Value;
use std::fs;

use vector::Vector3;
use scene::{Camera, Scene};

fn main() {
    let content = fs::read_to_string("./scene.json").unwrap();
    let v: Value = serde_json::from_str(&content).unwrap();
    let mut objs = Vec::new();
    if let Value::Array(objectlist) = &v["objectlist"] {
        for obj in objectlist.iter() {
            let rotate = Vector3::new(
                obj["rotate"][0].to_string().parse::<f32>().unwrap(),
                obj["rotate"][1].to_string().parse::<f32>().unwrap(),
                obj["rotate"][2].to_string().parse::<f32>().unwrap(),
            );
            let scale = Vector3::new(
                obj["scale"][0].to_string().parse::<f32>().unwrap(),
                obj["scale"][1].to_string().parse::<f32>().unwrap(),
                obj["scale"][2].to_string().parse::<f32>().unwrap(),
            );
            let translate = Vector3::new(
                obj["translate"][0].to_string().parse::<f32>().unwrap(),
                obj["translate"][1].to_string().parse::<f32>().unwrap(),
                obj["translate"][2].to_string().parse::<f32>().unwrap(),
            );
            if let Some(name) = obj["name"].as_str() {
                objs.push(object::Object::import(name, rotate, scale, translate));
            }
        }
    }
    let camera = &v["camera"];
    let sampling = camera["sampling"].to_string().parse::<usize>().unwrap();
    let position = Vector3::new(
        camera["position"][0].to_string().parse::<f32>().unwrap(),
        camera["position"][1].to_string().parse::<f32>().unwrap(),
        camera["position"][2].to_string().parse::<f32>().unwrap(),
    );
    let top = Vector3::new(
        camera["top"][0].to_string().parse::<f32>().unwrap(),
        camera["top"][1].to_string().parse::<f32>().unwrap(),
        camera["top"][2].to_string().parse::<f32>().unwrap(),
    );
    let forward = Vector3::new(
        camera["forward"][0].to_string().parse::<f32>().unwrap(),
        camera["forward"][1].to_string().parse::<f32>().unwrap(),
        camera["forward"][2].to_string().parse::<f32>().unwrap(),
    );
    let fov: f32 = camera["fov"].to_string().parse::<f32>().unwrap();
    let image_size = (camera["image_size"][0].to_string().parse::<u32>().unwrap(), camera["image_size"][1].to_string().parse::<u32>().unwrap());
    let right = top.cross(&forward);
    let camera = Camera::new(position, top, forward, right, fov, image_size);
    let scene = Scene::new(camera, objs);
    scene.render(sampling);
}