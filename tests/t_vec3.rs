use wgm::Vec3;
use glam::Vec3 as GlamVec3;

#[test]
fn vec3_length() {
    let v = Vec3::new(3.0f32, 4.0, 5.0);
    let glam_v = GlamVec3::new(3.0, 4.0, 5.0);
    assert!((v.length() - glam_v.length()).abs() < 1e-6);
}

#[test]
fn vec3_length_squared() {
    let v = Vec3::new(3.0f32, 4.0, 5.0);
    let glam_v = GlamVec3::new(3.0, 4.0, 5.0);
    assert!((v.length_squared() - glam_v.length_squared()).abs() < 1e-6);
}

#[test]
fn vec3_distance() {
    let a = Vec3::new(1.0f32, 2.0, 3.0);
    let b = Vec3::new(4.0, 6.0, 8.0);
    let glam_a = GlamVec3::new(1.0, 2.0, 3.0);
    let glam_b = GlamVec3::new(4.0, 6.0, 8.0);
    assert!((a.distance(b) - glam_a.distance(glam_b)).abs() < 1e-6);
}

#[test]
fn vec3_normalize() {
    let v = Vec3::new(3.0f32, 4.0, 5.0);
    let glam_v = GlamVec3::new(3.0, 4.0, 5.0);
    let n = v.normalize();
    let glam_n = glam_v.normalize();
    assert!((n.x - glam_n.x).abs() < 1e-6);
    assert!((n.y - glam_n.y).abs() < 1e-6);
    assert!((n.z - glam_n.z).abs() < 1e-6);
}

#[test]
fn vec3_dot() {
    let a = Vec3::new(1.0f32, 2.0, 3.0);
    let b = Vec3::new(3.0, 4.0, 5.0);
    let glam_a = GlamVec3::new(1.0, 2.0, 3.0);
    let glam_b = GlamVec3::new(3.0, 4.0, 5.0);
    assert!((a.dot(b) - glam_a.dot(glam_b)).abs() < 1e-6);
}

#[test]
fn vec3_lerp() {
    let a = Vec3::new(1.0f32, 2.0, 3.0);
    let b = Vec3::new(3.0, 4.0, 5.0);
    let glam_a = GlamVec3::new(1.0, 2.0, 3.0);
    let glam_b = GlamVec3::new(3.0, 4.0, 5.0);
    let t = 0.25f32;
    let l = a.lerp(b, t);
    let glam_l = glam_a.lerp(glam_b, t);
    assert!((l.x - glam_l.x).abs() < 1e-6);
    assert!((l.y - glam_l.y).abs() < 1e-6);
    assert!((l.z - glam_l.z).abs() < 1e-6);
}

#[test]
fn vec3_new_and_splat() {
    let v = Vec3::new(1.0f32, 2.0, 3.0);
    let glam_v = GlamVec3::new(1.0, 2.0, 3.0);
    assert_eq!(v.x, glam_v.x);
    assert_eq!(v.y, glam_v.y);
    assert_eq!(v.z, glam_v.z);

    let s = Vec3::splat(5.0f32);
    let glam_s = GlamVec3::splat(5.0);
    assert_eq!(s.x, glam_s.x);
    assert_eq!(s.y, glam_s.y);
    assert_eq!(s.z, glam_s.z);
}

#[test]
fn vec3_extend_truncate() {
    let v = Vec3::new(1.0f32, 2.0, 3.0);
    let ext = v.extend(4.0);
    assert_eq!(ext.x, 1.0);
    assert_eq!(ext.y, 2.0);
    assert_eq!(ext.z, 3.0);
    assert_eq!(ext.w, 4.0);

    let t = v.truncate();
    assert_eq!(t.x, 1.0);
    assert_eq!(t.y, 2.0);
}

#[test]
fn vec3_normalize_between() {
    let v = Vec3::new(3.0f32, 4.0, 5.0);
    let glam_v = GlamVec3::new(3.0, 4.0, 5.0);
    let n = v.normalize_between(1.0, 2.0);
    let glam_n = glam_v.normalize() * (2.0 - 1.0) + 1.0;
    assert!((n.x - glam_n.x).abs() < 1e-6);
    assert!((n.y - glam_n.y).abs() < 1e-6);
    assert!((n.z - glam_n.z).abs() < 1e-6);
}

#[test]
fn vec3_min_max_clamp() {
    let a = Vec3::new(1.0f32, 5.0, 3.0);
    let b = Vec3::new(2.0, 4.0, 6.0);
    let glam_a = GlamVec3::new(1.0, 5.0, 3.0);
    let glam_b = GlamVec3::new(2.0, 4.0, 6.0);
    let min = a.min(b);
    let glam_min = glam_a.min(glam_b);
    assert_eq!(min.x, glam_min.x);
    assert_eq!(min.y, glam_min.y);
    assert_eq!(min.z, glam_min.z);

    let max = a.max(b);
    let glam_max = glam_a.max(glam_b);
    assert_eq!(max.x, glam_max.x);
    assert_eq!(max.y, glam_max.y);
    assert_eq!(max.z, glam_max.z);

    let clamp = a.clamp(Vec3::splat(2.0), Vec3::splat(6.0));
    let glam_clamp = glam_a.clamp(GlamVec3::splat(2.0), GlamVec3::splat(6.0));
    assert_eq!(clamp.x, glam_clamp.x);
    assert_eq!(clamp.y, glam_clamp.y);
    assert_eq!(clamp.z, glam_clamp.z);
}

