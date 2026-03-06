//! Serde JSON roundtrip for wgm types using the `serde` feature.

use serde::{Deserialize, Serialize};
use wgm::units::Deg;
use wgm::{Mat4, Quat, Vec2, Vec3, Vec4};

fn roundtrip<T: Serialize + for<'a> Deserialize<'a> + PartialEq + std::fmt::Debug>(label: &str, val: &T) {
    let json = serde_json::to_string(val).unwrap();
    let back: T = serde_json::from_str(&json).unwrap();
    assert_eq!(*val, back);
    println!("{label}: {json}");
}

fn main() {
    roundtrip("Vec2", &Vec2::new(1.0f32, 2.0));
    roundtrip("Vec3", &Vec3::new(1.0f32, 2.0, 3.0));
    roundtrip("Vec4", &Vec4::new(1.0f32, 2.0, 3.0, 4.0));
    roundtrip("Mat4", &Mat4::<f32>::from_translation(Vec3::new(1.0, 2.0, 3.0)));
    roundtrip("Quat", &Quat::from_axis_angle(Vec3::unit_y(), Deg(90.0f32)));

    // Nested struct
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Transform {
        position: Vec3<f32>,
        rotation: Quat<f32>,
        scale: Vec3<f32>,
    }

    let t = Transform {
        position: Vec3::new(10.0, 20.0, 30.0),
        rotation: Quat::from_axis_angle(Vec3::unit_y(), Deg(45.0f32)),
        scale: Vec3::splat(1.0),
    };
    roundtrip("Transform", &t);

    println!("All roundtrips passed!");
}
