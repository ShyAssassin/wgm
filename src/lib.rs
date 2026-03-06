/*!
WebGPU mathematics (WGM) is a fast and easy to use mathematics library with minimal dependencies for graphics programming.

## Feature gates
All `wgm` dependencies are optional but you may want to enable some features to get the most out of the library.
* `swizzle` - Enable swizzling for all vectors.
* `rand` - Enable random number generation for all wgm types.
* `serde` - Enable serialization and deserialization for all wgm types.
* `bytemuck` - Enable bytemuck support for all vectors, matrices and units.
*/

/// Aliases for common types and functions conforming to WGSL conventions.
pub mod alias;
/// Units for representing angles and other quantities.
pub mod units;
/// Features that can be enabled with feature gates.
mod features;

mod mat2;
mod mat3;
mod mat4;
mod quat;
mod vec2;
mod vec3;
mod vec4;

pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4::Vec4;

pub use quat::Quat;

pub use mat2::Mat2;
pub use mat3::Mat3;
pub use mat4::Mat4;
