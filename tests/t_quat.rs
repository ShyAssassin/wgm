use wgm::{Vec3, Vec4, Mat3, Mat4, Quat};
use wgm::units::Rad;

const EPSILON: f32 = 1e-4;

fn approx_eq(a: f32, b: f32) -> bool {
    return (a - b).abs() < EPSILON;
}

fn quat_approx_eq(a: Quat<f32>, b: Quat<f32>) -> bool {
    // Quaternions q and -q represent the same rotation, so compare both signs
    let same = approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z) && approx_eq(a.w, b.w);
    let neg = approx_eq(a.x, -b.x) && approx_eq(a.y, -b.y) && approx_eq(a.z, -b.z) && approx_eq(a.w, -b.w);
    return same || neg;
}

fn vec3_approx_eq(a: Vec3<f32>, b: Vec3<f32>) -> bool {
    return approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z);
}

fn vec4_approx_eq(a: Vec4<f32>, b: Vec4<f32>) -> bool {
    return approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z) && approx_eq(a.w, b.w);
}

fn mat3_approx_eq(a: Mat3<f32>, b: Mat3<f32>) -> bool {
    let aa = a.to_cols_array();
    let bb = b.to_cols_array();
    return aa.iter().zip(bb.iter()).all(|(x, y)| return approx_eq(*x, *y));
}

fn mat4_approx_eq(a: Mat4<f32>, b: Mat4<f32>) -> bool {
    let aa = a.to_cols_array();
    let bb = b.to_cols_array();
    return aa.iter().zip(bb.iter()).all(|(x, y)| return approx_eq(*x, *y));
}

fn gq(q: Quat<f32>) -> glam::Quat {
    return glam::Quat::from_xyzw(q.x, q.y, q.z, q.w);
}

fn wq(q: glam::Quat) -> Quat<f32> {
    return Quat::new(q.x, q.y, q.z, q.w);
}

fn gv3(v: Vec3<f32>) -> glam::Vec3 {
    return glam::Vec3::new(v.x, v.y, v.z);
}

fn wv3(v: glam::Vec3) -> Vec3<f32> {
    return Vec3::new(v.x, v.y, v.z);
}

fn wm3(m: glam::Mat3) -> Mat3<f32> {
    return Mat3::from_cols_array(m.to_cols_array());
}

fn wm4(m: glam::Mat4) -> Mat4<f32> {
    return Mat4::from_cols_array(m.to_cols_array());
}

// ---- Construction ----

#[test]
fn new() {
    let q = Quat::new(1.0f32, 2.0, 3.0, 4.0);
    assert!(approx_eq(q.x, 1.0));
    assert!(approx_eq(q.y, 2.0));
    assert!(approx_eq(q.z, 3.0));
    assert!(approx_eq(q.w, 4.0));
}

#[test]
fn identity() {
    let q = Quat::<f32>::identity();
    let g = glam::Quat::IDENTITY;
    assert!(quat_approx_eq(q, wq(g)));
}

#[test]
fn from_axis_angle() {
    let axis = Vec3::new(0.0f32, 1.0, 0.0);
    let angle = std::f32::consts::FRAC_PI_2;
    let q = Quat::from_axis_angle(axis, Rad(angle));
    let g = glam::Quat::from_axis_angle(gv3(axis), angle);
    assert!(quat_approx_eq(q, wq(g)));
}

#[test]
fn from_axis_angle_arbitrary() {
    let axis = Vec3::new(1.0f32, 1.0, 1.0).normalize();
    let angle = 1.2f32;
    let q = Quat::from_axis_angle(axis, Rad(angle));
    let g = glam::Quat::from_axis_angle(gv3(axis), angle);
    assert!(quat_approx_eq(q, wq(g)));
}

#[test]
fn from_axis_angle_x() {
    let axis = Vec3::new(1.0f32, 0.0, 0.0);
    let angle = std::f32::consts::FRAC_PI_4;
    let q = Quat::from_axis_angle(axis, Rad(angle));
    let g = glam::Quat::from_axis_angle(gv3(axis), angle);
    assert!(quat_approx_eq(q, wq(g)));
}

