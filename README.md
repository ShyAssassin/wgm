# WGPU Mathematics

A fast and simple math library for game and graphics development.

> ⚠️ This library is still in active development.

## Features

- `Vec2`, `Vec3`, `Vec4` — generic vector types
- `Mat2`, `Mat3`, `Mat4` — column-major matrix types
- `Quat` — quaternion rotations, slerp, and conversion
- `Deg`, `Rad`, `Tau` — angle unit types with automatic conversion
- WGSL-style aliases (`vec2`, `vec3`, `mat4x4`, etc.)

## Feature Flags

All dependencies are optional:

| Flag | Description |
|------|-------------|
| `swizzle` | Swizzle accessors on vectors (`.xyz()`, `.zyxw()`, etc.) |
| `rand` | Random generation for vectors and quaternions |
| `serde` | Serialize / deserialize all types |
| `bytemuck` | `Pod` and `Zeroable` impls for all types |

## Usage

```toml
[dependencies]
wgm = "0.0.7"
# or with features:
wgm = { version = "0.0.7", features = ["swizzle", "serde"] }
```

```rust
use wgm::{Vec3, Mat4, Quat};
use wgm::units::Deg;

let a: Vec3<f32> = Vec3::new(1.0, 2.0, 3.0);
let b: Vec3<f32> = Vec3::splat(5.0);
let dot: f32 = a.dot(b);
let cross: Vec3<f32> = a.cross(b);
let norm: Vec3<f32> = a.normalize();

let trs: Mat4<f32> = Mat4::from_translation(Vec3::new(0.0, 1.0, 0.0))
    * Mat4::from_rotation_y(Deg(90.0))
    * Mat4::from_scale(Vec3::splat(2.0));

let q: Quat<f32> = Quat::from_axis_angle(Vec3::unit_y(), Deg(90.0));
let forward: Vec3<f32> = q * Vec3::new(0.0, 0.0, -1.0);
```
