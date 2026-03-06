use wgm::Vec4;

const EPSILON: f32 = 1e-5;

fn approx_eq(a: f32, b: f32) -> bool {
    return (a - b).abs() < EPSILON;
}

fn vec4_approx_eq(a: Vec4<f32>, b: Vec4<f32>) -> bool {
    return approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z) && approx_eq(a.w, b.w);
}

fn gv4(v: Vec4<f32>) -> glam::Vec4 {
    return glam::Vec4::new(v.x, v.y, v.z, v.w);
}

fn wv4(v: glam::Vec4) -> Vec4<f32> {
    return Vec4::new(v.x, v.y, v.z, v.w);
}

// ---- Construction ----

#[test]
fn new() {
    let v = Vec4::new(1.0f32, 2.0, 3.0, 4.0);
    let g = glam::Vec4::new(1.0, 2.0, 3.0, 4.0);
    assert!(approx_eq(v.x, g.x));
    assert!(approx_eq(v.y, g.y));
    assert!(approx_eq(v.z, g.z));
    assert!(approx_eq(v.w, g.w));
}

#[test]
fn splat() {
    let v = Vec4::splat(5.0f32);
    let g = glam::Vec4::splat(5.0);
    assert!(vec4_approx_eq(v, wv4(g)));
}

#[test]
fn zero() {
    let v = Vec4::<f32>::zero();
    let g = glam::Vec4::ZERO;
    assert!(vec4_approx_eq(v, wv4(g)));
}

#[test]
fn one() {
    let v = Vec4::<f32>::one();
    let g = glam::Vec4::ONE;
    assert!(vec4_approx_eq(v, wv4(g)));
}

#[test]
fn unit_x() {
    let v = Vec4::<f32>::unit_x();
    let g = glam::Vec4::X;
    assert!(vec4_approx_eq(v, wv4(g)));
}

#[test]
fn unit_y() {
    let v = Vec4::<f32>::unit_y();
    let g = glam::Vec4::Y;
    assert!(vec4_approx_eq(v, wv4(g)));
}

#[test]
fn unit_z() {
    let v = Vec4::<f32>::unit_z();
    let g = glam::Vec4::Z;
    assert!(vec4_approx_eq(v, wv4(g)));
}

#[test]
fn unit_w() {
    let v = Vec4::<f32>::unit_w();
    let g = glam::Vec4::W;
    assert!(vec4_approx_eq(v, wv4(g)));
}

// ---- Geometry ----

#[test]
fn length() {
    let v = Vec4::new(1.0f32, 2.0, 3.0, 4.0);
    let g = glam::Vec4::new(1.0, 2.0, 3.0, 4.0);
    assert!(approx_eq(v.length(), g.length()));
}

#[test]
fn length_squared() {
    let v = Vec4::new(1.0f32, 2.0, 3.0, 4.0);
    let g = glam::Vec4::new(1.0, 2.0, 3.0, 4.0);
    assert!(approx_eq(v.length_squared(), g.length_squared()));
}

#[test]
fn distance() {
    let a = Vec4::new(1.0f32, 2.0, 3.0, 4.0);
    let b = Vec4::new(5.0, 6.0, 7.0, 8.0);
    let ga = glam::Vec4::new(1.0, 2.0, 3.0, 4.0);
    let gb = glam::Vec4::new(5.0, 6.0, 7.0, 8.0);
    assert!(approx_eq(a.distance(b), ga.distance(gb)));
}

#[test]
fn distance_squared() {
    let a = Vec4::new(1.0f32, 2.0, 3.0, 4.0);
    let b = Vec4::new(5.0, 6.0, 7.0, 8.0);
    let ga = glam::Vec4::new(1.0, 2.0, 3.0, 4.0);
    let gb = glam::Vec4::new(5.0, 6.0, 7.0, 8.0);
    assert!(approx_eq(a.distance_squared(b), ga.distance_squared(gb)));
}

#[test]
fn normalize() {
    let v = Vec4::new(1.0f32, 2.0, 3.0, 4.0);
    let g = glam::Vec4::new(1.0, 2.0, 3.0, 4.0);
    assert!(vec4_approx_eq(v.normalize(), wv4(g.normalize())));
}

#[test]
fn dot() {
    let a = Vec4::new(1.0f32, 2.0, 3.0, 4.0);
    let b = Vec4::new(5.0, 6.0, 7.0, 8.0);
    let ga = glam::Vec4::new(1.0, 2.0, 3.0, 4.0);
    let gb = glam::Vec4::new(5.0, 6.0, 7.0, 8.0);
    assert!(approx_eq(a.dot(b), ga.dot(gb)));
}

