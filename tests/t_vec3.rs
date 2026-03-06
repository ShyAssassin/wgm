use wgm::{Vec2, Vec3};

const EPSILON: f32 = 1e-5;

fn approx_eq(a: f32, b: f32) -> bool {
    return (a - b).abs() < EPSILON;
}

fn vec3_approx_eq(a: Vec3<f32>, b: Vec3<f32>) -> bool {
    return approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z);
}

fn gv3(v: Vec3<f32>) -> glam::Vec3 {
    return glam::Vec3::new(v.x, v.y, v.z);
}

fn wv3(v: glam::Vec3) -> Vec3<f32> {
    return Vec3::new(v.x, v.y, v.z);
}

// ---- Construction ----

#[test]
fn new() {
    let v = Vec3::new(1.0f32, 2.0, 3.0);
    let g = glam::Vec3::new(1.0, 2.0, 3.0);
    assert!(approx_eq(v.x, g.x));
    assert!(approx_eq(v.y, g.y));
    assert!(approx_eq(v.z, g.z));
}

#[test]
fn splat() {
    let v = Vec3::splat(5.0f32);
    let g = glam::Vec3::splat(5.0);
    assert!(vec3_approx_eq(v, wv3(g)));
}

#[test]
fn zero() {
    let v = Vec3::<f32>::zero();
    let g = glam::Vec3::ZERO;
    assert!(vec3_approx_eq(v, wv3(g)));
}

#[test]
fn one() {
    let v = Vec3::<f32>::one();
    let g = glam::Vec3::ONE;
    assert!(vec3_approx_eq(v, wv3(g)));
}

#[test]
fn unit_x() {
    let v = Vec3::<f32>::unit_x();
    let g = glam::Vec3::X;
    assert!(vec3_approx_eq(v, wv3(g)));
}

#[test]
fn unit_y() {
    let v = Vec3::<f32>::unit_y();
    let g = glam::Vec3::Y;
    assert!(vec3_approx_eq(v, wv3(g)));
}

#[test]
fn unit_z() {
    let v = Vec3::<f32>::unit_z();
    let g = glam::Vec3::Z;
    assert!(vec3_approx_eq(v, wv3(g)));
}

// ---- Geometry ----

#[test]
fn length() {
    let v = Vec3::new(1.0f32, 2.0, 3.0);
    let g = glam::Vec3::new(1.0, 2.0, 3.0);
    assert!(approx_eq(v.length(), g.length()));
}

#[test]
fn length_squared() {
    let v = Vec3::new(1.0f32, 2.0, 3.0);
    let g = glam::Vec3::new(1.0, 2.0, 3.0);
    assert!(approx_eq(v.length_squared(), g.length_squared()));
}

#[test]
fn distance() {
    let a = Vec3::new(1.0f32, 2.0, 3.0);
    let b = Vec3::new(4.0, 6.0, 8.0);
    let ga = glam::Vec3::new(1.0, 2.0, 3.0);
    let gb = glam::Vec3::new(4.0, 6.0, 8.0);
    assert!(approx_eq(a.distance(b), ga.distance(gb)));
}

#[test]
fn distance_squared() {
    let a = Vec3::new(1.0f32, 2.0, 3.0);
    let b = Vec3::new(4.0, 6.0, 8.0);
    let ga = glam::Vec3::new(1.0, 2.0, 3.0);
    let gb = glam::Vec3::new(4.0, 6.0, 8.0);
    assert!(approx_eq(a.distance_squared(b), ga.distance_squared(gb)));
}

#[test]
fn normalize() {
    let v = Vec3::new(1.0f32, 2.0, 3.0);
    let g = glam::Vec3::new(1.0, 2.0, 3.0);
    assert!(vec3_approx_eq(v.normalize(), wv3(g.normalize())));
}

#[test]
fn dot() {
    let a = Vec3::new(1.0f32, 2.0, 3.0);
    let b = Vec3::new(4.0, 5.0, 6.0);
    let ga = glam::Vec3::new(1.0, 2.0, 3.0);
    let gb = glam::Vec3::new(4.0, 5.0, 6.0);
    assert!(approx_eq(a.dot(b), ga.dot(gb)));
}

