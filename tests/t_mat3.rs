use wgm::{Vec2, Vec3, Mat2, Mat3};
use wgm::units::Rad;

const EPSILON: f32 = 1e-4;

fn approx_eq(a: f32, b: f32) -> bool {
    return (a - b).abs() < EPSILON;
}

fn mat3_approx_eq(a: Mat3<f32>, b: Mat3<f32>) -> bool {
    let aa = a.to_cols_array();
    let bb = b.to_cols_array();
    return aa.iter().zip(bb.iter()).all(|(x, y)| return approx_eq(*x, *y));
}

fn vec3_approx_eq(a: Vec3<f32>, b: Vec3<f32>) -> bool {
    return approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z);
}

fn gm3(m: Mat3<f32>) -> glam::Mat3 {
    let a = m.to_cols_array();
    return glam::Mat3::from_cols_array(&a);
}

fn wm3(m: glam::Mat3) -> Mat3<f32> {
    return Mat3::from_cols_array(m.to_cols_array());
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
    let m = Mat3::new(
        Vec3::new(1.0f32, 2.0, 3.0),
        Vec3::new(4.0, 5.0, 6.0),
        Vec3::new(7.0, 8.0, 9.0),
    );
    let g = glam::Mat3::from_cols(
        glam::Vec3::new(1.0, 2.0, 3.0),
        glam::Vec3::new(4.0, 5.0, 6.0),
        glam::Vec3::new(7.0, 8.0, 9.0),
    );
    assert!(mat3_approx_eq(m, wm3(g)));
}

#[test]
fn identity() {
    let i = Mat3::<f32>::identity();
    let g = glam::Mat3::IDENTITY;
    assert!(mat3_approx_eq(i, wm3(g)));
}

#[test]
fn zero() {
    let z = Mat3::<f32>::zero();
    let g = glam::Mat3::ZERO;
    assert!(mat3_approx_eq(z, wm3(g)));
}

// ---- Determinant ----

#[test]
fn determinant_identity() {
    let i = Mat3::<f32>::identity();
    let g = glam::Mat3::IDENTITY;
    assert!(approx_eq(i.determinant(), g.determinant()));
}

#[test]
fn determinant() {
    let m = Mat3::<f32>::from_cols(
        1.0, 0.0, 0.0,
        0.0, 2.0, 0.0,
        0.0, 0.0, 3.0,
    );
    let g = glam::Mat3::from_cols(
        glam::Vec3::new(1.0, 0.0, 0.0),
        glam::Vec3::new(0.0, 2.0, 0.0),
        glam::Vec3::new(0.0, 0.0, 3.0),
    );
    assert!(approx_eq(m.determinant(), g.determinant()));
}

#[test]
fn determinant_general() {
    let m = Mat3::<f32>::from_cols(
        2.0, 1.0, 0.0,
        0.0, 1.0, 1.0,
        1.0, 0.0, 1.0,
    );
    let g = glam::Mat3::from_cols(
        glam::Vec3::new(2.0, 1.0, 0.0),
        glam::Vec3::new(0.0, 1.0, 1.0),
        glam::Vec3::new(1.0, 0.0, 1.0),
    );
    assert!(approx_eq(m.determinant(), g.determinant()));
}

// ---- Transpose ----

#[test]
fn transpose() {
    let m = Mat3::<f32>::from_cols(
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0,
    );
    let g = glam::Mat3::from_cols(
        glam::Vec3::new(1.0, 2.0, 3.0),
        glam::Vec3::new(4.0, 5.0, 6.0),
        glam::Vec3::new(7.0, 8.0, 9.0),
    );
    assert!(mat3_approx_eq(m.transpose(), wm3(g.transpose())));
}

#[test]
fn transpose_identity() {
    let i = Mat3::<f32>::identity();
    let g = glam::Mat3::IDENTITY;
    assert!(mat3_approx_eq(i.transpose(), wm3(g.transpose())));
}

// ---- Inverse ----

#[test]
fn inverse() {
    let m = Mat3::<f32>::from_cols(
        1.0, 0.0, 0.0,
        0.0, 2.0, 0.0,
        0.0, 0.0, 4.0,
    );
    let g = glam::Mat3::from_cols(
        glam::Vec3::new(1.0, 0.0, 0.0),
        glam::Vec3::new(0.0, 2.0, 0.0),
        glam::Vec3::new(0.0, 0.0, 4.0),
    );
    let inv = m.inverse().unwrap();
    let ginv = g.inverse();
    assert!(mat3_approx_eq(inv, wm3(ginv)));
}

