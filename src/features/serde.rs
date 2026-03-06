use core::fmt;
use core::marker::PhantomData;
use ::serde::ser::SerializeStruct;
use crate::{Vec2, Vec3, Vec4, Mat2, Mat3, Mat4, Quat};
use ::serde::de::{self, MapAccess, SeqAccess, Visitor};
use ::serde::{Serialize, Deserialize, Serializer, Deserializer};

macro_rules! field_enum {
    ($name:ident { $($variant:ident => $str:expr),+ $(,)? }) => {
        #[allow(clippy::enum_variant_names)]
        enum $name { $($variant),+ }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct FieldVisitor;
                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = $name;
                    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        return f.write_str(concat!("one of: ", $($str, ", "),+))
                    }
                    fn visit_str<E: de::Error>(self, value: &str) -> Result<$name, E> {
                        match value {
                            $($str => return Ok($name::$variant),)+
                            _ => return Err(de::Error::unknown_field(value, &[$($str),+])),
                        }
                    }
                }
                return deserializer.deserialize_identifier(FieldVisitor)
            }
        }
    };
}

impl<T: Serialize> Serialize for Vec2<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Vec2", 2)?;
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        return state.end()
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Vec2<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        field_enum!(Field { X => "x", Y => "y" });

        struct Vec2Visitor<T>(PhantomData<T>);

        impl<'de, T: Deserialize<'de>> Visitor<'de> for Vec2Visitor<T> {
            type Value = Vec2<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                return formatter.write_str("struct Vec2")
            }

            fn visit_seq<V: SeqAccess<'de>>(self, mut seq: V) -> Result<Vec2<T>, V::Error> {
                let x = seq.next_element()?.ok_or_else(|| return de::Error::invalid_length(0, &self))?;
                let y = seq.next_element()?.ok_or_else(|| return de::Error::invalid_length(1, &self))?;
                return Ok(Vec2::new(x, y))
            }

            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Vec2<T>, V::Error> {
                let mut x = None;
                let mut y = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::X => { x = Some(map.next_value()?); }
                        Field::Y => { y = Some(map.next_value()?); }
                    }
                }
                let x = x.ok_or_else(|| return de::Error::missing_field("x"))?;
                let y = y.ok_or_else(|| return de::Error::missing_field("y"))?;
                return Ok(Vec2::new(x, y))
            }
        }

        const FIELDS: &[&str] = &["x", "y"];
        return deserializer.deserialize_struct("Vec2", FIELDS, Vec2Visitor(PhantomData))
    }
}

impl<T: Serialize> Serialize for Vec3<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Vec3", 3)?;
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        state.serialize_field("z", &self.z)?;
        return state.end()
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Vec3<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        field_enum!(Field { X => "x", Y => "y", Z => "z" });

        struct Vec3Visitor<T>(PhantomData<T>);

        impl<'de, T: Deserialize<'de>> Visitor<'de> for Vec3Visitor<T> {
            type Value = Vec3<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                return formatter.write_str("struct Vec3")
            }

            fn visit_seq<V: SeqAccess<'de>>(self, mut seq: V) -> Result<Vec3<T>, V::Error> {
                let x = seq.next_element()?.ok_or_else(|| return de::Error::invalid_length(0, &self))?;
                let y = seq.next_element()?.ok_or_else(|| return de::Error::invalid_length(1, &self))?;
                let z = seq.next_element()?.ok_or_else(|| return de::Error::invalid_length(2, &self))?;
                return Ok(Vec3::new(x, y, z))
            }

            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Vec3<T>, V::Error> {
                let mut x = None;
                let mut y = None;
                let mut z = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::X => { x = Some(map.next_value()?); }
                        Field::Y => { y = Some(map.next_value()?); }
                        Field::Z => { z = Some(map.next_value()?); }
                    }
                }
                let x = x.ok_or_else(|| return de::Error::missing_field("x"))?;
                let y = y.ok_or_else(|| return de::Error::missing_field("y"))?;
                let z = z.ok_or_else(|| return de::Error::missing_field("z"))?;
                return Ok(Vec3::new(x, y, z))
            }
        }

        const FIELDS: &[&str] = &["x", "y", "z"];
        return deserializer.deserialize_struct("Vec3", FIELDS, Vec3Visitor(PhantomData))
    }
}

