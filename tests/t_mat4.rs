use wgm::{Vec3, Vec4, Mat3, Mat4};
use wgm::units::Rad;

const EPSILON: f32 = 1e-4;

fn approx_eq(a: f32, b: f32) -> bool {
    return (a - b).abs() < EPSILON;
}

fn mat4_approx_eq(a: Mat4<f32>, b: Mat4<f32>) -> bool {
    let aa = a.to_cols_array();
    let bb = b.to_cols_array();
    return aa.iter().zip(bb.iter()).all(|(x, y)| return approx_eq(*x, *y));
}

fn vec4_approx_eq(a: Vec4<f32>, b: Vec4<f32>) -> bool {
    return approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z) && approx_eq(a.w, b.w);
}

fn vec3_approx_eq(a: Vec3<f32>, b: Vec3<f32>) -> bool {
    return approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z);
}

fn gm4(m: Mat4<f32>) -> glam::Mat4 {
    return glam::Mat4::from_cols_array(&m.to_cols_array());
}

fn wm4(m: glam::Mat4) -> Mat4<f32> {
    return Mat4::from_cols_array(m.to_cols_array());
}

#[allow(dead_code)]
fn gv4(v: Vec4<f32>) -> glam::Vec4 {
    return glam::Vec4::new(v.x, v.y, v.z, v.w);
}

fn wv4(v: glam::Vec4) -> Vec4<f32> {
    return Vec4::new(v.x, v.y, v.z, v.w);
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
    let m = Mat4::new(
        Vec4::new(1.0f32, 2.0, 3.0, 4.0),
        Vec4::new(5.0, 6.0, 7.0, 8.0),
        Vec4::new(9.0, 10.0, 11.0, 12.0),
        Vec4::new(13.0, 14.0, 15.0, 16.0),
    );
    let g = glam::Mat4::from_cols(
        glam::Vec4::new(1.0, 2.0, 3.0, 4.0),
        glam::Vec4::new(5.0, 6.0, 7.0, 8.0),
        glam::Vec4::new(9.0, 10.0, 11.0, 12.0),
        glam::Vec4::new(13.0, 14.0, 15.0, 16.0),
    );
    assert!(mat4_approx_eq(m, wm4(g)));
}

#[test]
fn from_cols() {
    let m = Mat4::<f32>::from_cols(
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0,
    );
    let g = glam::Mat4::from_cols(
        glam::Vec4::new(1.0, 2.0, 3.0, 4.0),
        glam::Vec4::new(5.0, 6.0, 7.0, 8.0),
        glam::Vec4::new(9.0, 10.0, 11.0, 12.0),
        glam::Vec4::new(13.0, 14.0, 15.0, 16.0),
    );
    assert!(mat4_approx_eq(m, wm4(g)));
}

#[test]
fn identity() {
    let i = Mat4::<f32>::identity();
    let g = glam::Mat4::IDENTITY;
    assert!(mat4_approx_eq(i, wm4(g)));
}

#[test]
fn zero() {
    let z = Mat4::<f32>::zero();
    let g = glam::Mat4::ZERO;
    assert!(mat4_approx_eq(z, wm4(g)));
}

// ---- Determinant ----

#[test]
fn determinant_identity() {
    let i = Mat4::<f32>::identity();
    let g = glam::Mat4::IDENTITY;
    assert!(approx_eq(i.determinant(), g.determinant()));
}

#[test]
fn determinant_diagonal() {
    let m = Mat4::<f32>::from_cols(
        2.0, 0.0, 0.0, 0.0,
        0.0, 3.0, 0.0, 0.0,
        0.0, 0.0, 4.0, 0.0,
        0.0, 0.0, 0.0, 5.0,
    );
    let g = gm4(m);
    assert!(approx_eq(m.determinant(), g.determinant()));
}

#[test]
fn determinant_general() {
    let m = Mat4::<f32>::from_cols(
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0,
    );
    let g = gm4(m);
    assert!(approx_eq(m.determinant(), g.determinant()));
}

// ---- Transpose ----

#[test]
fn transpose() {
    let m = Mat4::<f32>::from_cols(
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0,
    );
    let g = gm4(m);
    assert!(mat4_approx_eq(m.transpose(), wm4(g.transpose())));
}

