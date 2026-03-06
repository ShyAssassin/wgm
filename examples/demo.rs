//! Overview of wgm's core types: vectors, matrices, quaternions, and projections.

use wgm::units::Deg;
use wgm::{Mat2, Mat4, Quat, Vec2, Vec3};

fn main() {
    // Vectors
    let a = Vec3::new(1.0f32, 2.0, 3.0);
    let b = Vec3::new(4.0f32, 5.0, 6.0);
    println!("a + b     = {}", a + b);
    println!("a.dot(b)  = {}", a.dot(b));
    println!("a x b     = {}", a.cross(b));
    println!("length    = {}", a.length());
    println!("normalize = {}", a.normalize());
    println!("lerp 0.5  = {}", a.lerp(b, 0.5));
    println!("reflect   = {}", Vec3::new(1.0f32, -1.0, 0.0).reflect(Vec3::unit_y()));

    let v2 = Vec2::new(3.0f32, 4.0);
    println!("perp      = {}", v2.perp());

    // Extend / truncate
    let v4 = a.extend(4.0);
    println!("extend    = {v4}");
    println!("truncate  = {}", v4.truncate());

    // Mat2 rotation
    let rot = Mat2::<f32>::from_angle(Deg(45.0));
    println!("\nMat2 45° * (1,0) = {}", rot * Vec2::new(1.0, 0.0));

    // Mat4 TRS
    let trs = Mat4::<f32>::from_translation(Vec3::new(1.0, 2.0, 3.0))
        * Mat4::<f32>::from_rotation_y(Deg(90.0))
        * Mat4::<f32>::from_scale(Vec3::splat(2.0));
    let p = Vec3::new(1.0f32, 0.0, 0.0);
    let tp = trs.transform_point3(p);
    println!("TRS: {p} => {tp}");
    println!("inverse:  {}", trs.inverse().unwrap().transform_point3(tp));

    // Quaternions
    let q = Quat::from_axis_angle(Vec3::unit_y(), Deg(90.0f32));
    let forward = Vec3::new(0.0f32, 0.0, -1.0);
    println!("\nQuat 90° Y: {forward} => {}", q * forward);
    println!("slerp 0.5 = {}", q.slerp(Quat::identity(), 0.5) * forward);

    // Projection
    let proj = Mat4::<f32>::perspective_rh(Deg(60.0), 16.0 / 9.0, 0.1, 100.0);
    let view = Mat4::look_at_rh(
        Vec3::new(0.0f32, 0.0, 5.0),
        Vec3::zero(),
        Vec3::unit_y(),
    );
    let vp = proj * view;
    let clip = vp.mul_vec4(Vec3::new(1.0f32, 1.0, 0.0).extend(1.0));
    println!("\nclip = {clip}");
}