// ---- Interpolation ----

#[test]
fn lerp() {
    let a = Vec4::new(0.0f32, 0.0, 0.0, 0.0);
    let b = Vec4::new(10.0, 20.0, 30.0, 40.0);
    let ga = glam::Vec4::new(0.0, 0.0, 0.0, 0.0);
    let gb = glam::Vec4::new(10.0, 20.0, 30.0, 40.0);
    assert!(vec4_approx_eq(a.lerp(b, 0.5), wv4(ga.lerp(gb, 0.5))));
}

#[test]
fn lerp_endpoints() {
    let a = Vec4::new(1.0f32, 2.0, 3.0, 4.0);
    let b = Vec4::new(5.0, 6.0, 7.0, 8.0);
    let ga = glam::Vec4::new(1.0, 2.0, 3.0, 4.0);
    let gb = glam::Vec4::new(5.0, 6.0, 7.0, 8.0);
    assert!(vec4_approx_eq(a.lerp(b, 0.0), wv4(ga.lerp(gb, 0.0))));
    assert!(vec4_approx_eq(a.lerp(b, 1.0), wv4(ga.lerp(gb, 1.0))));
}

// ---- Component-wise math ----

#[test]
fn min() {
    let a = Vec4::new(1.0f32, 5.0, 3.0, 7.0);
    let b = Vec4::new(4.0, 2.0, 6.0, 0.0);
    let ga = glam::Vec4::new(1.0, 5.0, 3.0, 7.0);
    let gb = glam::Vec4::new(4.0, 2.0, 6.0, 0.0);
    assert!(vec4_approx_eq(a.min(b), wv4(ga.min(gb))));
}

#[test]
fn max() {
    let a = Vec4::new(1.0f32, 5.0, 3.0, 7.0);
    let b = Vec4::new(4.0, 2.0, 6.0, 0.0);
    let ga = glam::Vec4::new(1.0, 5.0, 3.0, 7.0);
    let gb = glam::Vec4::new(4.0, 2.0, 6.0, 0.0);
    assert!(vec4_approx_eq(a.max(b), wv4(ga.max(gb))));
}

#[test]
fn clamp() {
    let v = Vec4::new(-1.0f32, 5.0, 2.0, 10.0);
    let min = Vec4::new(0.0, 0.0, 0.0, 0.0);
    let max = Vec4::new(3.0, 3.0, 3.0, 3.0);
    let g = glam::Vec4::new(-1.0, 5.0, 2.0, 10.0);
    let gmin = glam::Vec4::new(0.0, 0.0, 0.0, 0.0);
    let gmax = glam::Vec4::new(3.0, 3.0, 3.0, 3.0);
    assert!(vec4_approx_eq(v.clamp(min, max), wv4(g.clamp(gmin, gmax))));
}

#[test]
fn abs() {
    let v = Vec4::new(-3.0f32, -4.0, -5.0, -6.0);
    let g = glam::Vec4::new(-3.0, -4.0, -5.0, -6.0);
    assert!(vec4_approx_eq(v.abs(), wv4(g.abs())));
}

#[test]
fn signum() {
    let v = Vec4::new(-3.0f32, 4.0, 0.0, -1.0);
    let g = glam::Vec4::new(-3.0, 4.0, 0.0, -1.0);
    assert!(vec4_approx_eq(v.signum(), wv4(g.signum())));
}

#[test]
fn recip() {
    let v = Vec4::new(2.0f32, 4.0, 8.0, 16.0);
    let g = glam::Vec4::new(2.0, 4.0, 8.0, 16.0);
    assert!(vec4_approx_eq(v.recip(), wv4(g.recip())));
}

#[test]
fn floor() {
    let v = Vec4::new(1.7f32, -2.3, 3.9, -0.1);
    let g = glam::Vec4::new(1.7, -2.3, 3.9, -0.1);
    assert!(vec4_approx_eq(v.floor(), wv4(g.floor())));
}

#[test]
fn ceil() {
    let v = Vec4::new(1.2f32, -2.8, 3.1, -0.9);
    let g = glam::Vec4::new(1.2, -2.8, 3.1, -0.9);
    assert!(vec4_approx_eq(v.ceil(), wv4(g.ceil())));
}

#[test]
fn round() {
    let v = Vec4::new(1.3f32, -2.7, 3.4, -0.6);
    let g = glam::Vec4::new(1.3, -2.7, 3.4, -0.6);
    assert!(vec4_approx_eq(v.round(), wv4(g.round())));
}