// ---- Inverse ----

#[test]
fn inverse_diagonal() {
    let m = Mat4::<f32>::from_cols(
        2.0, 0.0, 0.0, 0.0,
        0.0, 3.0, 0.0, 0.0,
        0.0, 0.0, 4.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );
    let g = gm4(m);
    let inv = m.inverse().unwrap();
    let ginv = g.inverse();
    assert!(mat4_approx_eq(inv, wm4(ginv)));
}

#[test]
fn inverse_general() {
    let m = Mat4::<f32>::from_cols(
        1.0, 0.0, 0.0, 0.0,
        0.0, 2.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        3.0, 4.0, 5.0, 1.0,
    );
    let g = gm4(m);
    let inv = m.inverse().unwrap();
    let ginv = g.inverse();
    assert!(mat4_approx_eq(inv, wm4(ginv)));
}

#[test]
fn inverse_roundtrip() {
    let m = Mat4::<f32>::from_cols(
        2.0, 0.0, 0.0, 0.0,
        0.0, 3.0, 0.0, 0.0,
        0.0, 0.0, 4.0, 0.0,
        1.0, 2.0, 3.0, 1.0,
    );
    let inv = m.inverse().unwrap();
    let product = m * inv;
    let g = gm4(m);
    let gproduct = g * g.inverse();
    assert!(mat4_approx_eq(product, wm4(gproduct)));
}

#[test]
fn inverse_singular() {
    let m = Mat4::<f32>::from_cols(
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0,
    );
    assert!(m.inverse().is_none());
}

// ---- From constructors ----

#[test]
fn from_translation() {
    let m = Mat4::<f32>::from_translation(Vec3::new(3.0, 4.0, 5.0));
    let g = glam::Mat4::from_translation(glam::Vec3::new(3.0, 4.0, 5.0));
    assert!(mat4_approx_eq(m, wm4(g)));
}

#[test]
fn from_scale() {
    let m = Mat4::<f32>::from_scale(Vec3::new(2.0, 3.0, 4.0));
    let g = glam::Mat4::from_scale(glam::Vec3::new(2.0, 3.0, 4.0));
    assert!(mat4_approx_eq(m, wm4(g)));
}

#[test]
fn from_rotation_x() {
    let angle = std::f32::consts::FRAC_PI_4;
    let m = Mat4::<f32>::from_rotation_x(Rad(angle));
    let g = glam::Mat4::from_rotation_x(angle);
    assert!(mat4_approx_eq(m, wm4(g)));
}

#[test]
fn from_rotation_y() {
    let angle = std::f32::consts::FRAC_PI_3;
    let m = Mat4::<f32>::from_rotation_y(Rad(angle));
    let g = glam::Mat4::from_rotation_y(angle);
    assert!(mat4_approx_eq(m, wm4(g)));
}

#[test]
fn from_rotation_z() {
    let angle = std::f32::consts::FRAC_PI_6;
    let m = Mat4::<f32>::from_rotation_z(Rad(angle));
    let g = glam::Mat4::from_rotation_z(angle);
    assert!(mat4_approx_eq(m, wm4(g)));
}

#[test]
fn from_axis_angle() {
    let axis = Vec3::new(1.0f32, 1.0, 1.0).normalize();
    let angle = 1.2f32;
    let m = Mat4::<f32>::from_axis_angle(axis, Rad(angle));
    let g = glam::Mat4::from_axis_angle(gv3(axis), angle);
    assert!(mat4_approx_eq(m, wm4(g)));
}

#[test]
fn from_axis_angle_x() {
    let axis = Vec3::new(1.0f32, 0.0, 0.0);
    let angle = std::f32::consts::FRAC_PI_2;
    let m = Mat4::<f32>::from_axis_angle(axis, Rad(angle));
    let g = glam::Mat4::from_axis_angle(gv3(axis), angle);
    assert!(mat4_approx_eq(m, wm4(g)));
}

