use ::rand::Rng;
use crate::{Vec2, Vec3, Vec4, Quat};
use ::rand::distributions::{Distribution, Standard};

impl<T> Distribution<Vec2<T>> for Standard
where Standard: Distribution<T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec2<T> {
        return Vec2::new(rng.gen(), rng.gen())
    }
}

impl<T> Distribution<Vec3<T>> for Standard
where Standard: Distribution<T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3<T> {
        return Vec3::new(rng.gen(), rng.gen(), rng.gen())
    }
}

impl<T> Distribution<Vec4<T>> for Standard
where Standard: Distribution<T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec4<T> {
        return Vec4::new(rng.gen(), rng.gen(), rng.gen(), rng.gen())
    }
}

impl<T> Distribution<Quat<T>> for Standard
where Standard: Distribution<T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quat<T> {
        return Quat::new(rng.gen(), rng.gen(), rng.gen(), rng.gen())
    }
}