#[test]
fn fract() {
    let v = Vec4::new(1.7f32, -2.3, 3.9, -0.1);
    let g = glam::Vec4::new(1.7, -2.3, 3.9, -0.1);
    assert!(vec4_approx_eq(v.fract(), wv4(g.fract())));
}

#[test]
fn powf() {
    let v = Vec4::new(2.0f32, 3.0, 4.0, 5.0);
    let g = glam::Vec4::new(2.0, 3.0, 4.0, 5.0);
    assert!(vec4_approx_eq(v.powf(2.0), wv4(g.powf(2.0))));
}

// ---- Reduction ----

#[test]
fn sum() {
    let v = Vec4::new(1.0f32, 2.0, 3.0, 4.0);
    let g = glam::Vec4::new(1.0, 2.0, 3.0, 4.0);
    assert!(approx_eq(v.sum(), g.element_sum()));
}

#[test]
fn product() {
    let v = Vec4::new(2.0f32, 3.0, 4.0, 5.0);
    let g = glam::Vec4::new(2.0, 3.0, 4.0, 5.0);
    assert!(approx_eq(v.product(), g.element_product()));
}

#[test]
fn min_element() {
    let v = Vec4::new(3.0f32, 1.0, 5.0, 2.0);
    let g = glam::Vec4::new(3.0, 1.0, 5.0, 2.0);
    assert!(approx_eq(v.min_element(), g.min_element()));
}

#[test]
fn max_element() {
    let v = Vec4::new(3.0f32, 7.0, 5.0, 2.0);
    let g = glam::Vec4::new(3.0, 7.0, 5.0, 2.0);
    assert!(approx_eq(v.max_element(), g.max_element()));
}

// ---- Reflection / Projection ----

#[test]
fn project_onto() {
    let v = Vec4::new(3.0f32, 4.0, 0.0, 0.0);
    let onto = Vec4::new(1.0, 0.0, 0.0, 0.0);
    let g = glam::Vec4::new(3.0, 4.0, 0.0, 0.0);
    let gonto = glam::Vec4::new(1.0, 0.0, 0.0, 0.0);
    assert!(vec4_approx_eq(v.project_onto(onto), wv4(g.project_onto(gonto))));
}

#[test]
fn reject_from() {
    let v = Vec4::new(3.0f32, 4.0, 0.0, 0.0);
    let from = Vec4::new(1.0, 0.0, 0.0, 0.0);
    let g = glam::Vec4::new(3.0, 4.0, 0.0, 0.0);
    let gfrom = glam::Vec4::new(1.0, 0.0, 0.0, 0.0);
    assert!(vec4_approx_eq(v.reject_from(from), wv4(g.reject_from(gfrom))));
}

#[test]
fn reflect() {
    let v = Vec4::new(1.0f32, -1.0, 0.0, 0.0);
    let n = Vec4::new(0.0, 1.0, 0.0, 0.0);
    // reflect(v,n) = v - 2*dot(v,n)*n
    let expected = gv4(v) - 2.0 * gv4(v).dot(gv4(n)) * gv4(n);
    assert!(vec4_approx_eq(v.reflect(n), wv4(expected)));
}

// ---- Truncate / Extend ----

#[test]
fn truncate() {
    let v = Vec4::new(1.0f32, 2.0, 3.0, 4.0);
    let g = glam::Vec4::new(1.0, 2.0, 3.0, 4.0);
    let t = v.truncate();
    let gt = g.truncate();
    assert!(approx_eq(t.x, gt.x));
    assert!(approx_eq(t.y, gt.y));
    assert!(approx_eq(t.z, gt.z));
}

// ---- Mul-Add ----

#[test]
fn mul_add() {
    let v = Vec4::new(1.0f32, 2.0, 3.0, 4.0);
    let a = Vec4::new(5.0, 6.0, 7.0, 8.0);
    let b = Vec4::new(9.0, 10.0, 11.0, 12.0);
    let g = glam::Vec4::new(1.0, 2.0, 3.0, 4.0);
    let ga = glam::Vec4::new(5.0, 6.0, 7.0, 8.0);
    let gb = glam::Vec4::new(9.0, 10.0, 11.0, 12.0);
    assert!(vec4_approx_eq(v.mul_add(a, b), wv4(g.mul_add(ga, gb))));
}

// ---- Operators ----

#[test]
fn add() {
    let a = Vec4::new(1.0f32, 2.0, 3.0, 4.0);
    let b = Vec4::new(5.0, 6.0, 7.0, 8.0);
    let ga = glam::Vec4::new(1.0, 2.0, 3.0, 4.0);
    let gb = glam::Vec4::new(5.0, 6.0, 7.0, 8.0);
    assert!(vec4_approx_eq(a + b, wv4(ga + gb)));
}

