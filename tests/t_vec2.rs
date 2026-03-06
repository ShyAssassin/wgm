use wgm::Vec2;
use wgm::units::Rad;

const EPSILON: f32 = 1e-5;

fn approx_eq(a: f32, b: f32) -> bool {
    return (a - b).abs() < EPSILON;
}

fn vec2_approx_eq(a: Vec2<f32>, b: Vec2<f32>) -> bool {
    return approx_eq(a.x, b.x) && approx_eq(a.y, b.y);
}

fn gv2(v: Vec2<f32>) -> glam::Vec2 {
    return glam::Vec2::new(v.x, v.y);
}

fn wv2(v: glam::Vec2) -> Vec2<f32> {
    return Vec2::new(v.x, v.y);
}

// ---- Construction ----

#[test]
fn new() {
    let v = Vec2::new(1.0f32, 2.0);
    let g = glam::Vec2::new(1.0, 2.0);
    assert!(approx_eq(v.x, g.x));
    assert!(approx_eq(v.y, g.y));
}

#[test]
fn splat() {
    let v = Vec2::splat(3.0f32);
    let g = glam::Vec2::splat(3.0);
    assert!(vec2_approx_eq(v, wv2(g)));
}

#[test]
fn zero() {
    let v = Vec2::<f32>::zero();
    let g = glam::Vec2::ZERO;
    assert!(vec2_approx_eq(v, wv2(g)));
}

#[test]
fn one() {
    let v = Vec2::<f32>::one();
    let g = glam::Vec2::ONE;
    assert!(vec2_approx_eq(v, wv2(g)));
}

#[test]
fn unit_x() {
    let v = Vec2::<f32>::unit_x();
    let g = glam::Vec2::X;
    assert!(vec2_approx_eq(v, wv2(g)));
}

#[test]
fn unit_y() {
    let v = Vec2::<f32>::unit_y();
    let g = glam::Vec2::Y;
    assert!(vec2_approx_eq(v, wv2(g)));
}

// ---- Geometry ----

#[test]
fn length() {
    let v = Vec2::new(3.0f32, 4.0);
    let g = glam::Vec2::new(3.0, 4.0);
    assert!(approx_eq(v.length(), g.length()));
}

#[test]
fn length_squared() {
    let v = Vec2::new(3.0f32, 4.0);
    let g = glam::Vec2::new(3.0, 4.0);
    assert!(approx_eq(v.length_squared(), g.length_squared()));
}

#[test]
fn distance() {
    let a = Vec2::new(1.0f32, 2.0);
    let b = Vec2::new(4.0, 6.0);
    let ga = glam::Vec2::new(1.0, 2.0);
    let gb = glam::Vec2::new(4.0, 6.0);
    assert!(approx_eq(a.distance(b), ga.distance(gb)));
}

#[test]
fn distance_squared() {
    let a = Vec2::new(1.0f32, 2.0);
    let b = Vec2::new(4.0, 6.0);
    let ga = glam::Vec2::new(1.0, 2.0);
    let gb = glam::Vec2::new(4.0, 6.0);
    assert!(approx_eq(a.distance_squared(b), ga.distance_squared(gb)));
}

#[test]
fn normalize() {
    let v = Vec2::new(3.0f32, 4.0);
    let g = glam::Vec2::new(3.0, 4.0);
    let n = v.normalize();
    let gn = g.normalize();
    assert!(vec2_approx_eq(n, wv2(gn)));
}

#[test]
fn dot() {
    let a = Vec2::new(1.0f32, 2.0);
    let b = Vec2::new(3.0, 4.0);
    let ga = glam::Vec2::new(1.0, 2.0);
    let gb = glam::Vec2::new(3.0, 4.0);
    assert!(approx_eq(a.dot(b), ga.dot(gb)));
}

#[test]
fn perp() {
    let v = Vec2::new(1.0f32, 2.0);
    let g = glam::Vec2::new(1.0, 2.0);
    let p = v.perp();
    let gp = g.perp();
    assert!(vec2_approx_eq(p, wv2(gp)));
}

#[test]
fn perp_dot() {
    let a = Vec2::new(1.0f32, 2.0);
    let b = Vec2::new(3.0, 4.0);
    let ga = glam::Vec2::new(1.0, 2.0);
    let gb = glam::Vec2::new(3.0, 4.0);
    assert!(approx_eq(a.perp_dot(b), ga.perp_dot(gb)));
}