#[test]
fn cross() {
    let a = Vec3::new(1.0f32, 0.0, 0.0);
    let b = Vec3::new(0.0, 1.0, 0.0);
    let ga = glam::Vec3::new(1.0, 0.0, 0.0);
    let gb = glam::Vec3::new(0.0, 1.0, 0.0);
    assert!(vec3_approx_eq(a.cross(b), wv3(ga.cross(gb))));
}

#[test]
fn cross_arbitrary() {
    let a = Vec3::new(2.0f32, 3.0, 4.0);
    let b = Vec3::new(5.0, 6.0, 7.0);
    let ga = glam::Vec3::new(2.0, 3.0, 4.0);
    let gb = glam::Vec3::new(5.0, 6.0, 7.0);
    assert!(vec3_approx_eq(a.cross(b), wv3(ga.cross(gb))));
}

#[test]
fn angle_between() {
    let a = Vec3::new(1.0f32, 0.0, 0.0);
    let b = Vec3::new(0.0, 1.0, 0.0);
    let ga = glam::Vec3::new(1.0, 0.0, 0.0);
    let gb = glam::Vec3::new(0.0, 1.0, 0.0);
    assert!(approx_eq(a.angle_between(b).inner(), ga.angle_between(gb)));
}

#[test]
fn angle_between_arbitrary() {
    let a = Vec3::new(1.0f32, 2.0, 3.0);
    let b = Vec3::new(-1.0, 1.0, 2.0);
    let ga = glam::Vec3::new(1.0, 2.0, 3.0);
    let gb = glam::Vec3::new(-1.0, 1.0, 2.0);
    assert!(approx_eq(a.angle_between(b).inner(), ga.angle_between(gb)));
}

// ---- Interpolation ----

#[test]
fn lerp() {
    let a = Vec3::new(0.0f32, 0.0, 0.0);
    let b = Vec3::new(10.0, 20.0, 30.0);
    let ga = glam::Vec3::new(0.0, 0.0, 0.0);
    let gb = glam::Vec3::new(10.0, 20.0, 30.0);
    assert!(vec3_approx_eq(a.lerp(b, 0.5), wv3(ga.lerp(gb, 0.5))));
}

#[test]
fn lerp_endpoints() {
    let a = Vec3::new(1.0f32, 2.0, 3.0);
    let b = Vec3::new(4.0, 5.0, 6.0);
    let ga = glam::Vec3::new(1.0, 2.0, 3.0);
    let gb = glam::Vec3::new(4.0, 5.0, 6.0);
    assert!(vec3_approx_eq(a.lerp(b, 0.0), wv3(ga.lerp(gb, 0.0))));
    assert!(vec3_approx_eq(a.lerp(b, 1.0), wv3(ga.lerp(gb, 1.0))));
}

// ---- Component-wise math ----

#[test]
fn min() {
    let a = Vec3::new(1.0f32, 5.0, 3.0);
    let b = Vec3::new(4.0, 2.0, 6.0);
    let ga = glam::Vec3::new(1.0, 5.0, 3.0);
    let gb = glam::Vec3::new(4.0, 2.0, 6.0);
    assert!(vec3_approx_eq(a.min(b), wv3(ga.min(gb))));
}

#[test]
fn max() {
    let a = Vec3::new(1.0f32, 5.0, 3.0);
    let b = Vec3::new(4.0, 2.0, 6.0);
    let ga = glam::Vec3::new(1.0, 5.0, 3.0);
    let gb = glam::Vec3::new(4.0, 2.0, 6.0);
    assert!(vec3_approx_eq(a.max(b), wv3(ga.max(gb))));
}