#[test]
fn from_mat3() {
    let m3 = Mat3::<f32>::from_cols(
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0,
    );
    let m4 = Mat4::from_mat3(m3);
    let gm3 = glam::Mat3::from_cols(
        glam::Vec3::new(1.0, 2.0, 3.0),
        glam::Vec3::new(4.0, 5.0, 6.0),
        glam::Vec3::new(7.0, 8.0, 9.0),
    );
    let gm4 = glam::Mat4::from_mat3(gm3);
    assert!(mat4_approx_eq(m4, wm4(gm4)));
}

// ---- Projection ----

#[test]
fn perspective_rh() {
    let fov = std::f32::consts::FRAC_PI_2;
    let aspect = 16.0 / 9.0;
    let near = 0.1f32;
    let far = 100.0f32;
    let m = Mat4::<f32>::perspective_rh(Rad(fov), aspect, near, far);
    let g = glam::Mat4::perspective_rh(fov, aspect, near, far);
    assert!(mat4_approx_eq(m, wm4(g)));
}

#[test]
fn perspective_lh() {
    let fov = std::f32::consts::FRAC_PI_2;
    let aspect = 16.0 / 9.0;
    let near = 0.1f32;
    let far = 100.0f32;
    let m = Mat4::<f32>::perspective_lh(Rad(fov), aspect, near, far);
    let g = glam::Mat4::perspective_lh(fov, aspect, near, far);
    assert!(mat4_approx_eq(m, wm4(g)));
}

#[test]
fn perspective_infinite_reverse_rh() {
    let fov = std::f32::consts::FRAC_PI_2;
    let aspect = 16.0 / 9.0;
    let near = 0.1f32;
    let m = Mat4::<f32>::perspective_infinite_reverse_rh(Rad(fov), aspect, near);
    let g = glam::Mat4::perspective_infinite_reverse_rh(fov, aspect, near);
    assert!(mat4_approx_eq(m, wm4(g)));
}

#[test]
fn perspective_infinite_reverse_lh() {
    let fov = std::f32::consts::FRAC_PI_2;
    let aspect = 16.0 / 9.0;
    let near = 0.1f32;
    let m = Mat4::<f32>::perspective_infinite_reverse_lh(Rad(fov), aspect, near);
    let g = glam::Mat4::perspective_infinite_reverse_lh(fov, aspect, near);
    assert!(mat4_approx_eq(m, wm4(g)));
}

#[test]
fn orthographic_rh() {
    let m = Mat4::<f32>::orthographic_rh(-10.0, 10.0, -5.0, 5.0, 0.1, 100.0);
    let g = glam::Mat4::orthographic_rh(-10.0, 10.0, -5.0, 5.0, 0.1, 100.0);
    assert!(mat4_approx_eq(m, wm4(g)));
}

#[test]
fn orthographic_lh() {
    let m = Mat4::<f32>::orthographic_lh(-10.0, 10.0, -5.0, 5.0, 0.1, 100.0);
    let g = glam::Mat4::orthographic_lh(-10.0, 10.0, -5.0, 5.0, 0.1, 100.0);
    assert!(mat4_approx_eq(m, wm4(g)));
}

// ---- Look-at ----

#[test]
fn look_at_rh() {
    let eye = Vec3::new(0.0f32, 5.0, 10.0);
    let center = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let m = Mat4::<f32>::look_at_rh(eye, center, up);
    let g = glam::Mat4::look_at_rh(gv3(eye), gv3(center), gv3(up));
    assert!(mat4_approx_eq(m, wm4(g)));
}

#[test]
fn look_at_lh() {
    let eye = Vec3::new(0.0f32, 5.0, 10.0);
    let center = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let m = Mat4::<f32>::look_at_lh(eye, center, up);
    let g = glam::Mat4::look_at_lh(gv3(eye), gv3(center), gv3(up));
    assert!(mat4_approx_eq(m, wm4(g)));
}

#[test]
fn look_to_rh() {
    let eye = Vec3::new(0.0f32, 5.0, 10.0);
    let dir = Vec3::new(0.0, -0.4472, -0.8944);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let m = Mat4::<f32>::look_to_rh(eye, dir, up);
    let g = glam::Mat4::look_to_rh(gv3(eye), gv3(dir), gv3(up));
    assert!(mat4_approx_eq(m, wm4(g)));
}