#[test]
fn inverse_general() {
    let m = Mat3::<f32>::from_cols(
        2.0, 1.0, 0.0,
        0.0, 1.0, 1.0,
        1.0, 0.0, 1.0,
    );
    let g = gm3(m);
    let inv = m.inverse().unwrap();
    let ginv = g.inverse();
    assert!(mat3_approx_eq(inv, wm3(ginv)));
}

#[test]
fn inverse_roundtrip() {
    let m = Mat3::<f32>::from_cols(
        2.0, 1.0, 0.0,
        0.0, 1.0, 1.0,
        1.0, 0.0, 1.0,
    );
    let inv = m.inverse().unwrap();
    let product = m * inv;
    let g = gm3(m);
    let gproduct = g * g.inverse();
    assert!(mat3_approx_eq(product, wm3(gproduct)));
}

#[test]
fn inverse_singular() {
    let m = Mat3::<f32>::from_cols(
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        1.0, 2.0, 3.0,
    );
    assert!(m.inverse().is_none());
}

// ---- From constructors ----

#[test]
fn from_translation() {
    let m = Mat3::<f32>::from_translation(Vec2::new(3.0, 4.0));
    let g = glam::Mat3::from_translation(glam::Vec2::new(3.0, 4.0));
    assert!(mat3_approx_eq(m, wm3(g)));
}

#[test]
fn from_translation_transform() {
    let m = Mat3::<f32>::from_translation(Vec2::new(3.0, 4.0));
    let g = glam::Mat3::from_translation(glam::Vec2::new(3.0, 4.0));
    let v = m * Vec3::new(1.0, 2.0, 1.0);
    let gv = g * glam::Vec3::new(1.0, 2.0, 1.0);
    assert!(vec3_approx_eq(v, wv3(gv)));
}

#[test]
fn from_angle() {
    let angle = std::f32::consts::FRAC_PI_2;
    let m = Mat3::<f32>::from_angle(Rad(angle));
    let g = glam::Mat3::from_angle(angle);
    assert!(mat3_approx_eq(m, wm3(g)));
}

#[test]
fn from_angle_transform() {
    let angle = std::f32::consts::FRAC_PI_2;
    let m = Mat3::<f32>::from_angle(Rad(angle));
    let g = glam::Mat3::from_angle(angle);
    let v = m * Vec3::new(1.0, 0.0, 0.0);
    let gv = g * glam::Vec3::new(1.0, 0.0, 0.0);
    assert!(vec3_approx_eq(v, wv3(gv)));
}

#[test]
fn from_scale() {
    let m = Mat3::<f32>::from_scale(Vec2::new(2.0, 3.0));
    let g = glam::Mat3::from_scale(glam::Vec2::new(2.0, 3.0));
    assert!(mat3_approx_eq(m, wm3(g)));
}

#[test]
fn from_mat2() {
    let m2 = Mat2::<f32>::from_cols(1.0, 2.0, 3.0, 4.0);
    let m3 = Mat3::from_mat2(m2);
    let gm2 = glam::Mat2::from_cols(glam::Vec2::new(1.0, 2.0), glam::Vec2::new(3.0, 4.0));
    let gm3 = glam::Mat3::from_mat2(gm2);
    assert!(mat3_approx_eq(m3, wm3(gm3)));
}

// ---- Multiply ----

#[test]
fn mul_identity() {
    let m = Mat3::<f32>::from_cols(
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0,
    );
    let i = Mat3::identity();
    let g = gm3(m);
    let gi = glam::Mat3::IDENTITY;
    assert!(mat3_approx_eq(m * i, wm3(g * gi)));
    assert!(mat3_approx_eq(i * m, wm3(gi * g)));
}

