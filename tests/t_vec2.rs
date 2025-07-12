use wgm::Vec2;
use glam::Vec2 as GlamVec2;

#[test]
fn vec2_length() {
    let v = Vec2::new(3.0f32, 4.0);
    let glam_v = GlamVec2::new(3.0, 4.0);
    assert!((v.length() - glam_v.length()).abs() < 1e-6);
}

#[test]
fn vec2_length_squared() {
    let v = Vec2::new(3.0f32, 4.0);
    let glam_v = GlamVec2::new(3.0, 4.0);
    assert!((v.length_squared() - glam_v.length_squared()).abs() < 1e-6);
}

#[test]
fn vec2_distance() {
    let a = Vec2::new(1.0f32, 2.0);
    let b = Vec2::new(4.0, 6.0);
    let glam_a = GlamVec2::new(1.0, 2.0);
    let glam_b = GlamVec2::new(4.0, 6.0);
    assert!((a.distance(b) - glam_a.distance(glam_b)).abs() < 1e-6);
}

#[test]
fn vec2_normalize() {
    let v = Vec2::new(3.0f32, 4.0);
    let glam_v = GlamVec2::new(3.0, 4.0);
    let n = v.normalize();
    let glam_n = glam_v.normalize();
    assert!((n.x - glam_n.x).abs() < 1e-6);
    assert!((n.y - glam_n.y).abs() < 1e-6);
}

#[test]
fn vec2_dot() {
    let a = Vec2::new(1.0f32, 2.0);
    let b = Vec2::new(3.0, 4.0);
    let glam_a = GlamVec2::new(1.0, 2.0);
    let glam_b = GlamVec2::new(3.0, 4.0);
    assert!((a.dot(b) - glam_a.dot(glam_b)).abs() < 1e-6);
}

#[test]
fn vec2_lerp() {
    let a = Vec2::new(1.0f32, 2.0);
    let b = Vec2::new(3.0, 4.0);
    let glam_a = GlamVec2::new(1.0, 2.0);
    let glam_b = GlamVec2::new(3.0, 4.0);
    let t = 0.25f32;
    let l = a.lerp(b, t);
    let glam_l = glam_a.lerp(glam_b, t);
    assert!((l.x - glam_l.x).abs() < 1e-6);
    assert!((l.y - glam_l.y).abs() < 1e-6);
}

#[test]
fn vec2_new_and_splat() {
    let v = Vec2::new(1.0f32, 2.0);
    let glam_v = GlamVec2::new(1.0, 2.0);
    assert_eq!(v.x, glam_v.x);
    assert_eq!(v.y, glam_v.y);

    let s = Vec2::splat(5.0f32);
    let glam_s = GlamVec2::splat(5.0);
    assert_eq!(s.x, glam_s.x);
    assert_eq!(s.y, glam_s.y);
}

#[test]
fn vec2_extend_truncate() {
    let v = Vec2::new(1.0f32, 2.0);
    let ext = v.extend(3.0);
    assert_eq!(ext.x, 1.0);
    assert_eq!(ext.y, 2.0);
    assert_eq!(ext.z, 3.0);

    let t = v.truncate();
    assert_eq!(t, 1.0);
}

#[test]
fn vec2_normalize_between() {
    let v = Vec2::new(3.0f32, 4.0);
    let glam_v = GlamVec2::new(3.0, 4.0);
    let n = v.normalize_between(1.0, 2.0);
    let glam_n = glam_v.normalize() * (2.0 - 1.0) + 1.0;
    assert!((n.x - glam_n.x).abs() < 1e-6);
    assert!((n.y - glam_n.y).abs() < 1e-6);
}

#[test]
fn vec2_min_max_clamp() {
    let a = Vec2::new(1.0f32, 5.0);
    let b = Vec2::new(2.0, 4.0);
    let glam_a = GlamVec2::new(1.0, 5.0);
    let glam_b = GlamVec2::new(2.0, 4.0);
    let min = a.min(b);
    let glam_min = glam_a.min(glam_b);
    assert_eq!(min.x, glam_min.x);
    assert_eq!(min.y, glam_min.y);

    let max = a.max(b);
    let glam_max = glam_a.max(glam_b);
    assert_eq!(max.x, glam_max.x);
    assert_eq!(max.y, glam_max.y);

    let clamp = a.clamp(Vec2::splat(2.0), Vec2::splat(6.0));
    let glam_clamp = glam_a.clamp(GlamVec2::splat(2.0), GlamVec2::splat(6.0));
    assert_eq!(clamp.x, glam_clamp.x);
    assert_eq!(clamp.y, glam_clamp.y);
}

