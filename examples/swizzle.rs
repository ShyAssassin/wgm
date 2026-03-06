//! Swizzle operations on vectors using the `swizzle` feature.

use wgm::{Vec2, Vec3, Vec4};

fn main() {
    let v2 = Vec2::new(1.0f32, 2.0);
    println!("v2.yx()   = {}", v2.yx());
    println!("v2.xyx()  = {}", v2.xyx());
    println!("v2.xyxy() = {}", v2.xyxy());

    let v3 = Vec3::new(1.0f32, 2.0, 3.0);
    println!("v3.xz()   = {}", v3.xz());
    println!("v3.zyx()  = {}", v3.zyx());
    println!("v3.xyzx() = {}", v3.xyzx());

    let v4 = Vec4::new(1.0f32, 2.0, 3.0, 4.0);
    println!("v4.xw()   = {}", v4.xw());
    println!("v4.wzy()  = {}", v4.wzy());
    println!("v4.wzyx() = {}", v4.wzyx());

    // Practical: homogeneous -> 3D, RGBA -> BGRA
    let pos = Vec4::new(3.0f32, 4.0, 5.0, 1.0);
    println!("\nxyz({pos}) = {}", pos.xyz());

    let rgba = Vec4::new(0.2f32, 0.4, 0.6, 1.0);
    println!("RGBA {rgba} => BGRA {}", rgba.zyxw());
}