#[test]
fn mul_mat3() {
    let a = Mat3::<f32>::from_cols(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    let b = Mat3::<f32>::from_cols(9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0);
    let c = a * b;
    let ga = gm3(a);
    let gb = gm3(b);
    let gc = ga * gb;
    assert!(mat3_approx_eq(c, wm3(gc)));
}

#[test]
fn mul_vec3() {
    let m = Mat3::<f32>::from_cols(
        2.0, 1.0, 0.0,
        0.0, 1.0, 1.0,
        1.0, 0.0, 1.0,
    );
    let v = Vec3::new(1.0f32, 2.0, 3.0);
    let g = gm3(m);
    let gv = glam::Vec3::new(1.0, 2.0, 3.0);
    assert!(vec3_approx_eq(m * v, wv3(g * gv)));
}

// ---- Array conversions ----

#[test]
fn to_from_cols_array() {
    let m = Mat3::<f32>::from_cols(
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0,
    );
    let g = gm3(m);
    assert_eq!(m.to_cols_array(), g.to_cols_array());
    let m2 = Mat3::from_cols_array(m.to_cols_array());
    let g2 = glam::Mat3::from_cols_array(&g.to_cols_array());
    assert!(mat3_approx_eq(m2, wm3(g2)));
}

// ---- Col / Row ----

#[test]
fn col_row() {
    let m = Mat3::<f32>::from_cols(
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0,
    );
    let g = gm3(m);
    assert!(vec3_approx_eq(*m.col(0), wv3(g.col(0))));
    assert!(vec3_approx_eq(*m.col(1), wv3(g.col(1))));
    assert!(vec3_approx_eq(*m.col(2), wv3(g.col(2))));
    assert!(vec3_approx_eq(m.row(0), wv3(g.row(0))));
    assert!(vec3_approx_eq(m.row(1), wv3(g.row(1))));
    assert!(vec3_approx_eq(m.row(2), wv3(g.row(2))));
}

// ---- Index ----

#[test]
fn index() {
    let m = Mat3::<f32>::from_cols(
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0,
    );
    let g = gm3(m);
    assert!(vec3_approx_eq(m[0], wv3(g.col(0))));
    assert!(vec3_approx_eq(m[1], wv3(g.col(1))));
    assert!(vec3_approx_eq(m[2], wv3(g.col(2))));
}

#[test]
fn index_mut() {
    let mut m = Mat3::<f32>::identity();
    m[0] = Vec3::new(9.0, 8.0, 7.0);
    assert!(vec3_approx_eq(m.x, Vec3::new(9.0, 8.0, 7.0)));
}

// ---- Arithmetic Ops ----

#[test]
fn add_sub() {
    let a = Mat3::<f32>::from_cols(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    let b = Mat3::<f32>::from_cols(9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0);
    let ga = gm3(a);
    let gb = gm3(b);
    assert!(mat3_approx_eq(a + b, wm3(ga + gb)));
    assert!(mat3_approx_eq(b - a, wm3(gb - ga)));
}

#[test]
fn mul_div_scalar() {
    let m = Mat3::<f32>::from_cols(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    let g = gm3(m);
    assert!(mat3_approx_eq(m * 2.0, wm3(g * 2.0)));
    assert!(mat3_approx_eq(m / 2.0, wm3(g / 2.0)));
}

#[test]
fn neg() {
    let m = Mat3::<f32>::from_cols(1.0, -2.0, 3.0, -4.0, 5.0, -6.0, 7.0, -8.0, 9.0);
    let g = gm3(m);
    assert!(mat3_approx_eq(-m, wm3(-g)));
}

// ---- Assign Ops ----

#[test]
fn assign_ops() {
    let mut m = Mat3::<f32>::from_cols(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    let mut g = gm3(m);

    let ones = Mat3::<f32>::from_cols(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
    let gones = gm3(ones);

    m += ones;
    g += gones;
    assert!(mat3_approx_eq(m, wm3(g)));

    m *= 2.0f32;
    g *= 2.0;
    assert!(mat3_approx_eq(m, wm3(g)));
}

// ---- Display / Checks ----

#[test]
fn display() {
    let m = Mat3::<f32>::identity();
    let s = format!("{}", m);
    assert!(s.contains("Mat3"));
}

#[test]
fn is_nan() {
    let m = Mat3::<f32>::from_cols(1.0, f32::NAN, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    assert!(m.is_nan());
    assert!(!Mat3::<f32>::identity().is_nan());
}

#[test]
fn is_finite() {
    assert!(Mat3::<f32>::identity().is_finite());
    let m = Mat3::<f32>::from_cols(1.0, f32::INFINITY, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    assert!(!m.is_finite());
}

// ---- Conversions ----

#[test]
fn from_array_conversion() {
    let cols = [
        Vec3::new(1.0f32, 2.0, 3.0),
        Vec3::new(4.0, 5.0, 6.0),
        Vec3::new(7.0, 8.0, 9.0),
    ];
    let m: Mat3<f32> = Mat3::from(cols);
    let arr: [Vec3<f32>; 3] = m.into();
    assert_eq!(arr, cols);
}

#[test]
fn mat2_to_mat3() {
    let m2 = Mat2::<f32>::from_cols(1.0, 2.0, 3.0, 4.0);
    let m3: Mat3<f32> = m2.into();
    let gm2 = glam::Mat2::from_cols(glam::Vec2::new(1.0, 2.0), glam::Vec2::new(3.0, 4.0));
    let gm3 = glam::Mat3::from_mat2(gm2);
    assert!(mat3_approx_eq(m3, wm3(gm3)));
}

// ---- Comprehensive comparisons ----

#[test]
fn comprehensive_mul_vs_glam() {
    let mats = [
        [1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        [2.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0],
        [-1.0, 3.0, 2.0, 4.0, -2.0, 1.0, 0.0, 5.0, -3.0],
    ];
    for cols in &mats {
        for cols2 in &mats {
            let a = Mat3::from_cols_array(*cols);
            let b = Mat3::from_cols_array(*cols2);
            let c = a * b;
            let ga = gm3(a);
            let gb = gm3(b);
            let gc = ga * gb;
            assert!(mat3_approx_eq(c, wm3(gc)));
        }
    }
}

#[test]
fn comprehensive_inverse_vs_glam() {
    let mats = [
        [1.0f32, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 3.0],
        [2.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0],
        [3.0, 1.0, -2.0, 0.0, 4.0, 1.0, -1.0, 2.0, 5.0],
    ];
    for cols in mats {
        let m = Mat3::from_cols_array(cols);
        let g = gm3(m);
        let inv = m.inverse().unwrap();
        let ginv = g.inverse();
        assert!(mat3_approx_eq(inv, wm3(ginv)));
    }
}

#[test]
fn comprehensive_mul_vec_vs_glam() {
    let m = Mat3::<f32>::from_cols(
        2.0, 1.0, 0.0,
        0.0, 1.0, 1.0,
        1.0, 0.0, 1.0,
    );
    let g = gm3(m);
    let vecs = [
        Vec3::new(1.0f32, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(3.0, 4.0, 5.0),
        Vec3::new(-1.0, 2.0, -3.0),
    ];
    for v in vecs {
        assert!(vec3_approx_eq(m * v, wv3(g * gv3(v))));
    }
}

#[test]
fn look_to_rh() {
    let dir = Vec3::new(0.0f32, 0.0, -1.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let m = Mat3::<f32>::look_to_rh(dir, up);
    let g = glam::Mat3::look_to_rh(gv3(dir), gv3(up));
    assert!(mat3_approx_eq(m, wm3(g)));
}

#[test]
fn look_to_lh() {
    let dir = Vec3::new(0.0f32, 0.0, -1.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let m = Mat3::<f32>::look_to_lh(dir, up);
    let g = glam::Mat3::look_to_lh(gv3(dir), gv3(up));
    assert!(mat3_approx_eq(m, wm3(g)));
}

#[test]
fn look_at_rh() {
    let eye = Vec3::new(0.0f32, 5.0, 10.0);
    let center = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let m = Mat3::<f32>::look_at_rh(eye, center, up);
    let g = glam::Mat3::look_at_rh(gv3(eye), gv3(center), gv3(up));
    assert!(mat3_approx_eq(m, wm3(g)));
}

#[test]
fn look_at_lh() {
    let eye = Vec3::new(0.0f32, 5.0, 10.0);
    let center = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let m = Mat3::<f32>::look_at_lh(eye, center, up);
    let g = glam::Mat3::look_at_lh(gv3(eye), gv3(center), gv3(up));
    assert!(mat3_approx_eq(m, wm3(g)));
}

#[test]
fn comprehensive_look_to_vs_glam() {
    let dirs = [
        Vec3::new(0.0f32, 0.0, -1.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
    ];
    let up = Vec3::new(0.0f32, 1.0, 0.0);

    for dir in dirs {
        let mrh = Mat3::<f32>::look_to_rh(dir, up);
        let grh = glam::Mat3::look_to_rh(gv3(dir), gv3(up));
        assert!(mat3_approx_eq(mrh, wm3(grh)));

        let mlh = Mat3::<f32>::look_to_lh(dir, up);
        let glh = glam::Mat3::look_to_lh(gv3(dir), gv3(up));
        assert!(mat3_approx_eq(mlh, wm3(glh)));
    }
}

#[test]
fn comprehensive_look_at_vs_glam() {
    let eyes = [
        Vec3::new(0.0f32, 5.0, 10.0),
        Vec3::new(3.0, 3.0, 3.0),
        Vec3::new(-5.0, 0.0, 5.0),
    ];
    let center = Vec3::new(0.0f32, 0.0, 0.0);
    let up = Vec3::new(0.0f32, 1.0, 0.0);

    for eye in eyes {
        let mrh = Mat3::<f32>::look_at_rh(eye, center, up);
        let grh = glam::Mat3::look_at_rh(gv3(eye), gv3(center), gv3(up));
        assert!(mat3_approx_eq(mrh, wm3(grh)));

        let mlh = Mat3::<f32>::look_at_lh(eye, center, up);
        let glh = glam::Mat3::look_at_lh(gv3(eye), gv3(center), gv3(up));
        assert!(mat3_approx_eq(mlh, wm3(glh)));
    }
}
