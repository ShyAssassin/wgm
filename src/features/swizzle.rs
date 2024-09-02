use crate::{Vec2, Vec3, Vec4};

#[doc(hidden)]
impl<T: Copy> Vec2<T> {
    pub fn x(&self) -> T {
        return self.x
    }

    pub fn y(&self) -> T {
        return self.y
    }

    pub fn xx(&self) -> Vec2<T> {
        return Vec2::new(self.x, self.x)
    }

    pub fn xy(&self) -> Vec2<T> {
        return Vec2::new(self.x, self.y)
    }

    pub fn yx(&self) -> Vec2<T> {
        return Vec2::new(self.y, self.x)
    }

    pub fn yy(&self) -> Vec2<T> {
        return Vec2::new(self.y, self.y)
    }

    pub fn xxx(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.x, self.x)
    }

    pub fn xxy(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.x, self.y)
    }

    pub fn xyx(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.y, self.x)
    }

    pub fn xyy(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.y, self.y)
    }

    pub fn yxx(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.x, self.x)
    }

    pub fn yxy(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.x, self.y)
    }

    pub fn yyx(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.y, self.x)
    }

    pub fn yyy(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.y, self.y)
    }

    pub fn xxxx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.x, self.x)
    }

    pub fn xxxy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.x, self.y)
    }

    pub fn xxyx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.y, self.x)
    }

    pub fn xxyy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.y, self.y)
    }

    pub fn xyxx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.x, self.x)
    }

    pub fn xyxy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.x, self.y)
    }

    pub fn xyyx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.y, self.x)
    }

    pub fn xyyy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.y, self.y)
    }

    pub fn yxxx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.x, self.x)
    }

    pub fn yxxy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.x, self.y)
    }

    pub fn yxyx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.y, self.x)
    }

    pub fn yxyy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.y, self.y)
    }

    pub fn yyxx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.x, self.x)
    }

    pub fn yyxy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.x, self.y)
    }

    pub fn yyyx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.y, self.x)
    }

    pub fn yyyy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.y, self.y)
    }
}

#[doc(hidden)]
impl<T: Copy> Vec3<T> {
    pub fn x(&self) -> T {
        return self.x
    }

    pub fn y(&self) -> T {
        return self.y
    }

    pub fn z(&self) -> T {
        return self.z
    }

    pub fn xx(&self) -> Vec2<T> {
        return Vec2::new(self.x, self.x)
    }

    pub fn xy(&self) -> Vec2<T> {
        return Vec2::new(self.x, self.y)
    }

    pub fn xz(&self) -> Vec2<T> {
        return Vec2::new(self.x, self.z)
    }

    pub fn yx(&self) -> Vec2<T> {
        return Vec2::new(self.y, self.x)
    }

    pub fn yy(&self) -> Vec2<T> {
        return Vec2::new(self.y, self.y)
    }

    pub fn yz(&self) -> Vec2<T> {
        return Vec2::new(self.y, self.z)
    }

    pub fn zx(&self) -> Vec2<T> {
        return Vec2::new(self.z, self.x)
    }

    pub fn zy(&self) -> Vec2<T> {
        return Vec2::new(self.z, self.y)
    }

    pub fn zz(&self) -> Vec2<T> {
        return Vec2::new(self.z, self.z)
    }