#[test]
fn look_to_lh() {
    let eye = Vec3::new(0.0f32, 5.0, 10.0);
    let dir = Vec3::new(0.0, -0.4472, -0.8944);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let m = Mat4::<f32>::look_to_lh(eye, dir, up);
    let g = glam::Mat4::look_to_lh(gv3(eye), gv3(dir), gv3(up));
    assert!(mat4_approx_eq(m, wm4(g)));
}

// ---- Multiply ----

#[test]
fn mul_identity() {
    let m = Mat4::<f32>::from_cols(
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0,
    );
    let i = Mat4::identity();
    let g = gm4(m);
    let gi = glam::Mat4::IDENTITY;
    assert!(mat4_approx_eq(m * i, wm4(g * gi)));
    assert!(mat4_approx_eq(i * m, wm4(gi * g)));
}

#[test]
fn mul_mat4() {
    let a = Mat4::<f32>::from_cols(
        1.0, 0.0, 0.0, 0.0,
        0.0, 2.0, 0.0, 0.0,
        0.0, 0.0, 3.0, 0.0,
        4.0, 5.0, 6.0, 1.0,
    );
    let b = Mat4::<f32>::from_cols(
        2.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 2.0, 0.0,
        1.0, 2.0, 3.0, 1.0,
    );
    let ga = gm4(a);
    let gb = gm4(b);
    assert!(mat4_approx_eq(a * b, wm4(ga * gb)));
}

#[test]
fn mul_vec4() {
    let m = Mat4::<f32>::from_cols(
        2.0, 0.0, 0.0, 0.0,
        0.0, 3.0, 0.0, 0.0,
        0.0, 0.0, 4.0, 0.0,
        1.0, 2.0, 3.0, 1.0,
    );
    let v = Vec4::new(1.0f32, 2.0, 3.0, 1.0);
    let g = gm4(m);
    let gv = glam::Vec4::new(1.0, 2.0, 3.0, 1.0);
    assert!(vec4_approx_eq(m * v, wv4(g * gv)));
}

// ---- Transform ----

#[test]
fn transform_point3() {
    let m = Mat4::<f32>::from_translation(Vec3::new(10.0, 20.0, 30.0));
    let p = Vec3::new(1.0f32, 2.0, 3.0);
    let g = glam::Mat4::from_translation(glam::Vec3::new(10.0, 20.0, 30.0));
    let gp = glam::Vec3::new(1.0, 2.0, 3.0);
    assert!(vec3_approx_eq(m.transform_point3(p), wv3(g.transform_point3(gp))));
}

#[test]
fn transform_vector3() {
    let m = Mat4::<f32>::from_translation(Vec3::new(10.0, 20.0, 30.0));
    let v = Vec3::new(1.0f32, 2.0, 3.0);
    let g = glam::Mat4::from_translation(glam::Vec3::new(10.0, 20.0, 30.0));
    let gv = glam::Vec3::new(1.0, 2.0, 3.0);
    assert!(vec3_approx_eq(m.transform_vector3(v), wv3(g.transform_vector3(gv))));
}

#[test]
fn transform_point3_rotation() {
    let angle = std::f32::consts::FRAC_PI_2;
    let m = Mat4::<f32>::from_rotation_z(Rad(angle));
    let p = Vec3::new(1.0f32, 0.0, 0.0);
    let g = glam::Mat4::from_rotation_z(angle);
    let gp = glam::Vec3::new(1.0, 0.0, 0.0);
    assert!(vec3_approx_eq(m.transform_point3(p), wv3(g.transform_point3(gp))));
}

// ---- Array conversions ----

#[test]
fn to_from_cols_array() {
    let m = Mat4::<f32>::from_cols(
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0,
    );
    let g = gm4(m);
    assert_eq!(m.to_cols_array(), g.to_cols_array());
    let m2 = Mat4::from_cols_array(m.to_cols_array());
    let g2 = glam::Mat4::from_cols_array(&g.to_cols_array());
    assert!(mat4_approx_eq(m2, wm4(g2)));
}

// ---- Col / Row ----

