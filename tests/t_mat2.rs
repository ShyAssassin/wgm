use wgm::{Vec2, Mat2};
use wgm::units::Rad;

const EPSILON: f32 = 1e-5;

fn approx_eq(a: f32, b: f32) -> bool {
    return (a - b).abs() < EPSILON;
}

fn mat2_approx_eq(a: Mat2<f32>, b: Mat2<f32>) -> bool {
    return approx_eq(a.x.x, b.x.x)
        && approx_eq(a.x.y, b.x.y)
        && approx_eq(a.y.x, b.y.x)
        && approx_eq(a.y.y, b.y.y);
}

fn vec2_approx_eq(a: Vec2<f32>, b: Vec2<f32>) -> bool {
    return approx_eq(a.x, b.x) && approx_eq(a.y, b.y);
}

fn gm2(m: Mat2<f32>) -> glam::Mat2 {
    return glam::Mat2::from_cols(glam::Vec2::new(m.x.x, m.x.y), glam::Vec2::new(m.y.x, m.y.y));
}

fn wm2(m: glam::Mat2) -> Mat2<f32> {
    return Mat2::new(Vec2::new(m.x_axis.x, m.x_axis.y), Vec2::new(m.y_axis.x, m.y_axis.y));
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
    let m = Mat2::new(Vec2::new(1.0f32, 2.0), Vec2::new(3.0, 4.0));
    let g = glam::Mat2::from_cols(glam::Vec2::new(1.0, 2.0), glam::Vec2::new(3.0, 4.0));
    assert!(mat2_approx_eq(m, wm2(g)));
}

#[test]
fn from_cols() {
    let m = Mat2::<f32>::from_cols(1.0, 2.0, 3.0, 4.0);
    let g = glam::Mat2::from_cols(glam::Vec2::new(1.0, 2.0), glam::Vec2::new(3.0, 4.0));
    assert!(mat2_approx_eq(m, wm2(g)));
}

#[test]
fn identity() {
    let i = Mat2::<f32>::identity();
    let g = glam::Mat2::IDENTITY;
    assert!(mat2_approx_eq(i, wm2(g)));
}

#[test]
fn zero() {
    let z = Mat2::<f32>::zero();
    let g = glam::Mat2::ZERO;
    assert!(mat2_approx_eq(z, wm2(g)));
}

// ---- Determinant ----

#[test]
fn determinant() {
    let m = Mat2::<f32>::from_cols(1.0, 2.0, 3.0, 4.0);
    let g = glam::Mat2::from_cols(glam::Vec2::new(1.0, 2.0), glam::Vec2::new(3.0, 4.0));
    assert!(approx_eq(m.determinant(), g.determinant()));
}

#[test]
fn determinant_identity() {
    let i = Mat2::<f32>::identity();
    let g = glam::Mat2::IDENTITY;
    assert!(approx_eq(i.determinant(), g.determinant()));
}

#[test]
fn determinant_varied() {
    let inputs = [
        [2.0f32, 0.0, 0.0, 3.0],
        [1.0, 3.0, 2.0, 7.0],
        [-1.0, 4.0, 2.0, -3.0],
    ];
    for cols in inputs {
        let m = Mat2::<f32>::from_cols(cols[0], cols[1], cols[2], cols[3]);
        let g = glam::Mat2::from_cols(glam::Vec2::new(cols[0], cols[1]), glam::Vec2::new(cols[2], cols[3]));
        assert!(approx_eq(m.determinant(), g.determinant()));
    }
}

// ---- Transpose ----

#[test]
fn transpose() {
    let m = Mat2::<f32>::from_cols(1.0, 2.0, 3.0, 4.0);
    let g = glam::Mat2::from_cols(glam::Vec2::new(1.0, 2.0), glam::Vec2::new(3.0, 4.0));
    assert!(mat2_approx_eq(m.transpose(), wm2(g.transpose())));
}

#[test]
fn transpose_identity() {
    let i = Mat2::<f32>::identity();
    let g = glam::Mat2::IDENTITY;
    assert!(mat2_approx_eq(i.transpose(), wm2(g.transpose())));
}

// ---- Inverse ----