#[test]
fn angle_between() {
    let a = Vec2::new(1.0f32, 0.0);
    let b = Vec2::new(0.0, 1.0);
    let ga = glam::Vec2::new(1.0, 0.0);
    let gb = glam::Vec2::new(0.0, 1.0);
    assert!(approx_eq(a.angle_between(b).inner(), ga.angle_to(gb)));
}

#[test]
fn angle_between_arbitrary() {
    let a = Vec2::new(3.0f32, 4.0);
    let b = Vec2::new(-1.0, 2.0);
    let ga = glam::Vec2::new(3.0, 4.0);
    let gb = glam::Vec2::new(-1.0, 2.0);
    assert!(approx_eq(a.angle_between(b).inner(), ga.angle_to(gb)));
}

#[test]
fn angle() {
    let v = Vec2::new(1.0f32, 1.0);
    let g = glam::Vec2::new(1.0, 1.0);
    assert!(approx_eq(v.angle().inner(), g.to_angle()));
}

#[test]
fn angle_negative() {
    let v = Vec2::new(-1.0f32, 0.0);
    let g = glam::Vec2::new(-1.0, 0.0);
    assert!(approx_eq(v.angle().inner(), g.to_angle()));
}

// ---- Interpolation ----

#[test]
fn lerp() {
    let a = Vec2::new(0.0f32, 0.0);
    let b = Vec2::new(10.0, 20.0);
    let ga = glam::Vec2::new(0.0, 0.0);
    let gb = glam::Vec2::new(10.0, 20.0);
    assert!(vec2_approx_eq(a.lerp(b, 0.5), wv2(ga.lerp(gb, 0.5))));
}

#[test]
fn lerp_endpoints() {
    let a = Vec2::new(1.0f32, 2.0);
    let b = Vec2::new(3.0, 4.0);
    let ga = glam::Vec2::new(1.0, 2.0);
    let gb = glam::Vec2::new(3.0, 4.0);
    assert!(vec2_approx_eq(a.lerp(b, 0.0), wv2(ga.lerp(gb, 0.0))));
    assert!(vec2_approx_eq(a.lerp(b, 1.0), wv2(ga.lerp(gb, 1.0))));
}

// ---- Component-wise math ----

#[test]
fn min() {
    let a = Vec2::new(1.0f32, 4.0);
    let b = Vec2::new(3.0, 2.0);
    let ga = glam::Vec2::new(1.0, 4.0);
    let gb = glam::Vec2::new(3.0, 2.0);
    assert!(vec2_approx_eq(a.min(b), wv2(ga.min(gb))));
}

#[test]
fn max() {
    let a = Vec2::new(1.0f32, 4.0);
    let b = Vec2::new(3.0, 2.0);
    let ga = glam::Vec2::new(1.0, 4.0);
    let gb = glam::Vec2::new(3.0, 2.0);
    assert!(vec2_approx_eq(a.max(b), wv2(ga.max(gb))));
}

#[test]
fn clamp() {
    let v = Vec2::new(-1.0f32, 5.0);
    let min = Vec2::new(0.0, 0.0);
    let max = Vec2::new(3.0, 3.0);
    let g = glam::Vec2::new(-1.0, 5.0);
    let gmin = glam::Vec2::new(0.0, 0.0);
    let gmax = glam::Vec2::new(3.0, 3.0);
    assert!(vec2_approx_eq(v.clamp(min, max), wv2(g.clamp(gmin, gmax))));
}

#[test]
fn abs() {
    let v = Vec2::new(-3.0f32, -4.0);
    let g = glam::Vec2::new(-3.0, -4.0);
    assert!(vec2_approx_eq(v.abs(), wv2(g.abs())));
}

#[test]
fn signum() {
    let v = Vec2::new(-3.0f32, 4.0);
    let g = glam::Vec2::new(-3.0, 4.0);
    assert!(vec2_approx_eq(v.signum(), wv2(g.signum())));
}

#[test]
fn recip() {
    let v = Vec2::new(2.0f32, 4.0);
    let g = glam::Vec2::new(2.0, 4.0);
    assert!(vec2_approx_eq(v.recip(), wv2(g.recip())));
}

#[test]
fn floor() {
    let v = Vec2::new(1.7f32, -2.3);
    let g = glam::Vec2::new(1.7, -2.3);
    assert!(vec2_approx_eq(v.floor(), wv2(g.floor())));
}

