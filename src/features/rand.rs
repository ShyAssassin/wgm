use crate::{Vec2, Vec3, Vec4};
use crate::{Mat2, Mat3, Mat4};
use rand::distributions::{Distribution, Standard};

impl<T> Distribution<Vec2<T>> for Standard
where Standard: Distribution<T> {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Vec2<T> {
        Vec2::new(rng.gen(), rng.gen())
    }
}

impl<T> Distribution<Vec3<T>> for Standard
where Standard: Distribution<T> {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Vec3<T> {
        Vec3::new(rng.gen(), rng.gen(), rng.gen())
    }
}

impl<T> Distribution<Vec4<T>> for Standard
where Standard: Distribution<T> {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Vec4<T> {
        Vec4::new(rng.gen(), rng.gen(), rng.gen(), rng.gen())
    }
}

impl<T> Distribution<Mat2<T>> for Standard
where Standard: Distribution<T> {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Mat2<T> {
        Mat2::new(rng.gen::<Vec2<T>>(), rng.gen::<Vec2<T>>())
    }
}

impl<T> Distribution<Mat3<T>> for Standard
where Standard: Distribution<T> {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Mat3<T> {
        Mat3::new(rng.gen::<Vec3<T>>(), rng.gen::<Vec3<T>>(), rng.gen::<Vec3<T>>())
    }
}

impl<T> Distribution<Mat4<T>> for Standard
where Standard: Distribution<T> {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Mat4<T> {
        Mat4::new(rng.gen::<Vec4<T>>(), rng.gen::<Vec4<T>>(), rng.gen::<Vec4<T>>(), rng.gen::<Vec4<T>>())
    }
}