impl<T: Serialize> Serialize for Vec4<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Vec4", 4)?;
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        state.serialize_field("z", &self.z)?;
        state.serialize_field("w", &self.w)?;
        return state.end()
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Vec4<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        field_enum!(Field { X => "x", Y => "y", Z => "z", W => "w" });

        struct Vec4Visitor<T>(PhantomData<T>);

        impl<'de, T: Deserialize<'de>> Visitor<'de> for Vec4Visitor<T> {
            type Value = Vec4<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                return formatter.write_str("struct Vec4")
            }

            fn visit_seq<V: SeqAccess<'de>>(self, mut seq: V) -> Result<Vec4<T>, V::Error> {
                let x = seq.next_element()?.ok_or_else(|| return de::Error::invalid_length(0, &self))?;
                let y = seq.next_element()?.ok_or_else(|| return de::Error::invalid_length(1, &self))?;
                let z = seq.next_element()?.ok_or_else(|| return de::Error::invalid_length(2, &self))?;
                let w = seq.next_element()?.ok_or_else(|| return de::Error::invalid_length(3, &self))?;
                return Ok(Vec4::new(x, y, z, w))
            }

            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Vec4<T>, V::Error> {
                let mut x = None;
                let mut y = None;
                let mut z = None;
                let mut w = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::X => { x = Some(map.next_value()?); }
                        Field::Y => { y = Some(map.next_value()?); }
                        Field::Z => { z = Some(map.next_value()?); }
                        Field::W => { w = Some(map.next_value()?); }
                    }
                }
                let x = x.ok_or_else(|| return de::Error::missing_field("x"))?;
                let y = y.ok_or_else(|| return de::Error::missing_field("y"))?;
                let z = z.ok_or_else(|| return de::Error::missing_field("z"))?;
                let w = w.ok_or_else(|| return de::Error::missing_field("w"))?;
                return Ok(Vec4::new(x, y, z, w))
            }
        }

        const FIELDS: &[&str] = &["x", "y", "z", "w"];
        return deserializer.deserialize_struct("Vec4", FIELDS, Vec4Visitor(PhantomData))
    }
}

impl<T: Serialize> Serialize for Mat2<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Mat2", 2)?;
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        return state.end()
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Mat2<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        field_enum!(Field { XAxis => "x", YAxis => "y" });

        struct Mat2Visitor<T>(PhantomData<T>);

        impl<'de, T: Deserialize<'de>> Visitor<'de> for Mat2Visitor<T> {
            type Value = Mat2<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                return formatter.write_str("struct Mat2")
            }

            fn visit_seq<V: SeqAccess<'de>>(self, mut seq: V) -> Result<Mat2<T>, V::Error> {
                let x = seq.next_element()?.ok_or_else(|| return de::Error::invalid_length(0, &self))?;
                let y = seq.next_element()?.ok_or_else(|| return de::Error::invalid_length(1, &self))?;
                return Ok(Mat2::new(x, y))
            }

            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Mat2<T>, V::Error> {
                let mut x = None;
                let mut y = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::XAxis => { x = Some(map.next_value()?); }
                        Field::YAxis => { y = Some(map.next_value()?); }
                    }
                }
                let x = x.ok_or_else(|| return de::Error::missing_field("x"))?;
                let y = y.ok_or_else(|| return de::Error::missing_field("y"))?;
                return Ok(Mat2::new(x, y))
            }
        }

        const FIELDS: &[&str] = &["x", "y"];
        return deserializer.deserialize_struct("Mat2", FIELDS, Mat2Visitor(PhantomData))
    }
}

impl<T: Serialize> Serialize for Mat3<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Mat3", 3)?;
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        state.serialize_field("z", &self.z)?;
        return state.end()
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Mat3<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        field_enum!(Field { XAxis => "x", YAxis => "y", ZAxis => "z" });

        struct Mat3Visitor<T>(PhantomData<T>);

        impl<'de, T: Deserialize<'de>> Visitor<'de> for Mat3Visitor<T> {
            type Value = Mat3<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                return formatter.write_str("struct Mat3")
            }

            fn visit_seq<V: SeqAccess<'de>>(self, mut seq: V) -> Result<Mat3<T>, V::Error> {
                let x = seq.next_element()?.ok_or_else(|| return de::Error::invalid_length(0, &self))?;
                let y = seq.next_element()?.ok_or_else(|| return de::Error::invalid_length(1, &self))?;
                let z = seq.next_element()?.ok_or_else(|| return de::Error::invalid_length(2, &self))?;
                return Ok(Mat3::new(x, y, z))
            }

            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Mat3<T>, V::Error> {
                let mut x = None;
                let mut y = None;
                let mut z = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::XAxis => { x = Some(map.next_value()?); }
                        Field::YAxis => { y = Some(map.next_value()?); }
                        Field::ZAxis => { z = Some(map.next_value()?); }
                    }
                }
                let x = x.ok_or_else(|| return de::Error::missing_field("x"))?;
                let y = y.ok_or_else(|| return de::Error::missing_field("y"))?;
                let z = z.ok_or_else(|| return de::Error::missing_field("z"))?;
                return Ok(Mat3::new(x, y, z))
            }
        }

        const FIELDS: &[&str] = &["x", "y", "z"];
        return deserializer.deserialize_struct("Mat3", FIELDS, Mat3Visitor(PhantomData))
    }
}