#[test]
fn clamp() {
    let v = Vec3::new(-1.0f32, 5.0, 2.0);
    let min = Vec3::new(0.0, 0.0, 0.0);
    let max = Vec3::new(3.0, 3.0, 3.0);
    let g = glam::Vec3::new(-1.0, 5.0, 2.0);
    let gmin = glam::Vec3::new(0.0, 0.0, 0.0);
    let gmax = glam::Vec3::new(3.0, 3.0, 3.0);
    assert!(vec3_approx_eq(v.clamp(min, max), wv3(g.clamp(gmin, gmax))));
}

#[test]
fn abs() {
    let v = Vec3::new(-3.0f32, -4.0, -5.0);
    let g = glam::Vec3::new(-3.0, -4.0, -5.0);
    assert!(vec3_approx_eq(v.abs(), wv3(g.abs())));
}

#[test]
fn signum() {
    let v = Vec3::new(-3.0f32, 4.0, 0.0);
    let g = glam::Vec3::new(-3.0, 4.0, 0.0);
    assert!(vec3_approx_eq(v.signum(), wv3(g.signum())));
}

#[test]
fn recip() {
    let v = Vec3::new(2.0f32, 4.0, 8.0);
    let g = glam::Vec3::new(2.0, 4.0, 8.0);
    assert!(vec3_approx_eq(v.recip(), wv3(g.recip())));
}

#[test]
fn floor() {
    let v = Vec3::new(1.7f32, -2.3, 3.9);
    let g = glam::Vec3::new(1.7, -2.3, 3.9);
    assert!(vec3_approx_eq(v.floor(), wv3(g.floor())));
}

#[test]
fn ceil() {
    let v = Vec3::new(1.2f32, -2.8, 3.1);
    let g = glam::Vec3::new(1.2, -2.8, 3.1);
    assert!(vec3_approx_eq(v.ceil(), wv3(g.ceil())));
}

#[test]
fn round() {
    let v = Vec3::new(1.5f32, -2.5, 3.4);
    let g = glam::Vec3::new(1.5, -2.5, 3.4);
    assert!(vec3_approx_eq(v.round(), wv3(g.round())));
}

#[test]
fn fract() {
    let v = Vec3::new(1.7f32, -2.3, 3.9);
    let g = glam::Vec3::new(1.7, -2.3, 3.9);
    assert!(vec3_approx_eq(v.fract(), wv3(g.fract())));
}

#[test]
fn powf() {
    let v = Vec3::new(2.0f32, 3.0, 4.0);
    let g = glam::Vec3::new(2.0, 3.0, 4.0);
    assert!(vec3_approx_eq(v.powf(2.0), wv3(g.powf(2.0))));
}

// ---- Reduction ----

#[test]
fn sum() {
    let v = Vec3::new(1.0f32, 2.0, 3.0);
    let g = glam::Vec3::new(1.0, 2.0, 3.0);
    assert!(approx_eq(v.sum(), g.element_sum()));
}

#[test]
fn product() {
    let v = Vec3::new(2.0f32, 3.0, 4.0);
    let g = glam::Vec3::new(2.0, 3.0, 4.0);
    assert!(approx_eq(v.product(), g.element_product()));
}

#[test]
fn min_element() {
    let v = Vec3::new(3.0f32, 1.0, 5.0);
    let g = glam::Vec3::new(3.0, 1.0, 5.0);
    assert!(approx_eq(v.min_element(), g.min_element()));
}

#[test]
fn max_element() {
    let v = Vec3::new(3.0f32, 7.0, 5.0);
    let g = glam::Vec3::new(3.0, 7.0, 5.0);
    assert!(approx_eq(v.max_element(), g.max_element()));
}

// ---- Reflection / Projection ----

#[test]
fn project_onto() {
    let v = Vec3::new(3.0f32, 4.0, 0.0);
    let onto = Vec3::new(1.0, 0.0, 0.0);
    let g = glam::Vec3::new(3.0, 4.0, 0.0);
    let gonto = glam::Vec3::new(1.0, 0.0, 0.0);
    assert!(vec3_approx_eq(v.project_onto(onto), wv3(g.project_onto(gonto))));
}