#[test]
fn from_axis_angle_z() {
    let axis = Vec3::new(0.0f32, 0.0, 1.0);
    let angle = std::f32::consts::PI;
    let q = Quat::from_axis_angle(axis, Rad(angle));
    let g = glam::Quat::from_axis_angle(gv3(axis), angle);
    assert!(quat_approx_eq(q, wq(g)));
}

// ---- Properties ----

#[test]
fn length() {
    let q = Quat::new(1.0f32, 2.0, 3.0, 4.0);
    let g = gq(q);
    assert!(approx_eq(q.length(), g.length()));
}

#[test]
fn length_squared() {
    let q = Quat::new(1.0f32, 2.0, 3.0, 4.0);
    let g = gq(q);
    assert!(approx_eq(q.length_squared(), g.length_squared()));
}

#[test]
fn normalize() {
    let q = Quat::new(1.0f32, 2.0, 3.0, 4.0);
    let g = gq(q);
    let n = q.normalize();
    let gn = g.normalize();
    assert!(quat_approx_eq(n, wq(gn)));
}

#[test]
fn is_normalized() {
    let q = Quat::from_axis_angle(Vec3::new(0.0f32, 1.0, 0.0), Rad(1.0));
    assert!(q.is_normalized());
    let r = Quat::new(1.0f32, 2.0, 3.0, 4.0);
    assert!(!r.is_normalized());
}

// ---- Conjugate / Inverse ----

#[test]
fn conjugate() {
    let q = Quat::new(1.0f32, 2.0, 3.0, 4.0).normalize();
    let g = gq(q);
    assert!(quat_approx_eq(q.conjugate(), wq(g.conjugate())));
}

#[test]
fn inverse() {
    let q = Quat::from_axis_angle(Vec3::new(0.0f32, 1.0, 0.0), Rad(1.0));
    let g = gq(q);
    assert!(quat_approx_eq(q.inverse(), wq(g.inverse())));
}

#[test]
fn inverse_roundtrip() {
    let q = Quat::from_axis_angle(Vec3::new(1.0f32, 0.0, 0.0), Rad(0.5));
    let qi = q.inverse();
    let product = q * qi;
    let g = gq(q);
    let gi = g.inverse();
    let gproduct = g * gi;
    assert!(quat_approx_eq(product, wq(gproduct)));
}

// ---- Dot ----

#[test]
fn dot() {
    let a = Quat::new(1.0f32, 2.0, 3.0, 4.0).normalize();
    let b = Quat::new(4.0f32, 3.0, 2.0, 1.0).normalize();
    let ga = gq(a);
    let gb = gq(b);
    assert!(approx_eq(a.dot(b), ga.dot(gb)));
}

// ---- Lerp / Slerp ----

#[test]
fn lerp() {
    let a = Quat::from_axis_angle(Vec3::new(0.0f32, 1.0, 0.0), Rad(0.0));
    let b = Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), Rad(1.0));
    let ga = gq(a);
    let gb = gq(b);
    for t in [0.0f32, 0.25, 0.5, 0.75, 1.0] {
        let l = a.lerp(b, t);
        let gl = ga.lerp(gb, t);
        assert!(quat_approx_eq(l, wq(gl)), "lerp failed at t={}", t);
    }
}

#[test]
fn slerp() {
    let a = Quat::from_axis_angle(Vec3::new(0.0f32, 1.0, 0.0), Rad(0.0));
    let b = Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), Rad(std::f32::consts::FRAC_PI_2));
    let ga = gq(a);
    let gb = gq(b);
    for t in [0.0f32, 0.25, 0.5, 0.75, 1.0] {
        let s = a.slerp(b, t);
        let gs = ga.slerp(gb, t);
        assert!(quat_approx_eq(s, wq(gs)), "slerp failed at t={}", t);
    }
}

#[test]
fn slerp_endpoints() {
    let a = Quat::from_axis_angle(Vec3::new(1.0f32, 0.0, 0.0), Rad(0.5));
    let b = Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), Rad(1.5));
    let ga = gq(a);
    let gb = gq(b);
    assert!(quat_approx_eq(a.slerp(b, 0.0), wq(ga.slerp(gb, 0.0))));
    assert!(quat_approx_eq(a.slerp(b, 1.0), wq(ga.slerp(gb, 1.0))));
}

// ---- Mul ----