#[test]
fn ceil() {
    let v = Vec2::new(1.2f32, -2.8);
    let g = glam::Vec2::new(1.2, -2.8);
    assert!(vec2_approx_eq(v.ceil(), wv2(g.ceil())));
}

#[test]
fn round() {
    let v = Vec2::new(1.5f32, -2.5);
    let g = glam::Vec2::new(1.5, -2.5);
    assert!(vec2_approx_eq(v.round(), wv2(g.round())));
}

#[test]
fn fract() {
    let v = Vec2::new(1.7f32, -2.3);
    let g = glam::Vec2::new(1.7, -2.3);
    assert!(vec2_approx_eq(v.fract(), wv2(g.fract())));
}

#[test]
fn powf() {
    let v = Vec2::new(2.0f32, 3.0);
    let g = glam::Vec2::new(2.0, 3.0);
    assert!(vec2_approx_eq(v.powf(2.0), wv2(g.powf(2.0))));
}

// ---- Reduction ----

#[test]
fn sum() {
    let v = Vec2::new(3.0f32, 4.0);
    let g = glam::Vec2::new(3.0, 4.0);
    assert!(approx_eq(v.sum(), g.element_sum()));
}

#[test]
fn product() {
    let v = Vec2::new(3.0f32, 4.0);
    let g = glam::Vec2::new(3.0, 4.0);
    assert!(approx_eq(v.product(), g.element_product()));
}

#[test]
fn min_element() {
    let v = Vec2::new(3.0f32, 1.0);
    let g = glam::Vec2::new(3.0, 1.0);
    assert!(approx_eq(v.min_element(), g.min_element()));
}

#[test]
fn max_element() {
    let v = Vec2::new(3.0f32, 7.0);
    let g = glam::Vec2::new(3.0, 7.0);
    assert!(approx_eq(v.max_element(), g.max_element()));
}

// ---- Reflection / Projection ----

#[test]
fn project_onto() {
    let v = Vec2::new(3.0f32, 4.0);
    let onto = Vec2::new(1.0, 0.0);
    let g = glam::Vec2::new(3.0, 4.0);
    let gonto = glam::Vec2::new(1.0, 0.0);
    assert!(vec2_approx_eq(v.project_onto(onto), wv2(g.project_onto(gonto))));
}

#[test]
fn project_onto_diagonal() {
    let v = Vec2::new(3.0f32, 4.0);
    let onto = Vec2::new(1.0, 1.0);
    let g = glam::Vec2::new(3.0, 4.0);
    let gonto = glam::Vec2::new(1.0, 1.0);
    assert!(vec2_approx_eq(v.project_onto(onto), wv2(g.project_onto(gonto))));
}

#[test]
fn reject_from() {
    let v = Vec2::new(3.0f32, 4.0);
    let from = Vec2::new(1.0, 0.0);
    let g = glam::Vec2::new(3.0, 4.0);
    let gfrom = glam::Vec2::new(1.0, 0.0);
    assert!(vec2_approx_eq(v.reject_from(from), wv2(g.reject_from(gfrom))));
}

#[test]
fn reject_from_diagonal() {
    let v = Vec2::new(3.0f32, 4.0);
    let from = Vec2::new(1.0, 1.0);
    let g = glam::Vec2::new(3.0, 4.0);
    let gfrom = glam::Vec2::new(1.0, 1.0);
    assert!(vec2_approx_eq(v.reject_from(from), wv2(g.reject_from(gfrom))));
}

#[test]
fn reflect() {
    // reflect(v,n) = v - 2*dot(v,n)*n
    let v = Vec2::new(1.0f32, -1.0);
    let n = Vec2::new(0.0, 1.0);
    let expected = gv2(v) - 2.0 * gv2(v).dot(gv2(n)) * gv2(n);
    assert!(vec2_approx_eq(v.reflect(n), wv2(expected)));
}

// ---- Rotate ----

#[test]
fn rotate() {
    let v = Vec2::new(1.0f32, 0.0);
    let angle = std::f32::consts::FRAC_PI_2;
    let g = glam::Vec2::new(1.0, 0.0);
    let rot = glam::Vec2::from_angle(angle);
    let result = v.rotate(Rad(angle));
    let gresult = rot.rotate(g);
    assert!(vec2_approx_eq(result, wv2(gresult)));
}

#[test]
fn rotate_arbitrary() {
    let v = Vec2::new(3.0f32, 4.0);
    let angle = 1.0f32;
    let g = glam::Vec2::new(3.0, 4.0);
    let rot = glam::Vec2::from_angle(angle);
    let result = v.rotate(Rad(angle));
    let gresult = rot.rotate(g);
    assert!(vec2_approx_eq(result, wv2(gresult)));
}