#[test]
fn project_onto_diagonal() {
    let v = Vec3::new(3.0f32, 4.0, 5.0);
    let onto = Vec3::new(1.0, 1.0, 1.0);
    let g = glam::Vec3::new(3.0, 4.0, 5.0);
    let gonto = glam::Vec3::new(1.0, 1.0, 1.0);
    assert!(vec3_approx_eq(v.project_onto(onto), wv3(g.project_onto(gonto))));
}

#[test]
fn reject_from() {
    let v = Vec3::new(3.0f32, 4.0, 0.0);
    let from = Vec3::new(1.0, 0.0, 0.0);
    let g = glam::Vec3::new(3.0, 4.0, 0.0);
    let gfrom = glam::Vec3::new(1.0, 0.0, 0.0);
    assert!(vec3_approx_eq(v.reject_from(from), wv3(g.reject_from(gfrom))));
}

#[test]
fn reject_from_diagonal() {
    let v = Vec3::new(3.0f32, 4.0, 5.0);
    let from = Vec3::new(1.0, 1.0, 1.0);
    let g = glam::Vec3::new(3.0, 4.0, 5.0);
    let gfrom = glam::Vec3::new(1.0, 1.0, 1.0);
    assert!(vec3_approx_eq(v.reject_from(from), wv3(g.reject_from(gfrom))));
}

#[test]
fn reflect() {
    let v = Vec3::new(1.0f32, -1.0, 0.0);
    let n = Vec3::new(0.0, 1.0, 0.0);
    // reflect(v,n) = v - 2*dot(v,n)*n
    let expected = gv3(v) - 2.0 * gv3(v).dot(gv3(n)) * gv3(n);
    assert!(vec3_approx_eq(v.reflect(n), wv3(expected)));
}

// ---- Extend / Truncate ----

#[test]
fn extend() {
    let v = Vec3::new(1.0f32, 2.0, 3.0);
    let g = glam::Vec3::new(1.0, 2.0, 3.0);
    let ext = v.extend(4.0);
    let gext = g.extend(4.0);
    assert!(approx_eq(ext.x, gext.x));
    assert!(approx_eq(ext.y, gext.y));
    assert!(approx_eq(ext.z, gext.z));
    assert!(approx_eq(ext.w, gext.w));
}

#[test]
fn truncate() {
    let v = Vec3::new(1.0f32, 2.0, 3.0);
    let g = glam::Vec3::new(1.0, 2.0, 3.0);
    let t = v.truncate();
    let gt = g.truncate();
    assert!(approx_eq(t.x, gt.x));
    assert!(approx_eq(t.y, gt.y));
}

// ---- Mul-Add ----

#[test]
fn mul_add() {
    let v = Vec3::new(1.0f32, 2.0, 3.0);
    let a = Vec3::new(4.0, 5.0, 6.0);
    let b = Vec3::new(7.0, 8.0, 9.0);
    let g = glam::Vec3::new(1.0, 2.0, 3.0);
    let ga = glam::Vec3::new(4.0, 5.0, 6.0);
    let gb = glam::Vec3::new(7.0, 8.0, 9.0);
    assert!(vec3_approx_eq(v.mul_add(a, b), wv3(g.mul_add(ga, gb))));
}

// ---- Operators ----

#[test]
fn add() {
    let a = Vec3::new(1.0f32, 2.0, 3.0);
    let b = Vec3::new(4.0, 5.0, 6.0);
    let ga = glam::Vec3::new(1.0, 2.0, 3.0);
    let gb = glam::Vec3::new(4.0, 5.0, 6.0);
    assert!(vec3_approx_eq(a + b, wv3(ga + gb)));
}

#[test]
fn sub() {
    let a = Vec3::new(5.0f32, 6.0, 7.0);
    let b = Vec3::new(1.0, 2.0, 3.0);
    let ga = glam::Vec3::new(5.0, 6.0, 7.0);
    let gb = glam::Vec3::new(1.0, 2.0, 3.0);
    assert!(vec3_approx_eq(a - b, wv3(ga - gb)));
}