#[test]
fn mul_quat() {
    let a = Quat::from_axis_angle(Vec3::new(1.0f32, 0.0, 0.0), Rad(std::f32::consts::FRAC_PI_4));
    let b = Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), Rad(std::f32::consts::FRAC_PI_3));
    let ga = gq(a);
    let gb = gq(b);
    assert!(quat_approx_eq(a * b, wq(ga * gb)));
}

#[test]
fn mul_quat_identity() {
    let q = Quat::from_axis_angle(Vec3::new(1.0f32, 0.0, 0.0), Rad(1.0));
    let i = Quat::<f32>::identity();
    let g = gq(q);
    let gi = glam::Quat::IDENTITY;
    assert!(quat_approx_eq(q * i, wq(g * gi)));
    assert!(quat_approx_eq(i * q, wq(gi * g)));
}

#[test]
fn mul_vec3() {
    let q = Quat::from_axis_angle(Vec3::new(0.0f32, 1.0, 0.0), Rad(std::f32::consts::FRAC_PI_2));
    let v = Vec3::new(1.0f32, 0.0, 0.0);
    let g = gq(q);
    let gv = glam::Vec3::new(1.0, 0.0, 0.0);
    assert!(vec3_approx_eq(q.mul_vec3(v), wv3(g.mul_vec3(gv))));
}

#[test]
fn mul_vec3_identity() {
    let q = Quat::<f32>::identity();
    let v = Vec3::new(1.0f32, 2.0, 3.0);
    let g = glam::Quat::IDENTITY;
    let gv = glam::Vec3::new(1.0, 2.0, 3.0);
    assert!(vec3_approx_eq(q.mul_vec3(v), wv3(g.mul_vec3(gv))));
}

#[test]
fn mul_vec3_rotation() {
    let q = Quat::from_axis_angle(Vec3::new(0.0f32, 0.0, 1.0), Rad(std::f32::consts::FRAC_PI_2));
    let v = Vec3::new(1.0f32, 0.0, 0.0);
    let g = gq(q);
    let gv = glam::Vec3::new(1.0, 0.0, 0.0);
    assert!(vec3_approx_eq(q.mul_vec3(v), wv3(g.mul_vec3(gv))));
}

// ---- To/From matrices ----

#[test]
fn to_mat3() {
    let q = Quat::from_axis_angle(Vec3::new(0.0f32, 1.0, 0.0), Rad(std::f32::consts::FRAC_PI_4));
    let m = q.to_mat3();
    let g = gq(q);
    let gm = glam::Mat3::from_quat(g);
    assert!(mat3_approx_eq(m, wm3(gm)));
}

#[test]
fn to_mat4() {
    let q = Quat::from_axis_angle(Vec3::new(0.0f32, 1.0, 0.0), Rad(std::f32::consts::FRAC_PI_4));
    let m = q.to_mat4();
    let g = gq(q);
    let gm = glam::Mat4::from_quat(g);
    assert!(mat4_approx_eq(m, wm4(gm)));
}

#[test]
fn from_rotation_mat3() {
    let q_orig = Quat::from_axis_angle(Vec3::new(1.0f32, 0.0, 0.0), Rad(1.0));
    let m3 = q_orig.to_mat3();
    let q_back = Quat::from_rotation_mat3(m3);
    let g_orig = gq(q_orig);
    let gm3 = glam::Mat3::from_quat(g_orig);
    let g_back = glam::Quat::from_mat3(&gm3);
    assert!(quat_approx_eq(q_back, wq(g_back)));
}

#[test]
fn mat3_roundtrip() {
    let q = Quat::from_axis_angle(Vec3::new(0.0f32, 1.0, 0.0), Rad(0.8));
    let m = q.to_mat3();
    let q2 = Quat::from_rotation_mat3(m);
    let g = gq(q);
    let gm = glam::Mat3::from_quat(g);
    let g2 = glam::Quat::from_mat3(&gm);
    assert!(quat_approx_eq(q2, wq(g2)));
}

// ---- angle_between / to_axis_angle ----

#[test]
fn angle_between() {
    let a = Quat::from_axis_angle(Vec3::new(0.0f32, 1.0, 0.0), Rad(0.0));
    let b = Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), Rad(1.0));
    let ga = gq(a);
    let gb = gq(b);
    assert!(approx_eq(a.angle_between(b).inner(), ga.angle_between(gb)));
}

