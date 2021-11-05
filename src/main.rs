use tobj;

mod scene;
mod operation;

fn main() {
    let cornell_box = tobj::load_obj(
        "airboat.obj",
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
    );
    let (models, materials) = cornell_box.expect("Failed to load obj");
    let _materials = materials.expect("Failed to load MTL file");

    let mut planes = Vec::new();
    for model in models.iter() {
        for i in 0..model.mesh.indices.len() / 3 {
            let j = model.mesh.indices[3*i] as usize;
            let x = model.mesh.positions[3*j];
            let y = model.mesh.positions[3*j+1];
            let z = model.mesh.positions[3*j+2];
            let mut v1 = vec![x,y,z];
            v1 = operation::rotate(&v1,0.,0.,0.);
            let j = model.mesh.indices[3*i+1] as usize;
            let x = model.mesh.positions[3*j];
            let y = model.mesh.positions[3*j+1];
            let z = model.mesh.positions[3*j+2];
            let mut v2 = vec![x,y,z];
            v2 = operation::rotate(&v2,0.,0.,0.);
            let j = model.mesh.indices[3*i+2] as usize;
            let x = model.mesh.positions[3*j];
            let y = model.mesh.positions[3*j+1];
            let z = model.mesh.positions[3*j+2];
            let mut v3 = vec![x,y,z];
            v3 = operation::rotate(&v3,0.,0.,0.);
            planes.push(scene::Plane::new(v1,v2,v3));
        }
    }
    let position = vec![10.0, 0.0, 0.0];
    let top = vec![0.0, 0.0, 1.0];
    let forward = vec![-1.0, 0.0, 0.0];
    let right = vec![0.0,1.0,0.0];
    let fov = 90.0;
    let image_size = (1280, 720);
    let camera = scene::Camera::new(position, top, forward, right, fov, image_size);
    let scene = scene::Scene::new(camera, planes);
    scene.render();
    //println!("{:?}", scene);
}