#[test]
fn sub() {
    let a = Vec4::new(5.0f32, 6.0, 7.0, 8.0);
    let b = Vec4::new(1.0, 2.0, 3.0, 4.0);
    let ga = glam::Vec4::new(5.0, 6.0, 7.0, 8.0);
    let gb = glam::Vec4::new(1.0, 2.0, 3.0, 4.0);
    assert!(vec4_approx_eq(a - b, wv4(ga - gb)));
}

#[test]
fn mul() {
    let a = Vec4::new(2.0f32, 3.0, 4.0, 5.0);
    let b = Vec4::new(6.0, 7.0, 8.0, 9.0);
    let ga = glam::Vec4::new(2.0, 3.0, 4.0, 5.0);
    let gb = glam::Vec4::new(6.0, 7.0, 8.0, 9.0);
    assert!(vec4_approx_eq(a * b, wv4(ga * gb)));
}

#[test]
fn div() {
    let a = Vec4::new(10.0f32, 20.0, 30.0, 40.0);
    let b = Vec4::new(2.0, 5.0, 6.0, 8.0);
    let ga = glam::Vec4::new(10.0, 20.0, 30.0, 40.0);
    let gb = glam::Vec4::new(2.0, 5.0, 6.0, 8.0);
    assert!(vec4_approx_eq(a / b, wv4(ga / gb)));
}

#[test]
fn mul_scalar() {
    let v = Vec4::new(2.0f32, 3.0, 4.0, 5.0);
    let g = glam::Vec4::new(2.0, 3.0, 4.0, 5.0);
    assert!(vec4_approx_eq(v * 5.0, wv4(g * 5.0)));
}

#[test]
fn div_scalar() {
    let v = Vec4::new(10.0f32, 20.0, 30.0, 40.0);
    let g = glam::Vec4::new(10.0, 20.0, 30.0, 40.0);
    assert!(vec4_approx_eq(v / 5.0, wv4(g / 5.0)));
}

#[test]
fn neg() {
    let v = Vec4::new(1.0f32, -2.0, 3.0, -4.0);
    let g = glam::Vec4::new(1.0, -2.0, 3.0, -4.0);
    assert!(vec4_approx_eq(-v, wv4(-g)));
}

// ---- Assign Ops ----

#[test]
fn add_assign() {
    let mut v = Vec4::new(1.0f32, 2.0, 3.0, 4.0);
    v += Vec4::new(5.0, 6.0, 7.0, 8.0);
    let mut g = glam::Vec4::new(1.0, 2.0, 3.0, 4.0);
    g += glam::Vec4::new(5.0, 6.0, 7.0, 8.0);
    assert!(vec4_approx_eq(v, wv4(g)));
}

#[test]
fn sub_assign() {
    let mut v = Vec4::new(5.0f32, 6.0, 7.0, 8.0);
    v -= Vec4::new(1.0, 2.0, 3.0, 4.0);
    let mut g = glam::Vec4::new(5.0, 6.0, 7.0, 8.0);
    g -= glam::Vec4::new(1.0, 2.0, 3.0, 4.0);
    assert!(vec4_approx_eq(v, wv4(g)));
}

#[test]
fn mul_assign() {
    let mut v = Vec4::new(2.0f32, 3.0, 4.0, 5.0);
    v *= 4.0f32;
    let mut g = glam::Vec4::new(2.0, 3.0, 4.0, 5.0);
    g *= 4.0;
    assert!(vec4_approx_eq(v, wv4(g)));
}

#[test]
fn div_assign() {
    let mut v = Vec4::new(10.0f32, 20.0, 30.0, 40.0);
    v /= 5.0f32;
    let mut g = glam::Vec4::new(10.0, 20.0, 30.0, 40.0);
    g /= 5.0;
    assert!(vec4_approx_eq(v, wv4(g)));
}

// ---- Boolean checks ----

#[test]
fn is_nan() {
    assert!(Vec4::new(f32::NAN, 0.0, 0.0, 0.0).is_nan());
    assert!(!Vec4::new(1.0f32, 2.0, 3.0, 4.0).is_nan());
}

#[test]
fn is_finite() {
    assert!(Vec4::new(1.0f32, 2.0, 3.0, 4.0).is_finite());
    assert!(!Vec4::new(f32::INFINITY, 0.0, 0.0, 0.0).is_finite());
}

#[test]
fn is_infinite() {
    assert!(!Vec4::new(1.0f32, 2.0, 3.0, 4.0).is_infinite());
    assert!(Vec4::new(f32::INFINITY, 0.0, 0.0, 0.0).is_infinite());
}

