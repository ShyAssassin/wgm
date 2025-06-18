//! A simple example demonstrating how to use vector swizzling. and cohersing vectors to other vector types.
//! Please Note: This example requires the `swizzle` feature to be enabled.
use wgm::{Vec2, Vec3, Vec4};

fn main() {
    // Our example position vector
    let position: Vec3<f32> = Vec3::new(1.0, 2.0, 3.0);
    println!("Our position vector: {:?}", position);

    // lets say we only want the x and y components
    // instead of manually creating a new Vec2 we can use vector swizzling
    let position2d: Vec2<f32> = position.xy();
    println!("positon.xy() = {:?}", position2d);
    assert!(position2d == Vec2::new(1.0, 2.0));

    // What about creating a new Vec3 with the same y components?
    let position2: Vec3<f32> = position.yyy();
    println!("position.yyy() = {:?}", position2);
    assert!(position2 == Vec3::new(2.0, 2.0, 2.0));

    // Or a new Vec4 with the same x and y components
    let position3: Vec4<f32> = position.xyxy();
    println!("position.xyxy() = {:?}", position3);
    assert!(position3 == Vec4::new(1.0, 2.0, 1.0, 2.0));
}
