
pub fn rotate(v: &Vec<f32>, a: f32, b: f32, c: f32) -> Vec<f32> {
    assert_eq!(v.len(), 3);
    let x = v[0];
    let y = v[1];
    let z = v[2];
    let mut r = Vec::new();
    r.push(x*(a.cos()*b.cos()*c.cos()-a.sin()*c.sin()) 
        + y*(-a.cos()*b.cos()*c.sin()+a.sin()*c.cos()) 
        + z*a.cos()*b.sin());
    r.push(x*(a.sin()*b.cos()*c.cos()+a.cos()*c.sin())
        + y*(-a.sin()*b.cos()*c.sin()+a.cos()*c.cos())
        + z*a.sin()*b.sin());
    r.push(x*(-b.sin()*c.cos())
        + y*(b.sin()*c.sin())
        + z*b.cos());
    r
}

pub fn scale(v: &Vec<f32>, a: f32, b: f32, c: f32) -> Vec<f32> {
    assert_eq!(v.len(), 3);
    let mut r = Vec::new();
    r.push(a*v[0]);
    r.push(b*v[1]);
    r.push(c*v[2]);
    r
}

pub fn tranlate(v: &Vec<f32>, a: f32, b: f32, c: f32) -> Vec<f32> {
    assert_eq!(v.len(), 3);
    let mut r = Vec::new();
    r.push(v[0]+a);
    r.push(v[1]+b);
    r.push(v[2]+c);
    r
}