#[test]
fn mul() {
    let a = Vec3::new(2.0f32, 3.0, 4.0);
    let b = Vec3::new(5.0, 6.0, 7.0);
    let ga = glam::Vec3::new(2.0, 3.0, 4.0);
    let gb = glam::Vec3::new(5.0, 6.0, 7.0);
    assert!(vec3_approx_eq(a * b, wv3(ga * gb)));
}

#[test]
fn div() {
    let a = Vec3::new(10.0f32, 20.0, 30.0);
    let b = Vec3::new(2.0, 5.0, 6.0);
    let ga = glam::Vec3::new(10.0, 20.0, 30.0);
    let gb = glam::Vec3::new(2.0, 5.0, 6.0);
    assert!(vec3_approx_eq(a / b, wv3(ga / gb)));
}

#[test]
fn mul_scalar() {
    let v = Vec3::new(2.0f32, 3.0, 4.0);
    let g = glam::Vec3::new(2.0, 3.0, 4.0);
    assert!(vec3_approx_eq(v * 5.0, wv3(g * 5.0)));
}

#[test]
fn div_scalar() {
    let v = Vec3::new(10.0f32, 20.0, 30.0);
    let g = glam::Vec3::new(10.0, 20.0, 30.0);
    assert!(vec3_approx_eq(v / 5.0, wv3(g / 5.0)));
}

#[test]
fn neg() {
    let v = Vec3::new(1.0f32, -2.0, 3.0);
    let g = glam::Vec3::new(1.0, -2.0, 3.0);
    assert!(vec3_approx_eq(-v, wv3(-g)));
}

// ---- Assign Ops ----

#[test]
fn add_assign() {
    let mut v = Vec3::new(1.0f32, 2.0, 3.0);
    v += Vec3::new(4.0, 5.0, 6.0);
    let mut g = glam::Vec3::new(1.0, 2.0, 3.0);
    g += glam::Vec3::new(4.0, 5.0, 6.0);
    assert!(vec3_approx_eq(v, wv3(g)));
}

#[test]
fn sub_assign() {
    let mut v = Vec3::new(5.0f32, 6.0, 7.0);
    v -= Vec3::new(1.0, 2.0, 3.0);
    let mut g = glam::Vec3::new(5.0, 6.0, 7.0);
    g -= glam::Vec3::new(1.0, 2.0, 3.0);
    assert!(vec3_approx_eq(v, wv3(g)));
}

#[test]
fn mul_assign() {
    let mut v = Vec3::new(2.0f32, 3.0, 4.0);
    v *= 4.0f32;
    let mut g = glam::Vec3::new(2.0, 3.0, 4.0);
    g *= 4.0;
    assert!(vec3_approx_eq(v, wv3(g)));
}

#[test]
fn div_assign() {
    let mut v = Vec3::new(10.0f32, 20.0, 30.0);
    v /= 5.0f32;
    let mut g = glam::Vec3::new(10.0, 20.0, 30.0);
    g /= 5.0;
    assert!(vec3_approx_eq(v, wv3(g)));
}

// ---- Boolean checks ----

#[test]
fn is_nan() {
    assert!(Vec3::new(f32::NAN, 0.0, 0.0).is_nan());
    assert!(!Vec3::new(1.0f32, 2.0, 3.0).is_nan());
}

#[test]
fn is_finite() {
    assert!(Vec3::new(1.0f32, 2.0, 3.0).is_finite());
    assert!(!Vec3::new(f32::INFINITY, 0.0, 0.0).is_finite());
}

#[test]
fn is_infinite() {
    assert!(!Vec3::new(1.0f32, 2.0, 3.0).is_infinite());
    assert!(Vec3::new(f32::INFINITY, 0.0, 0.0).is_infinite());
}

#[test]
fn any_zero() {
    assert!(Vec3::new(0.0f32, 1.0, 2.0).any_zero());
    assert!(!Vec3::new(1.0f32, 2.0, 3.0).any_zero());
}