#[test]
fn inverse() {
    let m = Mat2::<f32>::from_cols(1.0, 2.0, 3.0, 4.0);
    let g = glam::Mat2::from_cols(glam::Vec2::new(1.0, 2.0), glam::Vec2::new(3.0, 4.0));
    let inv = m.inverse().unwrap();
    let ginv = g.inverse();
    assert!(mat2_approx_eq(inv, wm2(ginv)));
}

#[test]
fn inverse_identity_roundtrip() {
    let m = Mat2::<f32>::from_cols(1.0, 2.0, 3.0, 4.0);
    let inv = m.inverse().unwrap();
    let product = m.mul_mat2(inv);
    let g = gm2(m);
    let gproduct = g * g.inverse();
    assert!(mat2_approx_eq(product, wm2(gproduct)));
}

#[test]
fn inverse_singular() {
    let m = Mat2::<f32>::from_cols(1.0, 2.0, 2.0, 4.0);
    assert!(m.inverse().is_none());
}

// ---- Rotation / Scale ----

#[test]
fn from_angle() {
    let angle = std::f32::consts::FRAC_PI_2;
    let m = Mat2::<f32>::from_angle(Rad(angle));
    let g = glam::Mat2::from_angle(angle);
    assert!(mat2_approx_eq(m, wm2(g)));
}

#[test]
fn from_angle_mul_vec() {
    let angle = std::f32::consts::FRAC_PI_2;
    let m = Mat2::<f32>::from_angle(Rad(angle));
    let g = glam::Mat2::from_angle(angle);
    let v = Vec2::new(1.0f32, 0.0);
    let gv = glam::Vec2::new(1.0, 0.0);
    assert!(vec2_approx_eq(m.mul_vec2(v), wv2(g * gv)));
}

#[test]
fn from_scale() {
    let m = Mat2::<f32>::from_scale(Vec2::new(2.0, 3.0));
    let g = glam::Mat2::from_diagonal(glam::Vec2::new(2.0, 3.0));
    let v = Vec2::new(1.0f32, 1.0);
    let gv = glam::Vec2::new(1.0, 1.0);
    assert!(vec2_approx_eq(m.mul_vec2(v), wv2(g * gv)));
}

#[test]
fn from_scale_uniform() {
    let m = Mat2::<f32>::from_scale_uniform(5.0);
    let g = glam::Mat2::from_diagonal(glam::Vec2::splat(5.0));
    let v = Vec2::new(1.0f32, 2.0);
    let gv = glam::Vec2::new(1.0, 2.0);
    assert!(vec2_approx_eq(m.mul_vec2(v), wv2(g * gv)));
}

#[test]
fn from_scale_angle() {
    let m = Mat2::<f32>::from_scale_angle(Vec2::new(2.0, 3.0), Rad(0.0));
    let v = m.mul_vec2(Vec2::new(1.0, 1.0));
    // At angle=0, scale_angle is just a scale
    let g = glam::Mat2::from_scale_angle(glam::Vec2::new(2.0, 3.0), 0.0);
    let gv = g * glam::Vec2::new(1.0, 1.0);
    assert!(vec2_approx_eq(v, wv2(gv)));
}

#[test]
fn from_scale_angle_rotated() {
    let angle = 1.0f32;
    let scale = Vec2::new(2.0, 3.0);
    let m = Mat2::<f32>::from_scale_angle(scale, Rad(angle));
    let g = glam::Mat2::from_scale_angle(glam::Vec2::new(2.0, 3.0), angle);
    assert!(mat2_approx_eq(m, wm2(g)));
}

// ---- Multiply ----

#[test]
fn mul_vec2() {
    let m = Mat2::<f32>::from_cols(1.0, 2.0, 3.0, 4.0);
    let v = Vec2::new(5.0f32, 6.0);
    let g = glam::Mat2::from_cols(glam::Vec2::new(1.0, 2.0), glam::Vec2::new(3.0, 4.0));
    let gv = glam::Vec2::new(5.0, 6.0);
    assert!(vec2_approx_eq(m.mul_vec2(v), wv2(g * gv)));
}