// ---- Extend ----

#[test]
fn extend() {
    let v = Vec2::new(1.0f32, 2.0);
    let g = glam::Vec2::new(1.0, 2.0);
    let ext = v.extend(3.0);
    let gext = g.extend(3.0);
    assert!(approx_eq(ext.x, gext.x));
    assert!(approx_eq(ext.y, gext.y));
    assert!(approx_eq(ext.z, gext.z));
}

// ---- Mul-Add ----

#[test]
fn mul_add() {
    let v = Vec2::new(1.0f32, 2.0);
    let a = Vec2::new(3.0, 4.0);
    let b = Vec2::new(5.0, 6.0);
    let g = glam::Vec2::new(1.0, 2.0);
    let ga = glam::Vec2::new(3.0, 4.0);
    let gb = glam::Vec2::new(5.0, 6.0);
    assert!(vec2_approx_eq(v.mul_add(a, b), wv2(g.mul_add(ga, gb))));
}

// ---- Operators ----

#[test]
fn add() {
    let a = Vec2::new(1.0f32, 2.0);
    let b = Vec2::new(3.0, 4.0);
    let ga = glam::Vec2::new(1.0, 2.0);
    let gb = glam::Vec2::new(3.0, 4.0);
    assert!(vec2_approx_eq(a + b, wv2(ga + gb)));
}

#[test]
fn sub() {
    let a = Vec2::new(5.0f32, 6.0);
    let b = Vec2::new(1.0, 2.0);
    let ga = glam::Vec2::new(5.0, 6.0);
    let gb = glam::Vec2::new(1.0, 2.0);
    assert!(vec2_approx_eq(a - b, wv2(ga - gb)));
}

#[test]
fn mul() {
    let a = Vec2::new(2.0f32, 3.0);
    let b = Vec2::new(4.0, 5.0);
    let ga = glam::Vec2::new(2.0, 3.0);
    let gb = glam::Vec2::new(4.0, 5.0);
    assert!(vec2_approx_eq(a * b, wv2(ga * gb)));
}

#[test]
fn div() {
    let a = Vec2::new(10.0f32, 20.0);
    let b = Vec2::new(2.0, 5.0);
    let ga = glam::Vec2::new(10.0, 20.0);
    let gb = glam::Vec2::new(2.0, 5.0);
    assert!(vec2_approx_eq(a / b, wv2(ga / gb)));
}

#[test]
fn mul_scalar() {
    let v = Vec2::new(2.0f32, 3.0);
    let g = glam::Vec2::new(2.0, 3.0);
    assert!(vec2_approx_eq(v * 5.0, wv2(g * 5.0)));
}

#[test]
fn div_scalar() {
    let v = Vec2::new(10.0f32, 20.0);
    let g = glam::Vec2::new(10.0, 20.0);
    assert!(vec2_approx_eq(v / 5.0, wv2(g / 5.0)));
}

#[test]
fn neg() {
    let v = Vec2::new(1.0f32, -2.0);
    let g = glam::Vec2::new(1.0, -2.0);
    assert!(vec2_approx_eq(-v, wv2(-g)));
}

// ---- Assign Ops ----

#[test]
fn add_assign() {
    let mut v = Vec2::new(1.0f32, 2.0);
    v += Vec2::new(3.0, 4.0);
    let mut g = glam::Vec2::new(1.0, 2.0);
    g += glam::Vec2::new(3.0, 4.0);
    assert!(vec2_approx_eq(v, wv2(g)));
}

#[test]
fn sub_assign() {
    let mut v = Vec2::new(5.0f32, 6.0);
    v -= Vec2::new(1.0, 2.0);
    let mut g = glam::Vec2::new(5.0, 6.0);
    g -= glam::Vec2::new(1.0, 2.0);
    assert!(vec2_approx_eq(v, wv2(g)));
}

#[test]
fn mul_assign() {
    let mut v = Vec2::new(2.0f32, 3.0);
    v *= 4.0f32;
    let mut g = glam::Vec2::new(2.0, 3.0);
    g *= 4.0;
    assert!(vec2_approx_eq(v, wv2(g)));
}

#[test]
fn div_assign() {
    let mut v = Vec2::new(10.0f32, 20.0);
    v /= 5.0f32;
    let mut g = glam::Vec2::new(10.0, 20.0);
    g /= 5.0;
    assert!(vec2_approx_eq(v, wv2(g)));
}