    pub fn xxx(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.x, self.x)
    }

    pub fn xxy(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.x, self.y)
    }

    pub fn xxz(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.x, self.z)
    }

    pub fn xyx(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.y, self.x)
    }

    pub fn xyy(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.y, self.y)
    }

    pub fn xyz(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.y, self.z)
    }

    pub fn xzx(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.z, self.x)
    }

    pub fn xzy(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.z, self.y)
    }

    pub fn xzz(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.z, self.z)
    }

    pub fn yxx(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.x, self.x)
    }

    pub fn yxy(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.x, self.y)
    }

    pub fn yxz(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.x, self.z)
    }

    pub fn yyx(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.y, self.x)
    }

    pub fn yyy(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.y, self.y)
    }

    pub fn yyz(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.y, self.z)
    }

    pub fn yzx(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.z, self.x)
    }

    pub fn yzy(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.z, self.y)
    }

    pub fn yzz(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.z, self.z)
    }

    pub fn zxx(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.x, self.x)
    }

    pub fn zxy(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.x, self.y)
    }

    pub fn zxz(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.x, self.z)
    }

    pub fn zyx(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.y, self.x)
    }

    pub fn zyy(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.y, self.y)
    }

    pub fn zyz(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.y, self.z)
    }

    pub fn zzx(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.z, self.x)
    }

    pub fn zzy(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.z, self.y)
    }

    pub fn zzz(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.z, self.z)
    }

    pub fn xxxx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.x, self.x)
    }

    pub fn xxxy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.x, self.y)
    }

    pub fn xxxz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.x, self.z)
    }

    pub fn xxyx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.y, self.x)
    }

    pub fn xxyy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.y, self.y)
    }

    pub fn xxyz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.y, self.z)
    }

    pub fn xxzx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.z, self.x)
    }

    pub fn xxzy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.z, self.y)
    }

    pub fn xxzz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.z, self.z)
    }

    pub fn xyxx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.x, self.x)
    }

    pub fn xyxy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.x, self.y)
    }

    pub fn xyxz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.x, self.z)
    }

    pub fn xyyx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.y, self.x)
    }

    pub fn xyyy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.y, self.y)
    }

    pub fn xyyz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.y, self.z)
    }

    pub fn xyzx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.z, self.x)
    }

    pub fn xyzy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.z, self.y)
    }

    pub fn xyzz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.z, self.z)
    }

    pub fn xzxx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.x, self.x)
    }

    pub fn xzxy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.x, self.y)
    }

    pub fn xzxz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.x, self.z)
    }

    pub fn xzyx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.y, self.x)
    }

    pub fn xzyy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.y, self.y)
    }

    pub fn xzyz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.y, self.z)
    }

    pub fn xzzx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.z, self.x)
    }

    pub fn xzzy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.z, self.y)
    }

    pub fn xzzz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.z, self.z)
    }

    pub fn yxxx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.x, self.x)
    }

    pub fn yxxy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.x, self.y)
    }

    pub fn yxxz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.x, self.z)
    }

    pub fn yxyx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.y, self.x)
    }

    pub fn yxyy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.y, self.y)
    }

    pub fn yxyz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.y, self.z)
    }

    pub fn yxzx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.z, self.x)
    }

    pub fn yxzy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.z, self.y)
    }

    pub fn yxzz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.z, self.z)
    }

    pub fn yyxx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.x, self.x)
    }

    pub fn yyxy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.x, self.y)
    }

    pub fn yyxz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.x, self.z)
    }

    pub fn yyyx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.y, self.x)
    }

    pub fn yyyy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.y, self.y)
    }

    pub fn yyyz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.y, self.z)
    }

    pub fn yyzx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.z, self.x)
    }

    pub fn yyzy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.z, self.y)
    }

    pub fn yyzz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.z, self.z)
    }

    pub fn yzxx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.x, self.x)
    }

    pub fn yzxy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.x, self.y)
    }

    pub fn yzxz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.x, self.z)
    }

    pub fn yzyx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.y, self.x)
    }

    pub fn yzyy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.y, self.y)
    }

    pub fn yzyz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.y, self.z)
    }

    pub fn yzzx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.z, self.x)
    }

    pub fn yzzy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.z, self.y)
    }

    pub fn yzzz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.z, self.z)
    }

    pub fn zxxx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.x, self.x)
    }

    pub fn zxxy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.x, self.y)
    }

    pub fn zxxz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.x, self.z)
    }

    pub fn zxyx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.y, self.x)
    }

    pub fn zxyy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.y, self.y)
    }

    pub fn zxyz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.y, self.z)
    }

    pub fn zxzx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.z, self.x)
    }

    pub fn zxzy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.z, self.y)
    }

    pub fn zxzz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.z, self.z)
    }

    pub fn zyxx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.x, self.x)
    }

    pub fn zyxy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.x, self.y)
    }

    pub fn zyxz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.x, self.z)
    }

    pub fn zyyx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.y, self.x)
    }

    pub fn zyyy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.y, self.y)
    }

    pub fn zyyz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.y, self.z)
    }

    pub fn zyzx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.z, self.x)
    }

    pub fn zyzy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.z, self.y)
    }

    pub fn zyzz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.z, self.z)
    }

    pub fn zzxx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.x, self.x)
    }

    pub fn zzxy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.x, self.y)
    }

    pub fn zzxz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.x, self.z)
    }

    pub fn zzyx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.y, self.x)
    }

    pub fn zzyy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.y, self.y)
    }

    pub fn zzyz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.y, self.z)
    }

    pub fn zzzx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.z, self.x)
    }

    pub fn zzzy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.z, self.y)
    }

    pub fn zzzz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.z, self.z)
    }
}

