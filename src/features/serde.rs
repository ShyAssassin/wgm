use crate::{Vec2, Vec3, Vec4};
use crate::{Mat2, Mat3, Mat4};
use serde::{Serialize, Deserialize};

impl<T: Serialize + Copy> Serialize for Vec2<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        return [self.x, self.y].serialize(serializer)
    }

}
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Vec2<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let [x, y]: [T; 2] = Deserialize::deserialize(deserializer)?;
        return Ok(Vec2::new(x, y))
    }
}


impl<T: Serialize + Copy> Serialize for Vec3<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        return [self.x, self.y, self.z].serialize(serializer)
    }
}
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Vec3<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let [x, y, z]: [T; 3] = Deserialize::deserialize(deserializer)?;
        return Ok(Vec3::new(x, y, z))
    }
}


impl<T: Serialize + Copy> Serialize for Vec4<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        return [self.x, self.y, self.z, self.w].serialize(serializer)
    }
}
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Vec4<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let [x, y, z, w]: [T; 4] = Deserialize::deserialize(deserializer)?;
        return Ok(Vec4::new(x, y, z, w))
    }
}


impl<T: Serialize + Copy> Serialize for Mat2<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        return [self.x, self.y].serialize(serializer)
    }
}
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Mat2<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let [x, y]: [Vec2<T>; 2] = Deserialize::deserialize(deserializer)?;
        return Ok(Mat2::new(x, y))
    }
}


impl<T: Serialize + Copy> Serialize for Mat3<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        return [self.x, self.y, self.z].serialize(serializer)
    }
}
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Mat3<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let [x, y, z]: [Vec3<T>; 3] = Deserialize::deserialize(deserializer)?;
        return Ok(Mat3::new(x, y, z))
    }
}


impl<T: Serialize + Copy> Serialize for Mat4<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        return [self.x, self.y, self.z, self.w].serialize(serializer)
    }
}
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Mat4<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let [x, y, z, w]: [Vec4<T>; 4] = Deserialize::deserialize(deserializer)?;
        return Ok(Mat4::new(x, y, z, w))
    }
}