#[test]
fn mul_mat2() {
    let a = Mat2::<f32>::from_cols(1.0, 2.0, 3.0, 4.0);
    let b = Mat2::<f32>::from_cols(5.0, 6.0, 7.0, 8.0);
    let c = a.mul_mat2(b);
    let ga = glam::Mat2::from_cols(glam::Vec2::new(1.0, 2.0), glam::Vec2::new(3.0, 4.0));
    let gb = glam::Mat2::from_cols(glam::Vec2::new(5.0, 6.0), glam::Vec2::new(7.0, 8.0));
    let gc = ga * gb;
    assert!(mat2_approx_eq(c, wm2(gc)));
}

#[test]
fn mul_identity() {
    let m = Mat2::<f32>::from_cols(1.0, 2.0, 3.0, 4.0);
    let i = Mat2::identity();
    let g = gm2(m);
    let gi = glam::Mat2::IDENTITY;
    assert!(mat2_approx_eq(m * i, wm2(g * gi)));
    assert!(mat2_approx_eq(i * m, wm2(gi * g)));
}

// ---- Array conversions ----

#[test]
fn to_cols_array() {
    let m = Mat2::<f32>::from_cols(1.0, 2.0, 3.0, 4.0);
    let g = glam::Mat2::from_cols(glam::Vec2::new(1.0, 2.0), glam::Vec2::new(3.0, 4.0));
    assert_eq!(m.to_cols_array(), g.to_cols_array());
}

#[test]
fn from_cols_array() {
    let arr = [1.0f32, 2.0, 3.0, 4.0];
    let m = Mat2::from_cols_array(arr);
    let g = glam::Mat2::from_cols_array(&arr);
    assert!(mat2_approx_eq(m, wm2(g)));
}

// ---- Col / Row ----

#[test]
fn col_row() {
    let m = Mat2::<f32>::from_cols(1.0, 2.0, 3.0, 4.0);
    let g = glam::Mat2::from_cols(glam::Vec2::new(1.0, 2.0), glam::Vec2::new(3.0, 4.0));
    assert!(vec2_approx_eq(*m.col(0), wv2(g.col(0))));
    assert!(vec2_approx_eq(*m.col(1), wv2(g.col(1))));
    assert!(vec2_approx_eq(m.row(0), wv2(g.row(0))));
    assert!(vec2_approx_eq(m.row(1), wv2(g.row(1))));
}

// ---- Index ----

#[test]
fn index() {
    let m = Mat2::<f32>::from_cols(1.0, 2.0, 3.0, 4.0);
    let g = glam::Mat2::from_cols(glam::Vec2::new(1.0, 2.0), glam::Vec2::new(3.0, 4.0));
    assert!(vec2_approx_eq(m[0], wv2(g.col(0))));
    assert!(vec2_approx_eq(m[1], wv2(g.col(1))));
}

#[test]
fn index_mut() {
    let mut m = Mat2::<f32>::identity();
    m[0] = Vec2::new(5.0, 6.0);
    assert!(vec2_approx_eq(m.x, Vec2::new(5.0, 6.0)));
}

// ---- Arithmetic Ops ----

#[test]
fn add_sub() {
    let a = Mat2::<f32>::from_cols(1.0, 2.0, 3.0, 4.0);
    let b = Mat2::<f32>::from_cols(5.0, 6.0, 7.0, 8.0);
    let ga = gm2(a);
    let gb = gm2(b);
    assert!(mat2_approx_eq(a + b, wm2(ga + gb)));
    assert!(mat2_approx_eq(b - a, wm2(gb - ga)));
}

#[test]
fn mul_div_scalar() {
    let m = Mat2::<f32>::from_cols(2.0, 4.0, 6.0, 8.0);
    let g = gm2(m);
    assert!(mat2_approx_eq(m * 2.0, wm2(g * 2.0)));
    assert!(mat2_approx_eq(m / 2.0, wm2(g / 2.0)));
}

#[test]
fn neg() {
    let m = Mat2::<f32>::from_cols(1.0, -2.0, 3.0, -4.0);
    let g = gm2(m);
    assert!(mat2_approx_eq(-m, wm2(-g)));
}