#[test]
fn vec2_abs() {
    let v = Vec2::new(-1.0f32, 2.0);
    let glam_v = GlamVec2::new(-1.0, 2.0);
    let abs = v.abs();
    let glam_abs = glam_v.abs();
    assert_eq!(abs.x, glam_abs.x);
    assert_eq!(abs.y, glam_abs.y);
}

#[test]
fn vec2_is_nan_finite_infinite() {
    let v = Vec2::new(f32::NAN, 2.0);
    assert!(v.is_nan());

    let v = Vec2::new(1.0f32, 2.0);
    assert!(v.is_finite());

    let v = Vec2::new(f32::INFINITY, 2.0);
    assert!(v.is_infinite());
}

#[test]
fn vec2_sum_product_min_max_element() {
    let v = Vec2::new(1.0f32, 2.0);
    assert_eq!(v.sum(), 3.0);
    assert_eq!(v.product(), 2.0);
    assert_eq!(v.min_element(), 1.0);
    assert_eq!(v.max_element(), 2.0);
}

#[test]
fn vec2_any_all_zero() {
    let v = Vec2::new(0.0f32, 2.0);
    assert!(v.any_zero());

    let v = Vec2::new(0.0f32, 0.0);
    assert!(v.all_zero());
}

#[test]
fn vec2_reflect() {
    let v = Vec2::new(1.0f32, -1.0);
    let n = Vec2::new(0.0, 1.0);
    let glam_v = GlamVec2::new(1.0, -1.0);
    let glam_n = GlamVec2::new(0.0, 1.0);
    let r = v.reflect(n);
    let glam_r = glam_v.reflect(glam_n);
    assert!((r.x - glam_r.x).abs() < 1e-6);
    assert!((r.y - glam_r.y).abs() < 1e-6);
}

#[test]
fn vec2_project_onto() {
    let v = Vec2::new(2.0f32, 3.0);
    let onto = Vec2::new(1.0, 0.0);
    let glam_v = GlamVec2::new(2.0, 3.0);
    let glam_onto = GlamVec2::new(1.0, 0.0);
    let p = v.project_onto(onto);
    let glam_p = glam_v.project_onto(glam_onto);
    assert!((p.x - glam_p.x).abs() < 1e-6);
    assert!((p.y - glam_p.y).abs() < 1e-6);
}

#[test]
fn vec2_conversions() {
    let arr: [f32; 2] = Vec2::new(1.0, 2.0).into();
    assert_eq!(arr, [1.0, 2.0]);
    let v: Vec2<f32> = [1.0, 2.0].into();
    assert_eq!(v, Vec2::new(1.0, 2.0));
    let tuple: (f32, f32) = v.into();
    assert_eq!(tuple, (1.0, 2.0));
    let v2: Vec2<f32> = (1.0, 2.0).into();
    assert_eq!(v2, Vec2::new(1.0, 2.0));
}

#[test]
fn vec2_arithmetic() {
    let a = Vec2::new(1.0f32, 2.0);
    let b = Vec2::new(5.0, 6.0);

    assert_eq!(a * 2.0, Vec2::new(2.0, 4.0));
    assert_eq!(a / 2.0, Vec2::new(0.5, 1.0));
    assert_eq!(a + 2.0, Vec2::new(3.0, 4.0));
    assert_eq!(a - 1.0, Vec2::new(0.0, 1.0));

    assert_eq!(a * b, Vec2::new(5.0, 12.0));
    assert_eq!(b / a, Vec2::new(5.0, 3.0));
    assert_eq!(a + b, Vec2::new(6.0, 8.0));
    assert_eq!(b - a, Vec2::new(4.0, 4.0));
}
