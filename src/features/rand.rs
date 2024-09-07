use num_traits::Float;
use rand::distributions::{Distribution, Standard};
use crate::{Vec2, Vec3, Vec4};
use crate::{Mat2, Mat3, Mat4};
use crate::units::{Deg, Rad, Tau};

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

impl<T: Float> Distribution<Deg<T>> for Standard
where Standard: Distribution<T> {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Deg<T> {
        Deg::new(rng.gen())
    }
}

impl<T: Float> Distribution<Rad<T>> for Standard
where Standard: Distribution<T> {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Rad<T> {
        Rad::new(rng.gen())
    }
}

impl<T: Float> Distribution<Tau<T>> for Standard
where Standard: Distribution<T> {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Tau<T> {
        Tau::new(rng.gen())
    }
}