#[test]
fn all_zero() {
    assert!(Vec3::new(0.0f32, 0.0, 0.0).all_zero());
    assert!(!Vec3::new(0.0f32, 0.0, 1.0).all_zero());
}

// ---- Conversions ----

#[test]
fn to_array() {
    let v = Vec3::new(1.0f32, 2.0, 3.0);
    let g = glam::Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v.to_array(), g.to_array());
}

#[test]
fn from_array() {
    let arr = [1.0f32, 2.0, 3.0];
    let v: Vec3<f32> = arr.into();
    let g = glam::Vec3::from_array(arr);
    assert!(vec3_approx_eq(v, wv3(g)));
}

#[test]
fn from_tuple() {
    let t = (1.0f32, 2.0, 3.0);
    let v: Vec3<f32> = t.into();
    let g = glam::Vec3::from(t);
    assert!(vec3_approx_eq(v, wv3(g)));
}

#[test]
fn into_tuple() {
    let v = Vec3::new(1.0f32, 2.0, 3.0);
    let t: (f32, f32, f32) = v.into();
    let g = glam::Vec3::new(1.0, 2.0, 3.0);
    let gt: (f32, f32, f32) = g.into();
    assert_eq!(t, gt);
}

// ---- Index ----

#[test]
fn index() {
    let v = Vec3::new(1.0f32, 2.0, 3.0);
    let g = glam::Vec3::new(1.0, 2.0, 3.0);
    assert!(approx_eq(v[0], g[0]));
    assert!(approx_eq(v[1], g[1]));
    assert!(approx_eq(v[2], g[2]));
}

#[test]
fn index_mut() {
    let mut v = Vec3::new(1.0f32, 2.0, 3.0);
    v[0] = 9.0;
    v[1] = 8.0;
    v[2] = 7.0;
    let mut g = glam::Vec3::new(1.0, 2.0, 3.0);
    g[0] = 9.0;
    g[1] = 8.0;
    g[2] = 7.0;
    assert!(vec3_approx_eq(v, wv3(g)));
}

// ---- Display ----

#[test]
fn display() {
    let v = Vec3::new(1.0f32, 2.0, 3.0);
    let s = format!("{}", v);
    assert!(s.contains("Vec3"));
}

// ---- Comprehensive comparisons ----

#[test]
fn comprehensive_cross_vs_glam() {
    let pairs = [
        (Vec3::new(1.0f32, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
        (Vec3::new(2.0, 3.0, 4.0), Vec3::new(5.0, 6.0, 7.0)),
        (Vec3::new(-1.0, 2.0, -3.0), Vec3::new(4.0, -5.0, 6.0)),
    ];
    for (a, b) in pairs {
        let c = a.cross(b);
        let gc = gv3(a).cross(gv3(b));
        assert!(vec3_approx_eq(c, wv3(gc)));
    }
}

#[test]
fn comprehensive_dot_vs_glam() {
    let pairs = [
        (Vec3::new(1.0f32, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
        (Vec3::new(3.0, 4.0, 5.0), Vec3::new(-4.0, 3.0, 0.0)),
        (Vec3::new(-1.0, -2.0, -3.0), Vec3::new(-4.0, -5.0, -6.0)),
    ];
    for (a, b) in pairs {
        assert!(approx_eq(a.dot(b), gv3(a).dot(gv3(b))));
    }
}

#[test]
fn comprehensive_normalize_vs_glam() {
    let vecs = [
        Vec3::new(1.0f32, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(3.0, 4.0, 5.0),
        Vec3::new(-7.0, 24.0, 0.0),
    ];
    for v in vecs {
        assert!(vec3_approx_eq(v.normalize(), wv3(gv3(v).normalize())));
    }
}

#[test]
fn truncate_to_vec2() {
    let v = Vec3::new(1.0f32, 2.0, 3.0);
    let t: Vec2<f32> = v.truncate();
    let g = glam::Vec3::new(1.0, 2.0, 3.0);
    let gt = g.truncate();
    assert!(approx_eq(t.x, gt.x));
    assert!(approx_eq(t.y, gt.y));
}