#[test]
fn any_zero() {
    assert!(Vec4::new(0.0f32, 1.0, 2.0, 3.0).any_zero());
    assert!(!Vec4::new(1.0f32, 2.0, 3.0, 4.0).any_zero());
}

#[test]
fn all_zero() {
    assert!(Vec4::new(0.0f32, 0.0, 0.0, 0.0).all_zero());
    assert!(!Vec4::new(0.0f32, 0.0, 0.0, 1.0).all_zero());
}

// ---- Conversions ----

#[test]
fn to_array() {
    let v = Vec4::new(1.0f32, 2.0, 3.0, 4.0);
    let g = glam::Vec4::new(1.0, 2.0, 3.0, 4.0);
    assert_eq!(v.to_array(), g.to_array());
}

#[test]
fn from_array() {
    let arr = [1.0f32, 2.0, 3.0, 4.0];
    let v: Vec4<f32> = arr.into();
    let g = glam::Vec4::from_array(arr);
    assert!(vec4_approx_eq(v, wv4(g)));
}

#[test]
fn from_tuple() {
    let t = (1.0f32, 2.0, 3.0, 4.0);
    let v: Vec4<f32> = t.into();
    let g = glam::Vec4::from(t);
    assert!(vec4_approx_eq(v, wv4(g)));
}

#[test]
fn into_tuple() {
    let v = Vec4::new(1.0f32, 2.0, 3.0, 4.0);
    let t: (f32, f32, f32, f32) = v.into();
    let g = glam::Vec4::new(1.0, 2.0, 3.0, 4.0);
    let gt: (f32, f32, f32, f32) = g.into();
    assert_eq!(t, gt);
}

// ---- Index ----

#[test]
fn index() {
    let v = Vec4::new(1.0f32, 2.0, 3.0, 4.0);
    let g = glam::Vec4::new(1.0, 2.0, 3.0, 4.0);
    assert!(approx_eq(v[0], g[0]));
    assert!(approx_eq(v[1], g[1]));
    assert!(approx_eq(v[2], g[2]));
    assert!(approx_eq(v[3], g[3]));
}

#[test]
fn index_mut() {
    let mut v = Vec4::new(1.0f32, 2.0, 3.0, 4.0);
    v[0] = 9.0;
    v[1] = 8.0;
    v[2] = 7.0;
    v[3] = 6.0;
    let mut g = glam::Vec4::new(1.0, 2.0, 3.0, 4.0);
    g[0] = 9.0;
    g[1] = 8.0;
    g[2] = 7.0;
    g[3] = 6.0;
    assert!(vec4_approx_eq(v, wv4(g)));
}

// ---- Display ----

#[test]
fn display() {
    let v = Vec4::new(1.0f32, 2.0, 3.0, 4.0);
    let s = format!("{}", v);
    assert!(s.contains("Vec4"));
}

// ---- Comprehensive comparisons ----

#[test]
fn comprehensive_dot_vs_glam() {
    let pairs = [
        (Vec4::new(1.0f32, 0.0, 0.0, 0.0), Vec4::new(0.0, 1.0, 0.0, 0.0)),
        (Vec4::new(1.0, 2.0, 3.0, 4.0), Vec4::new(5.0, 6.0, 7.0, 8.0)),
        (Vec4::new(-1.0, -2.0, -3.0, -4.0), Vec4::new(-5.0, -6.0, -7.0, -8.0)),
    ];
    for (a, b) in pairs {
        assert!(approx_eq(a.dot(b), gv4(a).dot(gv4(b))));
    }
}

#[test]
fn comprehensive_normalize_vs_glam() {
    let vecs = [
        Vec4::new(1.0f32, 0.0, 0.0, 0.0),
        Vec4::new(1.0, 2.0, 3.0, 4.0),
        Vec4::new(-7.0, 24.0, 0.0, 1.0),
    ];
    for v in vecs {
        assert!(vec4_approx_eq(v.normalize(), wv4(gv4(v).normalize())));
    }
}

#[test]
fn comprehensive_length_vs_glam() {
    let vecs = [
        Vec4::new(0.0f32, 0.0, 0.0, 0.0),
        Vec4::new(1.0, 0.0, 0.0, 0.0),
        Vec4::new(1.0, 2.0, 3.0, 4.0),
        Vec4::new(-5.0, 12.0, 0.0, -3.0),
    ];
    for v in vecs {
        assert!(approx_eq(v.length(), gv4(v).length()));
        assert!(approx_eq(v.length_squared(), gv4(v).length_squared()));
    }
}