// ---- Assign Ops ----

#[test]
fn assign_ops() {
    let mut m = Mat2::<f32>::from_cols(1.0, 2.0, 3.0, 4.0);
    let mut g = gm2(m);

    m += Mat2::from_cols(1.0, 1.0, 1.0, 1.0);
    g += glam::Mat2::from_cols(glam::Vec2::splat(1.0), glam::Vec2::splat(1.0));
    assert!(mat2_approx_eq(m, wm2(g)));

    m -= Mat2::from_cols(1.0, 1.0, 1.0, 1.0);
    g -= glam::Mat2::from_cols(glam::Vec2::splat(1.0), glam::Vec2::splat(1.0));
    assert!(mat2_approx_eq(m, wm2(g)));

    m *= 2.0f32;
    g *= 2.0;
    assert!(mat2_approx_eq(m, wm2(g)));

    m /= 2.0f32;
    g /= 2.0;
    assert!(mat2_approx_eq(m, wm2(g)));
}

// ---- Display / Checks ----

#[test]
fn display() {
    let m = Mat2::<f32>::from_cols(1.0, 2.0, 3.0, 4.0);
    let s = format!("{}", m);
    assert!(s.contains("Mat2"));
}

#[test]
fn is_nan() {
    let m = Mat2::<f32>::from_cols(1.0, f32::NAN, 3.0, 4.0);
    assert!(m.is_nan());
    assert!(!Mat2::<f32>::identity().is_nan());
}

#[test]
fn is_finite() {
    assert!(Mat2::<f32>::identity().is_finite());
    let m = Mat2::<f32>::from_cols(1.0, f32::INFINITY, 3.0, 4.0);
    assert!(!m.is_finite());
}

// ---- Array/Tuple conversions ----

#[test]
fn from_array_conversion() {
    let cols = [Vec2::new(1.0f32, 2.0), Vec2::new(3.0, 4.0)];
    let m: Mat2<f32> = Mat2::from(cols);
    assert_eq!(m.x, cols[0]);
    assert_eq!(m.y, cols[1]);
    let arr: [Vec2<f32>; 2] = m.into();
    assert_eq!(arr, cols);
}

// ---- Comprehensive varied-input comparisons ----

#[test]
fn comprehensive_mul_vs_glam() {
    let pairs = [
        ([1.0f32, 0.0, 0.0, 1.0], [2.0f32, 3.0, 4.0, 5.0]),
        ([1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0]),
        ([-1.0, 2.0, -3.0, 4.0], [4.0, -3.0, 2.0, -1.0]),
    ];
    for (ac, bc) in pairs {
        let a = Mat2::<f32>::from_cols(ac[0], ac[1], ac[2], ac[3]);
        let b = Mat2::<f32>::from_cols(bc[0], bc[1], bc[2], bc[3]);
        let c = a * b;
        let ga = gm2(a);
        let gb = gm2(b);
        let gc = ga * gb;
        assert!(mat2_approx_eq(c, wm2(gc)));
    }
}

#[test]
fn comprehensive_inverse_vs_glam() {
    let mats = [
        [1.0f32, 2.0, 3.0, 4.0],
        [2.0, 0.0, 0.0, 3.0],
        [-1.0, 4.0, 2.0, -3.0],
        [5.0, 1.0, 2.0, 7.0],
    ];
    for cols in mats {
        let m = Mat2::<f32>::from_cols(cols[0], cols[1], cols[2], cols[3]);
        let g = gm2(m);
        let inv = m.inverse().unwrap();
        let ginv = g.inverse();
        assert!(mat2_approx_eq(inv, wm2(ginv)));
    }
}

#[test]
fn comprehensive_mul_vec_vs_glam() {
    let m = Mat2::<f32>::from_cols(1.0, 2.0, 3.0, 4.0);
    let g = gm2(m);
    let vecs = [
        Vec2::new(1.0f32, 0.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(3.0, 4.0),
        Vec2::new(-1.0, 2.0),
    ];
    for v in vecs {
        assert!(vec2_approx_eq(m.mul_vec2(v), wv2(g * gv2(v))));
    }
}