#[test]
fn vec3_abs() {
    let v = Vec3::new(-1.0f32, 2.0, -3.0);
    let glam_v = GlamVec3::new(-1.0, 2.0, -3.0);
    let abs = v.abs();
    let glam_abs = glam_v.abs();
    assert_eq!(abs.x, glam_abs.x);
    assert_eq!(abs.y, glam_abs.y);
    assert_eq!(abs.z, glam_abs.z);
}

#[test]
fn vec3_is_nan_finite_infinite() {
    let v = Vec3::new(f32::NAN, 2.0, 3.0);
    assert!(v.is_nan());

    let v = Vec3::new(1.0f32, 2.0, 3.0);
    assert!(v.is_finite());

    let v = Vec3::new(f32::INFINITY, 2.0, 3.0);
    assert!(v.is_infinite());
}

#[test]
fn vec3_sum_product_min_max_element() {
    let v = Vec3::new(1.0f32, 2.0, 3.0);
    assert_eq!(v.sum(), 6.0);
    assert_eq!(v.product(), 6.0);
    assert_eq!(v.min_element(), 1.0);
    assert_eq!(v.max_element(), 3.0);
}

#[test]
fn vec3_any_all_zero() {
    let v = Vec3::new(0.0f32, 2.0, 3.0);
    assert!(v.any_zero());

    let v = Vec3::new(0.0f32, 0.0, 0.0);
    assert!(v.all_zero());
}

#[test]
fn vec3_reflect() {
    let v = Vec3::new(1.0f32, -1.0, 0.0);
    let n = Vec3::new(0.0, 1.0, 0.0);
    let glam_v = GlamVec3::new(1.0, -1.0, 0.0);
    let glam_n = GlamVec3::new(0.0, 1.0, 0.0);
    let r = v.reflect(n);
    let glam_r = glam_v.reflect(glam_n);
    assert!((r.x - glam_r.x).abs() < 1e-6);
    assert!((r.y - glam_r.y).abs() < 1e-6);
    assert!((r.z - glam_r.z).abs() < 1e-6);
}

#[test]
fn vec3_project_onto() {
    let v = Vec3::new(2.0f32, 3.0, 4.0);
    let onto = Vec3::new(1.0, 0.0, 0.0);
    let glam_v = GlamVec3::new(2.0, 3.0, 4.0);
    let glam_onto = GlamVec3::new(1.0, 0.0, 0.0);
    let p = v.project_onto(onto);
    let glam_p = glam_v.project_onto(glam_onto);
    assert!((p.x - glam_p.x).abs() < 1e-6);
    assert!((p.y - glam_p.y).abs() < 1e-6);
    assert!((p.z - glam_p.z).abs() < 1e-6);
}

#[test]
fn vec3_cross() {
    let a = Vec3::new(1.0f32, 0.0, 0.0);
    let b = Vec3::new(0.0, 1.0, 0.0);
    let glam_a = GlamVec3::new(1.0, 0.0, 0.0);
    let glam_b = GlamVec3::new(0.0, 1.0, 0.0);
    let c = a.cross(b);
    let glam_c = glam_a.cross(glam_b);
    assert!((c.x - glam_c.x).abs() < 1e-6);
    assert!((c.y - glam_c.y).abs() < 1e-6);
    assert!((c.z - glam_c.z).abs() < 1e-6);
}

#[test]
fn vec3_conversions() {
    let arr: [f32; 3] = Vec3::new(1.0, 2.0, 3.0).into();
    assert_eq!(arr, [1.0, 2.0, 3.0]);
    let v: Vec3<f32> = [1.0, 2.0, 3.0].into();
    assert_eq!(v, Vec3::new(1.0, 2.0, 3.0));
    let tuple: (f32, f32, f32) = v.into();
    assert_eq!(tuple, (1.0, 2.0, 3.0));
    let v2: Vec3<f32> = (1.0, 2.0, 3.0).into();
    assert_eq!(v2, Vec3::new(1.0, 2.0, 3.0));
}

#[test]
fn vec3_arithmetic() {
    let a = Vec3::new(1.0f32, 2.0, 3.0);
    let b = Vec3::new(5.0, 6.0, 7.0);

    assert_eq!(a * 2.0, Vec3::new(2.0, 4.0, 6.0));
    assert_eq!(a / 2.0, Vec3::new(0.5, 1.0, 1.5));
    assert_eq!(a + 2.0, Vec3::new(3.0, 4.0, 5.0));
    assert_eq!(a - 1.0, Vec3::new(0.0, 1.0, 2.0));

    assert_eq!(a * b, Vec3::new(5.0, 12.0, 21.0));
    assert_eq!(b / a, Vec3::new(5.0, 3.0, 7.0/3.0));
    assert_eq!(a + b, Vec3::new(6.0, 8.0, 10.0));
    assert_eq!(b - a, Vec3::new(4.0, 4.0, 4.0));
}