#[test]
fn angle_between_same() {
    let q = Quat::from_axis_angle(Vec3::new(1.0f32, 0.0, 0.0), Rad(0.5));
    let g = gq(q);
    assert!(approx_eq(q.angle_between(q).inner(), g.angle_between(g)));
}

#[test]
fn to_axis_angle() {
    let axis_in = Vec3::new(0.0f32, 1.0, 0.0);
    let angle_in = 1.5f32;
    let q = Quat::from_axis_angle(axis_in, Rad(angle_in));
    let (axis_out, angle_out) = q.to_axis_angle();
    let g = gq(q);
    let (gaxis, gangle) = g.to_axis_angle();
    assert!(approx_eq(angle_out.inner(), gangle));
    assert!(vec3_approx_eq(axis_out, wv3(gaxis)));
}

#[test]
fn to_axis_angle_x() {
    let axis_in = Vec3::new(1.0f32, 0.0, 0.0);
    let angle_in = 0.8f32;
    let q = Quat::from_axis_angle(axis_in, Rad(angle_in));
    let (axis_out, angle_out) = q.to_axis_angle();
    let g = gq(q);
    let (gaxis, gangle) = g.to_axis_angle();
    assert!(approx_eq(angle_out.inner(), gangle));
    assert!(vec3_approx_eq(axis_out, wv3(gaxis)));
}

// ---- Euler angles ----

#[test]
fn from_euler_angles_vs_mat3() {
    // Verify euler angles produce correct rotation by converting to mat3
    // and comparing against composed axis rotations in ZYX order: Rz(yaw) * Ry(pitch) * Rx(roll)
    let roll = 0.3f32;
    let pitch = 0.5;
    let yaw = 0.7;
    let q = Quat::from_euler_angles(Rad(roll), Rad(pitch), Rad(yaw));

    let gx = glam::Quat::from_rotation_x(roll);
    let gy = glam::Quat::from_rotation_y(pitch);
    let gz = glam::Quat::from_rotation_z(yaw);
    let g_combined = gz * gy * gx;
    assert!(quat_approx_eq(q, wq(g_combined)));
}

#[test]
fn from_euler_angles_zero() {
    let q = Quat::from_euler_angles(Rad(0.0f32), Rad(0.0), Rad(0.0));
    let g = glam::Quat::IDENTITY;
    assert!(quat_approx_eq(q, wq(g)));
}

// ---- Vector / Scalar ----

#[test]
fn vector_scalar() {
    let q = Quat::new(1.0f32, 2.0, 3.0, 4.0);
    assert!(vec3_approx_eq(q.vector(), Vec3::new(1.0, 2.0, 3.0)));
    assert!(approx_eq(q.scalar(), 4.0));
}

// ---- to_vec4 / from_vec4 ----

#[test]
fn to_from_vec4() {
    let q = Quat::new(1.0f32, 2.0, 3.0, 4.0);
    let v = q.to_vec4();
    assert!(vec4_approx_eq(v, Vec4::new(1.0, 2.0, 3.0, 4.0)));
    let q2 = Quat::from_vec4(v);
    assert!(approx_eq(q.x, q2.x));
    assert!(approx_eq(q.y, q2.y));
    assert!(approx_eq(q.z, q2.z));
    assert!(approx_eq(q.w, q2.w));
}

// ---- Checks ----

#[test]
fn is_nan() {
    let q = Quat::new(f32::NAN, 0.0, 0.0, 1.0);
    assert!(q.is_nan());
    assert!(!Quat::<f32>::identity().is_nan());
}

#[test]
fn is_finite() {
    assert!(Quat::<f32>::identity().is_finite());
    let q = Quat::new(f32::INFINITY, 0.0, 0.0, 1.0);
    assert!(!q.is_finite());
}

// ---- Operators ----

#[test]
fn add() {
    let a = Quat::new(1.0f32, 2.0, 3.0, 4.0);
    let b = Quat::new(5.0f32, 6.0, 7.0, 8.0);
    let ga = gq(a);
    let gb = gq(b);
    let c = a + b;
    let gc = glam::Quat::from_xyzw(ga.x + gb.x, ga.y + gb.y, ga.z + gb.z, ga.w + gb.w);
    assert!(quat_approx_eq(c, wq(gc)));
}