#[test]
fn col_row() {
    let m = Mat4::<f32>::from_cols(
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0,
    );
    let g = gm4(m);
    assert!(vec4_approx_eq(*m.col(0), wv4(g.col(0))));
    assert!(vec4_approx_eq(*m.col(1), wv4(g.col(1))));
    assert!(vec4_approx_eq(*m.col(2), wv4(g.col(2))));
    assert!(vec4_approx_eq(*m.col(3), wv4(g.col(3))));
    assert!(vec4_approx_eq(m.row(0), wv4(g.row(0))));
    assert!(vec4_approx_eq(m.row(1), wv4(g.row(1))));
    assert!(vec4_approx_eq(m.row(2), wv4(g.row(2))));
    assert!(vec4_approx_eq(m.row(3), wv4(g.row(3))));
}

// ---- Index ----

#[test]
fn index() {
    let m = Mat4::<f32>::from_cols(
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0,
    );
    let g = gm4(m);
    assert!(vec4_approx_eq(m[0], wv4(g.col(0))));
    assert!(vec4_approx_eq(m[1], wv4(g.col(1))));
    assert!(vec4_approx_eq(m[2], wv4(g.col(2))));
    assert!(vec4_approx_eq(m[3], wv4(g.col(3))));
}

#[test]
fn index_mut() {
    let mut m = Mat4::<f32>::identity();
    m[0] = Vec4::new(9.0, 8.0, 7.0, 6.0);
    assert!(vec4_approx_eq(m.x, Vec4::new(9.0, 8.0, 7.0, 6.0)));
}

// ---- Arithmetic Ops ----

#[test]
fn add_sub() {
    let a = Mat4::<f32>::from_cols(
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    );
    let b = Mat4::<f32>::from_cols(
        16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0,
        8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
    );
    let ga = gm4(a);
    let gb = gm4(b);
    assert!(mat4_approx_eq(a + b, wm4(ga + gb)));
    assert!(mat4_approx_eq(a - b, wm4(ga - gb)));
}

#[test]
fn mul_div_scalar() {
    let m = Mat4::<f32>::from_cols(
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    );
    let g = gm4(m);
    assert!(mat4_approx_eq(m * 2.0, wm4(g * 2.0)));
    assert!(mat4_approx_eq(m / 2.0, wm4(g / 2.0)));
}

#[test]
fn neg() {
    let m = Mat4::<f32>::from_cols(
        1.0, -2.0, 3.0, -4.0, 5.0, -6.0, 7.0, -8.0,
        9.0, -10.0, 11.0, -12.0, 13.0, -14.0, 15.0, -16.0,
    );
    let g = gm4(m);
    assert!(mat4_approx_eq(-m, wm4(-g)));
}

// ---- Assign Ops ----

#[test]
fn assign_ops() {
    let ones_arr = [1.0f32; 16];
    let ones = Mat4::from_cols_array(ones_arr);
    let gones = glam::Mat4::from_cols_array(&ones_arr);

    let mut m = Mat4::<f32>::from_cols(
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    );
    let mut g = gm4(m);

    m += ones;
    g += gones;
    assert!(mat4_approx_eq(m, wm4(g)));

    m *= 2.0f32;
    g *= 2.0;
    assert!(mat4_approx_eq(m, wm4(g)));
}

// ---- Display / Checks ----

#[test]
fn display() {
    let m = Mat4::<f32>::identity();
    let s = format!("{}", m);
    assert!(s.contains("Mat4"));
}

#[test]
fn is_nan() {
    let mut m = Mat4::<f32>::identity();
    assert!(!m.is_nan());
    m.x.x = f32::NAN;
    assert!(m.is_nan());
}

#[test]
fn is_finite() {
    assert!(Mat4::<f32>::identity().is_finite());
    let mut m = Mat4::<f32>::identity();
    m.x.x = f32::INFINITY;
    assert!(!m.is_finite());
}

// ---- Conversions ----

#[test]
fn from_array_conversion() {
    let cols = [
        Vec4::new(1.0f32, 2.0, 3.0, 4.0),
        Vec4::new(5.0, 6.0, 7.0, 8.0),
        Vec4::new(9.0, 10.0, 11.0, 12.0),
        Vec4::new(13.0, 14.0, 15.0, 16.0),
    ];
    let m: Mat4<f32> = Mat4::from(cols);
    let arr: [Vec4<f32>; 4] = m.into();
    assert_eq!(arr, cols);
}