impl<T: Serialize> Serialize for Mat4<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Mat4", 4)?;
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        state.serialize_field("z", &self.z)?;
        state.serialize_field("w_axis", &self.w_axis)?;
        return state.end()
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Mat4<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        field_enum!(Field { XAxis => "x", YAxis => "y", ZAxis => "z", WAxis => "w_axis" });

        struct Mat4Visitor<T>(PhantomData<T>);

        impl<'de, T: Deserialize<'de>> Visitor<'de> for Mat4Visitor<T> {
            type Value = Mat4<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                return formatter.write_str("struct Mat4")
            }

            fn visit_seq<V: SeqAccess<'de>>(self, mut seq: V) -> Result<Mat4<T>, V::Error> {
                let x = seq.next_element()?.ok_or_else(|| return de::Error::invalid_length(0, &self))?;
                let y = seq.next_element()?.ok_or_else(|| return de::Error::invalid_length(1, &self))?;
                let z = seq.next_element()?.ok_or_else(|| return de::Error::invalid_length(2, &self))?;
                let w = seq.next_element()?.ok_or_else(|| return de::Error::invalid_length(3, &self))?;
                return Ok(Mat4::new(x, y, z, w))
            }

            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Mat4<T>, V::Error> {
                let mut x = None;
                let mut y = None;
                let mut z = None;
                let mut w_axis = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::XAxis => { x = Some(map.next_value()?); }
                        Field::YAxis => { y = Some(map.next_value()?); }
                        Field::ZAxis => { z = Some(map.next_value()?); }
                        Field::WAxis => { w_axis = Some(map.next_value()?); }
                    }
                }
                let x = x.ok_or_else(|| return de::Error::missing_field("x"))?;
                let y = y.ok_or_else(|| return de::Error::missing_field("y"))?;
                let z = z.ok_or_else(|| return de::Error::missing_field("z"))?;
                let w_axis = w_axis.ok_or_else(|| return de::Error::missing_field("w_axis"))?;
                return Ok(Mat4::new(x, y, z, w_axis))
            }
        }

        const FIELDS: &[&str] = &["x", "y", "z", "w_axis"];
        return deserializer.deserialize_struct("Mat4", FIELDS, Mat4Visitor(PhantomData))
    }
}

impl<T: Serialize> Serialize for Quat<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Quat", 4)?;
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        state.serialize_field("z", &self.z)?;
        state.serialize_field("w", &self.w)?;
        return state.end()
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Quat<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        field_enum!(Field { X => "x", Y => "y", Z => "z", W => "w" });

        struct QuatVisitor<T>(PhantomData<T>);

        impl<'de, T: Deserialize<'de>> Visitor<'de> for QuatVisitor<T> {
            type Value = Quat<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                return formatter.write_str("struct Quat")
            }

            fn visit_seq<V: SeqAccess<'de>>(self, mut seq: V) -> Result<Quat<T>, V::Error> {
                let x = seq.next_element()?.ok_or_else(|| return de::Error::invalid_length(0, &self))?;
                let y = seq.next_element()?.ok_or_else(|| return de::Error::invalid_length(1, &self))?;
                let z = seq.next_element()?.ok_or_else(|| return de::Error::invalid_length(2, &self))?;
                let w = seq.next_element()?.ok_or_else(|| return de::Error::invalid_length(3, &self))?;
                return Ok(Quat::new(x, y, z, w))
            }

            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Quat<T>, V::Error> {
                let mut x = None;
                let mut y = None;
                let mut z = None;
                let mut w = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::X => { x = Some(map.next_value()?); }
                        Field::Y => { y = Some(map.next_value()?); }
                        Field::Z => { z = Some(map.next_value()?); }
                        Field::W => { w = Some(map.next_value()?); }
                    }
                }
                let x = x.ok_or_else(|| return de::Error::missing_field("x"))?;
                let y = y.ok_or_else(|| return de::Error::missing_field("y"))?;
                let z = z.ok_or_else(|| return de::Error::missing_field("z"))?;
                let w = w.ok_or_else(|| return de::Error::missing_field("w"))?;
                return Ok(Quat::new(x, y, z, w))
            }
        }

        const FIELDS: &[&str] = &["x", "y", "z", "w"];
        return deserializer.deserialize_struct("Quat", FIELDS, QuatVisitor(PhantomData))
    }
}