#[test]
fn sub() {
    let a = Quat::new(5.0f32, 6.0, 7.0, 8.0);
    let b = Quat::new(1.0f32, 2.0, 3.0, 4.0);
    let c = a - b;
    assert!(approx_eq(c.x, 4.0));
    assert!(approx_eq(c.y, 4.0));
    assert!(approx_eq(c.z, 4.0));
    assert!(approx_eq(c.w, 4.0));
}

#[test]
fn neg() {
    let q = Quat::new(1.0f32, -2.0, 3.0, -4.0);
    let n = -q;
    assert!(approx_eq(n.x, -1.0));
    assert!(approx_eq(n.y, 2.0));
    assert!(approx_eq(n.z, -3.0));
    assert!(approx_eq(n.w, 4.0));
}

#[test]
fn mul_scalar() {
    let q = Quat::new(1.0f32, 2.0, 3.0, 4.0);
    let s = q * 2.0;
    assert!(approx_eq(s.x, 2.0));
    assert!(approx_eq(s.y, 4.0));
    assert!(approx_eq(s.z, 6.0));
    assert!(approx_eq(s.w, 8.0));
}

// ---- Assign Ops ----

#[test]
fn add_assign() {
    let mut a = Quat::new(1.0f32, 2.0, 3.0, 4.0);
    let b = Quat::new(5.0f32, 6.0, 7.0, 8.0);
    a += b;
    assert!(approx_eq(a.x, 6.0));
    assert!(approx_eq(a.y, 8.0));
    assert!(approx_eq(a.z, 10.0));
    assert!(approx_eq(a.w, 12.0));
}

#[test]
fn sub_assign() {
    let mut a = Quat::new(5.0f32, 6.0, 7.0, 8.0);
    let b = Quat::new(1.0f32, 2.0, 3.0, 4.0);
    a -= b;
    assert!(approx_eq(a.x, 4.0));
    assert!(approx_eq(a.y, 4.0));
    assert!(approx_eq(a.z, 4.0));
    assert!(approx_eq(a.w, 4.0));
}

#[test]
fn mul_assign_scalar() {
    let mut q = Quat::new(1.0f32, 2.0, 3.0, 4.0);
    q *= 3.0f32;
    assert!(approx_eq(q.x, 3.0));
    assert!(approx_eq(q.y, 6.0));
    assert!(approx_eq(q.z, 9.0));
    assert!(approx_eq(q.w, 12.0));
}

// ---- Display ----

#[test]
fn display() {
    let q = Quat::<f32>::identity();
    let s = format!("{}", q);
    assert!(s.contains("Quat"));
}

// ---- Comprehensive comparisons ----

#[test]
fn comprehensive_axis_angle_vs_glam() {
    let axes = [
        Vec3::new(1.0f32, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 1.0, 0.0).normalize(),
        Vec3::new(1.0, 1.0, 1.0).normalize(),
    ];
    let angles = [0.0f32, 0.5, 1.0, 1.5, std::f32::consts::PI, std::f32::consts::FRAC_PI_2];
    for axis in &axes {
        for angle in &angles {
            let q = Quat::from_axis_angle(*axis, Rad(*angle));
            let g = glam::Quat::from_axis_angle(gv3(*axis), *angle);
            assert!(quat_approx_eq(q, wq(g)), "failed for axis={:?} angle={}", axis, angle);
        }
    }
}

#[test]
fn comprehensive_mul_quat_vs_glam() {
    let quats = [
        Quat::from_axis_angle(Vec3::new(1.0f32, 0.0, 0.0), Rad(0.5)),
        Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), Rad(1.0)),
        Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), Rad(1.5)),
        Quat::from_axis_angle(Vec3::new(1.0, 1.0, 1.0).normalize(), Rad(0.8)),
    ];
    for a in &quats {
        for b in &quats {
            let c = *a * *b;
            let ga = gq(*a);
            let gb = gq(*b);
            let gc = ga * gb;
            assert!(quat_approx_eq(c, wq(gc)));
        }
    }
}