// ---- Boolean checks ----

#[test]
fn is_nan() {
    let v = Vec2::new(f32::NAN, 0.0);
    assert!(v.is_nan());
    assert!(!Vec2::new(1.0f32, 2.0).is_nan());
}

#[test]
fn is_finite() {
    assert!(Vec2::new(1.0f32, 2.0).is_finite());
    assert!(!Vec2::new(f32::INFINITY, 0.0).is_finite());
}

#[test]
fn is_infinite() {
    assert!(!Vec2::new(1.0f32, 2.0).is_infinite());
    assert!(Vec2::new(f32::INFINITY, 0.0).is_infinite());
}

#[test]
fn any_zero() {
    assert!(Vec2::new(0.0f32, 1.0).any_zero());
    assert!(!Vec2::new(1.0f32, 2.0).any_zero());
}

#[test]
fn all_zero() {
    assert!(Vec2::new(0.0f32, 0.0).all_zero());
    assert!(!Vec2::new(0.0f32, 1.0).all_zero());
}

// ---- Conversions ----

#[test]
fn to_array() {
    let v = Vec2::new(1.0f32, 2.0);
    let g = glam::Vec2::new(1.0, 2.0);
    assert_eq!(v.to_array(), g.to_array());
}

#[test]
fn from_array() {
    let arr = [1.0f32, 2.0];
    let v: Vec2<f32> = arr.into();
    let g = glam::Vec2::from_array(arr);
    assert!(vec2_approx_eq(v, wv2(g)));
}

#[test]
fn from_tuple() {
    let t = (1.0f32, 2.0);
    let v: Vec2<f32> = t.into();
    let g = glam::Vec2::from(t);
    assert!(vec2_approx_eq(v, wv2(g)));
}

#[test]
fn into_tuple() {
    let v = Vec2::new(1.0f32, 2.0);
    let t: (f32, f32) = v.into();
    let g = glam::Vec2::new(1.0, 2.0);
    let gt: (f32, f32) = g.into();
    assert_eq!(t, gt);
}

// ---- Index ----

#[test]
fn index() {
    let v = Vec2::new(1.0f32, 2.0);
    let g = glam::Vec2::new(1.0, 2.0);
    assert!(approx_eq(v[0], g[0]));
    assert!(approx_eq(v[1], g[1]));
}

#[test]
fn index_mut() {
    let mut v = Vec2::new(1.0f32, 2.0);
    v[0] = 9.0;
    v[1] = 8.0;
    let mut g = glam::Vec2::new(1.0, 2.0);
    g[0] = 9.0;
    g[1] = 8.0;
    assert!(vec2_approx_eq(v, wv2(g)));
}

// ---- Display ----

#[test]
fn display() {
    let v = Vec2::new(1.0f32, 2.0);
    let s = format!("{}", v);
    assert!(s.contains("Vec2"));
}

// ---- Comprehensive comparison with varied inputs ----

#[test]
fn comprehensive_dot_vs_glam() {
    let pairs = [
        (Vec2::new(1.0f32, 0.0), Vec2::new(0.0, 1.0)),
        (Vec2::new(3.0, 4.0), Vec2::new(-4.0, 3.0)),
        (Vec2::new(-1.0, -2.0), Vec2::new(-3.0, -4.0)),
        (Vec2::new(0.5, 0.7), Vec2::new(0.3, 0.9)),
    ];
    for (a, b) in pairs {
        let ga = gv2(a);
        let gb = gv2(b);
        assert!(approx_eq(a.dot(b), ga.dot(gb)));
    }
}

#[test]
fn comprehensive_normalize_vs_glam() {
    let vecs = [
        Vec2::new(1.0f32, 0.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(3.0, 4.0),
        Vec2::new(-7.0, 24.0),
        Vec2::new(0.001, 0.001),
    ];
    for v in vecs {
        let n = v.normalize();
        let gn = gv2(v).normalize();
        assert!(vec2_approx_eq(n, wv2(gn)));
    }
}

#[test]
fn comprehensive_length_vs_glam() {
    let vecs = [
        Vec2::new(0.0f32, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(3.0, 4.0),
        Vec2::new(-5.0, 12.0),
    ];
    for v in vecs {
        assert!(approx_eq(v.length(), gv2(v).length()));
        assert!(approx_eq(v.length_squared(), gv2(v).length_squared()));
    }
}
