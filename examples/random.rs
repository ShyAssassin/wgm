//! Random generation of wgm types using the `rand` feature.

use rand::Rng;
use wgm::{Quat, Vec2, Vec3, Vec4};

fn main() {
    let mut rng = rand::thread_rng();

    // Random vectors
    for _ in 0..3 { let v: Vec2<f32> = rng.gen(); println!("Vec2: {v}"); }
    for _ in 0..3 { let v: Vec3<f32> = rng.gen(); println!("Vec3: {v}"); }
    for _ in 0..3 { let v: Vec4<f32> = rng.gen(); println!("Vec4: {v}"); }

    // Random unit quaternions
    for _ in 0..3 {
        let q: Quat<f32> = rng.gen();
        println!("Quat: {q}  (length={:.6})", q.length());
    }

    // Random directions on a unit sphere
    for _ in 0..3 {
        let dir = rng.gen::<Vec3<f32>>().normalize();
        println!("Dir:  {dir}");
    }
}