#[doc(hidden)]
impl<T: Copy> Vec4<T> {
    pub fn x(&self) -> T {
        return self.x
    }

    pub fn y(&self) -> T {
        return self.y
    }

    pub fn z(&self) -> T {
        return self.z
    }

    pub fn w(&self) -> T {
        return self.w
    }

    pub fn xx(&self) -> Vec2<T> {
        return Vec2::new(self.x, self.x)
    }

    pub fn xy(&self) -> Vec2<T> {
        return Vec2::new(self.x, self.y)
    }

    pub fn xz(&self) -> Vec2<T> {
        return Vec2::new(self.x, self.z)
    }

    pub fn xw(&self) -> Vec2<T> {
        return Vec2::new(self.x, self.w)
    }

    pub fn yx(&self) -> Vec2<T> {
        return Vec2::new(self.y, self.x)
    }

    pub fn yy(&self) -> Vec2<T> {
        return Vec2::new(self.y, self.y)
    }

    pub fn yz(&self) -> Vec2<T> {
        return Vec2::new(self.y, self.z)
    }

    pub fn yw(&self) -> Vec2<T> {
        return Vec2::new(self.y, self.w)
    }

    pub fn zx(&self) -> Vec2<T> {
        return Vec2::new(self.z, self.x)
    }

    pub fn zy(&self) -> Vec2<T> {
        return Vec2::new(self.z, self.y)
    }

    pub fn zz(&self) -> Vec2<T> {
        return Vec2::new(self.z, self.z)
    }

    pub fn zw(&self) -> Vec2<T> {
        return Vec2::new(self.z, self.w)
    }

    pub fn wx(&self) -> Vec2<T> {
        return Vec2::new(self.w, self.x)
    }

    pub fn wy(&self) -> Vec2<T> {
        return Vec2::new(self.w, self.y)
    }

    pub fn wz(&self) -> Vec2<T> {
        return Vec2::new(self.w, self.z)
    }

    pub fn ww(&self) -> Vec2<T> {
        return Vec2::new(self.w, self.w)
    }