// ---- Comprehensive comparisons ----

#[test]
fn comprehensive_rotation_vs_glam() {
    let angles = [0.0f32, 0.5, 1.0, 1.5, 2.0, std::f32::consts::PI, std::f32::consts::TAU];
    for a in angles {
        let mx = Mat4::<f32>::from_rotation_x(Rad(a));
        let gx = glam::Mat4::from_rotation_x(a);
        assert!(mat4_approx_eq(mx, wm4(gx)), "rotation_x failed for angle {}", a);

        let my = Mat4::<f32>::from_rotation_y(Rad(a));
        let gy = glam::Mat4::from_rotation_y(a);
        assert!(mat4_approx_eq(my, wm4(gy)), "rotation_y failed for angle {}", a);

        let mz = Mat4::<f32>::from_rotation_z(Rad(a));
        let gz = glam::Mat4::from_rotation_z(a);
        assert!(mat4_approx_eq(mz, wm4(gz)), "rotation_z failed for angle {}", a);
    }
}

#[test]
fn comprehensive_mul_vs_glam() {
    let i_arr = [1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
    let t_arr = [1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 3.0, 4.0, 5.0, 1.0];
    let s_arr = [2.0f32, 0.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 0.0, 1.0];

    let mats_arr = [i_arr, t_arr, s_arr];
    for a in &mats_arr {
        for b in &mats_arr {
            let ma = Mat4::from_cols_array(*a);
            let mb = Mat4::from_cols_array(*b);
            let ga = gm4(ma);
            let gb = gm4(mb);
            assert!(mat4_approx_eq(ma * mb, wm4(ga * gb)));
        }
    }
}

#[test]
fn comprehensive_inverse_vs_glam() {
    let mats = [
        [2.0f32, 0.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 0.0, 1.0],
        [1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 3.0, 4.0, 5.0, 1.0],
    ];
    for cols in mats {
        let m = Mat4::from_cols_array(cols);
        let g = gm4(m);
        let inv = m.inverse().unwrap();
        let ginv = g.inverse();
        assert!(mat4_approx_eq(inv, wm4(ginv)));
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
        let mrh = Mat4::<f32>::look_at_rh(eye, center, up);
        let grh = glam::Mat4::look_at_rh(gv3(eye), gv3(center), gv3(up));
        assert!(mat4_approx_eq(mrh, wm4(grh)));

        let mlh = Mat4::<f32>::look_at_lh(eye, center, up);
        let glh = glam::Mat4::look_at_lh(gv3(eye), gv3(center), gv3(up));
        assert!(mat4_approx_eq(mlh, wm4(glh)));
    }
}

#[test]
fn comprehensive_perspective_vs_glam() {
    let fovs = [std::f32::consts::FRAC_PI_4, std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_3];
    let aspects = [1.0f32, 16.0 / 9.0, 4.0 / 3.0];
    for fov in fovs {
        for aspect in aspects {
            let m = Mat4::<f32>::perspective_rh(Rad(fov), aspect, 0.1, 100.0);
            let g = glam::Mat4::perspective_rh(fov, aspect, 0.1, 100.0);
            assert!(mat4_approx_eq(m, wm4(g)));
        }
    }
}

#[test]
fn comprehensive_look_to_vs_glam() {
    let eyes = [
        Vec3::new(0.0f32, 5.0, 10.0),
        Vec3::new(3.0, 3.0, 3.0),
        Vec3::new(-5.0, 0.0, 5.0),
    ];
    let dirs = [
        Vec3::new(0.0f32, 0.0, -1.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    ];
    let up = Vec3::new(0.0f32, 1.0, 0.0);

    for eye in eyes {
        for dir in dirs {
            let mrh = Mat4::<f32>::look_to_rh(eye, dir, up);
            let grh = glam::Mat4::look_to_rh(gv3(eye), gv3(dir), gv3(up));
            assert!(mat4_approx_eq(mrh, wm4(grh)));

            let mlh = Mat4::<f32>::look_to_lh(eye, dir, up);
            let glh = glam::Mat4::look_to_lh(gv3(eye), gv3(dir), gv3(up));
            assert!(mat4_approx_eq(mlh, wm4(glh)));
        }
    }
}