#[test]
fn comprehensive_mul_vec3_vs_glam() {
    let q = Quat::from_axis_angle(Vec3::new(1.0f32, 1.0, 1.0).normalize(), Rad(1.2));
    let g = gq(q);
    let vecs = [
        Vec3::new(1.0f32, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(3.0, 4.0, 5.0),
        Vec3::new(-1.0, 2.0, -3.0),
    ];
    for v in vecs {
        assert!(vec3_approx_eq(q.mul_vec3(v), wv3(g.mul_vec3(gv3(v)))));
    }
}

#[test]
fn comprehensive_to_mat3_vs_glam() {
    let quats = [
        Quat::from_axis_angle(Vec3::new(1.0f32, 0.0, 0.0), Rad(0.5)),
        Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), Rad(1.0)),
        Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), Rad(1.5)),
        Quat::from_axis_angle(Vec3::new(1.0, 1.0, 0.0).normalize(), Rad(2.0)),
    ];
    for q in quats {
        let m = q.to_mat3();
        let gm = glam::Mat3::from_quat(gq(q));
        assert!(mat3_approx_eq(m, wm3(gm)));
    }
}

#[test]
fn comprehensive_to_mat4_vs_glam() {
    let quats = [
        Quat::from_axis_angle(Vec3::new(1.0f32, 0.0, 0.0), Rad(0.5)),
        Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), Rad(1.0)),
        Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), Rad(1.5)),
    ];
    for q in quats {
        let m = q.to_mat4();
        let gm = glam::Mat4::from_quat(gq(q));
        assert!(mat4_approx_eq(m, wm4(gm)));
    }
}

#[test]
fn comprehensive_slerp_vs_glam() {
    let a = Quat::from_axis_angle(Vec3::new(1.0f32, 0.0, 0.0), Rad(0.2));
    let b = Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), Rad(1.5));
    let ga = gq(a);
    let gb = gq(b);
    for i in 0..=10 {
        let t = i as f32 / 10.0;
        let s = a.slerp(b, t);
        let gs = ga.slerp(gb, t);
        assert!(quat_approx_eq(s, wq(gs)), "slerp failed at t={}", t);
    }
}

#[test]
fn look_to_rh() {
    let dir = Vec3::new(0.0f32, 0.0, -1.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let q = Quat::<f32>::look_to_rh(dir, up);
    let g = glam::Quat::look_to_rh(gv3(dir), gv3(up));
    assert!(quat_approx_eq(q, wq(g)));
}

#[test]
fn look_to_lh() {
    let dir = Vec3::new(0.0f32, 0.0, -1.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let q = Quat::<f32>::look_to_lh(dir, up);
    let g = glam::Quat::look_to_lh(gv3(dir), gv3(up));
    assert!(quat_approx_eq(q, wq(g)));
}

#[test]
fn look_at_rh() {
    let eye = Vec3::new(0.0f32, 5.0, 10.0);
    let center = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let q = Quat::<f32>::look_at_rh(eye, center, up);
    let g = glam::Quat::look_at_rh(gv3(eye), gv3(center), gv3(up));
    assert!(quat_approx_eq(q, wq(g)));
}

#[test]
fn look_at_lh() {
    let eye = Vec3::new(0.0f32, 5.0, 10.0);
    let center = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let q = Quat::<f32>::look_at_lh(eye, center, up);
    let g = glam::Quat::look_at_lh(gv3(eye), gv3(center), gv3(up));
    assert!(quat_approx_eq(q, wq(g)));
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
        let qrh = Quat::<f32>::look_to_rh(dir, up);
        let grh = glam::Quat::look_to_rh(gv3(dir), gv3(up));
        assert!(quat_approx_eq(qrh, wq(grh)));

        let qlh = Quat::<f32>::look_to_lh(dir, up);
        let glh = glam::Quat::look_to_lh(gv3(dir), gv3(up));
        assert!(quat_approx_eq(qlh, wq(glh)));
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
        let qrh = Quat::<f32>::look_at_rh(eye, center, up);
        let grh = glam::Quat::look_at_rh(gv3(eye), gv3(center), gv3(up));
        assert!(quat_approx_eq(qrh, wq(grh)));

        let qlh = Quat::<f32>::look_at_lh(eye, center, up);
        let glh = glam::Quat::look_at_lh(gv3(eye), gv3(center), gv3(up));
        assert!(quat_approx_eq(qlh, wq(glh)));
    }
}