    pub fn xxx(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.x, self.x)
    }

    pub fn xxy(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.x, self.y)
    }

    pub fn xxz(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.x, self.z)
    }

    pub fn xxw(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.x, self.w)
    }

    pub fn xyx(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.y, self.x)
    }

    pub fn xyy(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.y, self.y)
    }

    pub fn xyz(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.y, self.z)
    }

    pub fn xyw(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.y, self.w)
    }

    pub fn xzx(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.z, self.x)
    }

    pub fn xzy(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.z, self.y)
    }

    pub fn xzz(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.z, self.z)
    }

    pub fn xzw(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.z, self.w)
    }

    pub fn xwx(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.w, self.x)
    }

    pub fn xwy(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.w, self.y)
    }

    pub fn xwz(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.w, self.z)
    }

    pub fn xww(&self) -> Vec3<T> {
        return Vec3::new(self.x, self.w, self.w)
    }

    pub fn yxx(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.x, self.x)
    }

    pub fn yxy(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.x, self.y)
    }

    pub fn yxz(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.x, self.z)
    }

    pub fn yxw(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.x, self.w)
    }

    pub fn yyx(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.y, self.x)
    }

    pub fn yyy(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.y, self.y)
    }

    pub fn yyz(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.y, self.z)
    }

    pub fn yyw(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.y, self.w)
    }

    pub fn yzx(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.z, self.x)
    }

    pub fn yzy(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.z, self.y)
    }

    pub fn yzz(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.z, self.z)
    }

    pub fn yzw(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.z, self.w)
    }

    pub fn ywx(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.w, self.x)
    }

    pub fn ywy(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.w, self.y)
    }

    pub fn ywz(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.w, self.z)
    }

    pub fn yww(&self) -> Vec3<T> {
        return Vec3::new(self.y, self.w, self.w)
    }

    pub fn zxx(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.x, self.x)
    }

    pub fn zxy(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.x, self.y)
    }

    pub fn zxz(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.x, self.z)
    }

    pub fn zxw(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.x, self.w)
    }

    pub fn zyx(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.y, self.x)
    }

    pub fn zyy(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.y, self.y)
    }

    pub fn zyz(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.y, self.z)
    }

    pub fn zyw(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.y, self.w)
    }

    pub fn zzx(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.z, self.x)
    }

    pub fn zzy(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.z, self.y)
    }

    pub fn zzz(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.z, self.z)
    }

    pub fn zzw(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.z, self.w)
    }

    pub fn zwx(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.w, self.x)
    }

    pub fn zwy(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.w, self.y)
    }

    pub fn zwz(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.w, self.z)
    }

    pub fn zww(&self) -> Vec3<T> {
        return Vec3::new(self.z, self.w, self.w)
    }

    pub fn wxx(&self) -> Vec3<T> {
        return Vec3::new(self.w, self.x, self.x)
    }

    pub fn wxy(&self) -> Vec3<T> {
        return Vec3::new(self.w, self.x, self.y)
    }

    pub fn wxz(&self) -> Vec3<T> {
        return Vec3::new(self.w, self.x, self.z)
    }

    pub fn wxw(&self) -> Vec3<T> {
        return Vec3::new(self.w, self.x, self.w)
    }

    pub fn wyx(&self) -> Vec3<T> {
        return Vec3::new(self.w, self.y, self.x)
    }

    pub fn wyy(&self) -> Vec3<T> {
        return Vec3::new(self.w, self.y, self.y)
    }

    pub fn wyz(&self) -> Vec3<T> {
        return Vec3::new(self.w, self.y, self.z)
    }

    pub fn wyw(&self) -> Vec3<T> {
        return Vec3::new(self.w, self.y, self.w)
    }

    pub fn wzx(&self) -> Vec3<T> {
        return Vec3::new(self.w, self.z, self.x)
    }

    pub fn wzy(&self) -> Vec3<T> {
        return Vec3::new(self.w, self.z, self.y)
    }

    pub fn wzz(&self) -> Vec3<T> {
        return Vec3::new(self.w, self.z, self.z)
    }

    pub fn wzw(&self) -> Vec3<T> {
        return Vec3::new(self.w, self.z, self.w)
    }

    pub fn wwx(&self) -> Vec3<T> {
        return Vec3::new(self.w, self.w, self.x)
    }

    pub fn wwy(&self) -> Vec3<T> {
        return Vec3::new(self.w, self.w, self.y)
    }

    pub fn wwz(&self) -> Vec3<T> {
        return Vec3::new(self.w, self.w, self.z)
    }

    pub fn www(&self) -> Vec3<T> {
        return Vec3::new(self.w, self.w, self.w)
    }

    pub fn xxxx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.x, self.x)
    }

    pub fn xxxy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.x, self.y)
    }

    pub fn xxxz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.x, self.z)
    }

    pub fn xxxw(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.x, self.w)
    }

    pub fn xxyx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.y, self.x)
    }

    pub fn xxyy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.y, self.y)
    }

    pub fn xxyz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.y, self.z)
    }

    pub fn xxyw(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.y, self.w)
    }

    pub fn xxzx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.z, self.x)
    }

    pub fn xxzy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.z, self.y)
    }

    pub fn xxzz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.z, self.z)
    }

    pub fn xxzw(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.z, self.w)
    }

    pub fn xxwx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.w, self.x)
    }

    pub fn xxwy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.w, self.y)
    }

    pub fn xxwz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.w, self.z)
    }

    pub fn xxww(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.x, self.w, self.w)
    }

    pub fn xyxx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.x, self.x)
    }

    pub fn xyxy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.x, self.y)
    }

    pub fn xyxz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.x, self.z)
    }

    pub fn xyxw(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.x, self.w)
    }

    pub fn xyyx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.y, self.x)
    }

    pub fn xyyy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.y, self.y)
    }

    pub fn xyyz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.y, self.z)
    }

    pub fn xyyw(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.y, self.w)
    }

    pub fn xyzx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.z, self.x)
    }

    pub fn xyzy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.z, self.y)
    }

    pub fn xyzz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.z, self.z)
    }

    pub fn xyzw(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.z, self.w)
    }

    pub fn xywx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.w, self.x)
    }

    pub fn xywy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.w, self.y)
    }

    pub fn xywz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.w, self.z)
    }

    pub fn xyww(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.y, self.w, self.w)
    }

    pub fn xzxx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.x, self.x)
    }

    pub fn xzxy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.x, self.y)
    }

    pub fn xzxz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.x, self.z)
    }

    pub fn xzxw(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.x, self.w)
    }

    pub fn xzyx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.y, self.x)
    }

    pub fn xzyy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.y, self.y)
    }

    pub fn xzyz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.y, self.z)
    }

    pub fn xzyw(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.y, self.w)
    }

    pub fn xzzx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.z, self.x)
    }

    pub fn xzzy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.z, self.y)
    }

    pub fn xzzz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.z, self.z)
    }

    pub fn xzzw(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.z, self.w)
    }

    pub fn xzwx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.w, self.x)
    }

    pub fn xzwy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.w, self.y)
    }

    pub fn xzwz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.w, self.z)
    }

    pub fn xzww(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.z, self.w, self.w)
    }

    pub fn xwxx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.w, self.x, self.x)
    }

    pub fn xwxy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.w, self.x, self.y)
    }

    pub fn xwxz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.w, self.x, self.z)
    }

    pub fn xwxw(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.w, self.x, self.w)
    }

    pub fn xwyx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.w, self.y, self.x)
    }

    pub fn xwyy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.w, self.y, self.y)
    }

    pub fn xwyz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.w, self.y, self.z)
    }

    pub fn xwyw(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.w, self.y, self.w)
    }

    pub fn xwzx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.w, self.z, self.x)
    }

    pub fn xwzy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.w, self.z, self.y)
    }

    pub fn xwzz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.w, self.z, self.z)
    }

    pub fn xwzw(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.w, self.z, self.w)
    }

    pub fn xwwx(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.w, self.w, self.x)
    }

    pub fn xwwy(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.w, self.w, self.y)
    }

    pub fn xwwz(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.w, self.w, self.z)
    }

    pub fn xwww(&self) -> Vec4<T> {
        return Vec4::new(self.x, self.w, self.w, self.w)
    }

    pub fn yxxx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.x, self.x)
    }

    pub fn yxxy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.x, self.y)
    }

    pub fn yxxz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.x, self.z)
    }

    pub fn yxxw(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.x, self.w)
    }

    pub fn yxyx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.y, self.x)
    }

    pub fn yxyy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.y, self.y)
    }

    pub fn yxyz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.y, self.z)
    }

    pub fn yxyw(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.y, self.w)
    }

    pub fn yxzx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.z, self.x)
    }

    pub fn yxzy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.z, self.y)
    }

    pub fn yxzz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.z, self.z)
    }

    pub fn yxzw(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.z, self.w)
    }

    pub fn yxwx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.w, self.x)
    }

    pub fn yxwy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.w, self.y)
    }

    pub fn yxwz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.w, self.z)
    }

    pub fn yxww(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.x, self.w, self.w)
    }

    pub fn yyxx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.x, self.x)
    }

    pub fn yyxy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.x, self.y)
    }

    pub fn yyxz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.x, self.z)
    }

    pub fn yyxw(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.x, self.w)
    }

    pub fn yyyx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.y, self.x)
    }

    pub fn yyyy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.y, self.y)
    }

    pub fn yyyz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.y, self.z)
    }

    pub fn yyyw(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.y, self.w)
    }

    pub fn yyzx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.z, self.x)
    }

    pub fn yyzy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.z, self.y)
    }

    pub fn yyzz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.z, self.z)
    }

    pub fn yyzw(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.z, self.w)
    }

    pub fn yywx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.w, self.x)
    }

    pub fn yywy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.w, self.y)
    }

    pub fn yywz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.w, self.z)
    }

    pub fn yyww(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.y, self.w, self.w)
    }

    pub fn yzxx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.x, self.x)
    }

    pub fn yzxy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.x, self.y)
    }

    pub fn yzxz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.x, self.z)
    }

    pub fn yzxw(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.x, self.w)
    }

    pub fn yzyx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.y, self.x)
    }

    pub fn yzyy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.y, self.y)
    }

    pub fn yzyz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.y, self.z)
    }

    pub fn yzyw(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.y, self.w)
    }

    pub fn yzzx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.z, self.x)
    }

    pub fn yzzy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.z, self.y)
    }

    pub fn yzzz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.z, self.z)
    }

    pub fn yzzw(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.z, self.w)
    }

    pub fn yzwx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.w, self.x)
    }

    pub fn yzwy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.w, self.y)
    }

    pub fn yzwz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.w, self.z)
    }

    pub fn yzww(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.z, self.w, self.w)
    }

    pub fn ywxx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.w, self.x, self.x)
    }

    pub fn ywxy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.w, self.x, self.y)
    }

    pub fn ywxz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.w, self.x, self.z)
    }

    pub fn ywxw(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.w, self.x, self.w)
    }

    pub fn ywyx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.w, self.y, self.x)
    }

    pub fn ywyy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.w, self.y, self.y)
    }

    pub fn ywyz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.w, self.y, self.z)
    }

    pub fn ywyw(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.w, self.y, self.w)
    }

    pub fn ywzx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.w, self.z, self.x)
    }

    pub fn ywzy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.w, self.z, self.y)
    }

    pub fn ywzz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.w, self.z, self.z)
    }

    pub fn ywzw(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.w, self.z, self.w)
    }

    pub fn ywwx(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.w, self.w, self.x)
    }

    pub fn ywwy(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.w, self.w, self.y)
    }

    pub fn ywwz(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.w, self.w, self.z)
    }

    pub fn ywww(&self) -> Vec4<T> {
        return Vec4::new(self.y, self.w, self.w, self.w)
    }

    pub fn zxxx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.x, self.x)
    }

    pub fn zxxy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.x, self.y)
    }

    pub fn zxxz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.x, self.z)
    }

    pub fn zxxw(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.x, self.w)
    }

    pub fn zxyx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.y, self.x)
    }

    pub fn zxyy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.y, self.y)
    }

    pub fn zxyz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.y, self.z)
    }

    pub fn zxyw(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.y, self.w)
    }

    pub fn zxzx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.z, self.x)
    }

    pub fn zxzy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.z, self.y)
    }

    pub fn zxzz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.z, self.z)
    }

    pub fn zxzw(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.z, self.w)
    }

    pub fn zxwx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.w, self.x)
    }

    pub fn zxwy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.w, self.y)
    }

    pub fn zxwz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.w, self.z)
    }

    pub fn zxww(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.x, self.w, self.w)
    }

    pub fn zyxx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.x, self.x)
    }

    pub fn zyxy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.x, self.y)
    }

    pub fn zyxz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.x, self.z)
    }

    pub fn zyxw(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.x, self.w)
    }

    pub fn zyyx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.y, self.x)
    }

    pub fn zyyy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.y, self.y)
    }

    pub fn zyyz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.y, self.z)
    }

    pub fn zyyw(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.y, self.w)
    }

    pub fn zyzx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.z, self.x)
    }

    pub fn zyzy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.z, self.y)
    }

    pub fn zyzz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.z, self.z)
    }

    pub fn zyzw(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.z, self.w)
    }

    pub fn zywx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.w, self.x)
    }

    pub fn zywy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.w, self.y)
    }

    pub fn zywz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.w, self.z)
    }

    pub fn zyww(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.y, self.w, self.w)
    }

    pub fn zzxx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.x, self.x)
    }

    pub fn zzxy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.x, self.y)
    }

    pub fn zzxz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.x, self.z)
    }

    pub fn zzxw(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.x, self.w)
    }

    pub fn zzyx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.y, self.x)
    }

    pub fn zzyy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.y, self.y)
    }

    pub fn zzyz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.y, self.z)
    }

    pub fn zzyw(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.y, self.w)
    }

    pub fn zzzx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.z, self.x)
    }

    pub fn zzzy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.z, self.y)
    }

    pub fn zzzz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.z, self.z)
    }

    pub fn zzzw(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.z, self.w)
    }

    pub fn zzwx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.w, self.x)
    }

    pub fn zzwy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.w, self.y)
    }

    pub fn zzwz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.w, self.z)
    }

    pub fn zzww(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.z, self.w, self.w)
    }

    pub fn zwxx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.w, self.x, self.x)
    }

    pub fn zwxy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.w, self.x, self.y)
    }

    pub fn zwxz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.w, self.x, self.z)
    }

    pub fn zwxw(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.w, self.x, self.w)
    }

    pub fn zwyx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.w, self.y, self.x)
    }

    pub fn zwyy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.w, self.y, self.y)
    }

    pub fn zwyz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.w, self.y, self.z)
    }

    pub fn zwyw(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.w, self.y, self.w)
    }

    pub fn zwzx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.w, self.z, self.x)
    }

    pub fn zwzy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.w, self.z, self.y)
    }

    pub fn zwzz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.w, self.z, self.z)
    }

    pub fn zwzw(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.w, self.z, self.w)
    }

    pub fn zwwx(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.w, self.w, self.x)
    }

    pub fn zwwy(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.w, self.w, self.y)
    }

    pub fn zwwz(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.w, self.w, self.z)
    }

    pub fn zwww(&self) -> Vec4<T> {
        return Vec4::new(self.z, self.w, self.w, self.w)
    }

    pub fn wxxx(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.x, self.x, self.x)
    }

    pub fn wxxy(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.x, self.x, self.y)
    }

    pub fn wxxz(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.x, self.x, self.z)
    }

    pub fn wxxw(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.x, self.x, self.w)
    }

    pub fn wxyx(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.x, self.y, self.x)
    }

    pub fn wxyy(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.x, self.y, self.y)
    }

    pub fn wxyz(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.x, self.y, self.z)
    }

    pub fn wxyw(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.x, self.y, self.w)
    }

    pub fn wxzx(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.x, self.z, self.x)
    }

    pub fn wxzy(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.x, self.z, self.y)
    }

    pub fn wxzz(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.x, self.z, self.z)
    }

    pub fn wxzw(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.x, self.z, self.w)
    }

    pub fn wxwx(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.x, self.w, self.x)
    }

    pub fn wxwy(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.x, self.w, self.y)
    }

    pub fn wxwz(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.x, self.w, self.z)
    }

    pub fn wxww(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.x, self.w, self.w)
    }

    pub fn wyxx(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.y, self.x, self.x)
    }

    pub fn wyxy(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.y, self.x, self.y)
    }

    pub fn wyxz(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.y, self.x, self.z)
    }

    pub fn wyxw(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.y, self.x, self.w)
    }

    pub fn wyyx(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.y, self.y, self.x)
    }

    pub fn wyyy(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.y, self.y, self.y)
    }

    pub fn wyyz(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.y, self.y, self.z)
    }

    pub fn wyyw(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.y, self.y, self.w)
    }

    pub fn wyzx(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.y, self.z, self.x)
    }

    pub fn wyzy(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.y, self.z, self.y)
    }

    pub fn wyzz(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.y, self.z, self.z)
    }

    pub fn wyzw(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.y, self.z, self.w)
    }

    pub fn wywx(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.y, self.w, self.x)
    }

    pub fn wywy(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.y, self.w, self.y)
    }

    pub fn wywz(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.y, self.w, self.z)
    }

    pub fn wyww(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.y, self.w, self.w)
    }

    pub fn wzxx(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.z, self.x, self.x)
    }

    pub fn wzxy(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.z, self.x, self.y)
    }

    pub fn wzxz(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.z, self.x, self.z)
    }

    pub fn wzxw(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.z, self.x, self.w)
    }

    pub fn wzyx(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.z, self.y, self.x)
    }

    pub fn wzyy(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.z, self.y, self.y)
    }

    pub fn wzyz(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.z, self.y, self.z)
    }

    pub fn wzyw(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.z, self.y, self.w)
    }

    pub fn wzzx(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.z, self.z, self.x)
    }

    pub fn wzzy(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.z, self.z, self.y)
    }

    pub fn wzzz(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.z, self.z, self.z)
    }

    pub fn wzzw(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.z, self.z, self.w)
    }

    pub fn wzwx(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.z, self.w, self.x)
    }

    pub fn wzwy(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.z, self.w, self.y)
    }

    pub fn wzwz(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.z, self.w, self.z)
    }

    pub fn wzww(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.z, self.w, self.w)
    }

    pub fn wwxx(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.w, self.x, self.x)
    }

    pub fn wwxy(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.w, self.x, self.y)
    }

    pub fn wwxz(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.w, self.x, self.z)
    }

    pub fn wwxw(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.w, self.x, self.w)
    }

    pub fn wwyx(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.w, self.y, self.x)
    }

    pub fn wwyy(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.w, self.y, self.y)
    }

    pub fn wwyz(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.w, self.y, self.z)
    }

    pub fn wwyw(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.w, self.y, self.w)
    }

    pub fn wwzx(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.w, self.z, self.x)
    }

    pub fn wwzy(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.w, self.z, self.y)
    }

    pub fn wwzz(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.w, self.z, self.z)
    }

    pub fn wwzw(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.w, self.z, self.w)
    }

    pub fn wwwx(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.w, self.w, self.x)
    }

    pub fn wwwy(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.w, self.w, self.y)
    }

    pub fn wwwz(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.w, self.w, self.z)
    }

    pub fn wwww(&self) -> Vec4<T> {
        return Vec4::new(self.w, self.w, self.w, self.w)
    }
}
