#[cfg(feature = "Unity+Mathematics+float3x3")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct float3x3 {
    pub c0: crate::Unity::Mathematics::float3,
    pub c1: crate::Unity::Mathematics::float3,
    pub c2: crate::Unity::Mathematics::float3,
}
#[cfg(feature = "Unity+Mathematics+float3x3")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::float3x3 =>
    "Unity.Mathematics"."float3x3"
);
#[cfg(feature = "Unity+Mathematics+float3x3")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Mathematics::float3x3 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+float3x3")]
impl crate::Unity::Mathematics::float3x3 {
    pub fn AxisAngle(
        axis: crate::Unity::Mathematics::float3,
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AxisAngle", (axis, angle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (o),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_float3x3_0(
        &mut self,
        rhs: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (rhs),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerXYZ_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerXYZ", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerXYZ_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerXYZ", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerXZY_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerXZY", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerXZY_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerXZY", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerYXZ_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerYXZ", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerYXZ_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerYXZ", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerYZX_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerYZX", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerYZX_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerYZX", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerZXY_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerZXY", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerZXY_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerZXY", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerZYX_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerZYX", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerZYX_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerZYX", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn Euler_f32_f32_f32_math_RotationOrder1(
        x: f32,
        y: f32,
        z: f32,
        order: crate::Unity::Mathematics::math_RotationOrder,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Euler", (x, y, z, order))?;
        Ok(__cordl_ret.into())
    }
    pub fn Euler_float3_math_RotationOrder0(
        xyz: crate::Unity::Mathematics::float3,
        order: crate::Unity::Mathematics::math_RotationOrder,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Euler", (xyz, order))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn LookRotation(
        forward: crate::Unity::Mathematics::float3,
        up: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LookRotation", (forward, up))?;
        Ok(__cordl_ret.into())
    }
    pub fn LookRotationSafe(
        forward: crate::Unity::Mathematics::float3,
        up: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LookRotationSafe", (forward, up))?;
        Ok(__cordl_ret.into())
    }
    pub fn RotateX(
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RotateX", (angle))?;
        Ok(__cordl_ret.into())
    }
    pub fn RotateY(
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RotateY", (angle))?;
        Ok(__cordl_ret.into())
    }
    pub fn RotateZ(
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RotateZ", (angle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Scale_f32_0(
        s: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Scale", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn Scale_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Scale", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn Scale_float3_2(
        v: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Scale", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString_IFormatProvider1(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (format, formatProvider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool3(
        &mut self,
        v: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_bool3x3_4(
        &mut self,
        v: crate::Unity::Mathematics::bool3x3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_double3x3_10(
        &mut self,
        v: crate::Unity::Mathematics::double3x3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_2(
        &mut self,
        v: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_f32_f32_f32_f32_f32_f32_f32_f32_1(
        &mut self,
        m00: f32,
        m01: f32,
        m02: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m20: f32,
        m21: f32,
        m22: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (m00, m01, m02, m10, m11, m12, m20, m21, m22),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f64_9(
        &mut self,
        v: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float3_float3_float3_0(
        &mut self,
        c0: crate::Unity::Mathematics::float3,
        c1: crate::Unity::Mathematics::float3,
        c2: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (c0, c1, c2),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float4x4_11(
        &mut self,
        f4x4: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (f4x4),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_5(
        &mut self,
        v: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_int3x3_6(
        &mut self,
        v: crate::Unity::Mathematics::int3x3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_quaternion12(
        &mut self,
        q: crate::Unity::Mathematics::quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (q),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_7(
        &mut self,
        v: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_uint3x3_8(
        &mut self,
        v: crate::Unity::Mathematics::uint3x3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float3>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Mathematics::float3,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Item", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_f32_float3x3_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_float3x3_f32_1(
        lhs: crate::Unity::Mathematics::float3x3,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_float3x3_float3x3_0(
        lhs: crate::Unity::Mathematics::float3x3,
        rhs: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Decrement(
        val: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Decrement", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_f32_float3x3_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_float3x3_f32_1(
        lhs: crate::Unity::Mathematics::float3x3,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_float3x3_float3x3_0(
        lhs: crate::Unity::Mathematics::float3x3,
        rhs: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_f32_float3x3_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_float3x3_f32_1(
        lhs: crate::Unity::Mathematics::float3x3,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_float3x3_float3x3_0(
        lhs: crate::Unity::Mathematics::float3x3,
        rhs: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit__cordl_bool0(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_bool3x3_1(
        v: crate::Unity::Mathematics::bool3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_double3x3_3(
        v: crate::Unity::Mathematics::double3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_f64_2(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_float4x4_4(
        f4x4: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (f4x4))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_f32_float3x3_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_float3x3_f32_1(
        lhs: crate::Unity::Mathematics::float3x3,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_float3x3_float3x3_0(
        lhs: crate::Unity::Mathematics::float3x3,
        rhs: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_f32_float3x3_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_float3x3_f32_1(
        lhs: crate::Unity::Mathematics::float3x3,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_float3x3_float3x3_0(
        lhs: crate::Unity::Mathematics::float3x3,
        rhs: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_f32_0(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_i32_1(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_int3x3_2(
        v: crate::Unity::Mathematics::int3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_u32_3(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_uint3x3_4(
        v: crate::Unity::Mathematics::uint3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Increment(
        val: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Increment", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_f32_float3x3_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_float3x3_f32_1(
        lhs: crate::Unity::Mathematics::float3x3,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_float3x3_float3x3_0(
        lhs: crate::Unity::Mathematics::float3x3,
        rhs: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_f32_float3x3_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_float3x3_f32_1(
        lhs: crate::Unity::Mathematics::float3x3,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_float3x3_float3x3_0(
        lhs: crate::Unity::Mathematics::float3x3,
        rhs: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_f32_float3x3_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_float3x3_f32_1(
        lhs: crate::Unity::Mathematics::float3x3,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_float3x3_float3x3_0(
        lhs: crate::Unity::Mathematics::float3x3,
        rhs: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_f32_float3x3_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_float3x3_f32_1(
        lhs: crate::Unity::Mathematics::float3x3,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_float3x3_float3x3_0(
        lhs: crate::Unity::Mathematics::float3x3,
        rhs: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_f32_float3x3_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_float3x3_f32_1(
        lhs: crate::Unity::Mathematics::float3x3,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_float3x3_float3x3_0(
        lhs: crate::Unity::Mathematics::float3x3,
        rhs: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_f32_float3x3_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_float3x3_f32_1(
        lhs: crate::Unity::Mathematics::float3x3,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_float3x3_float3x3_0(
        lhs: crate::Unity::Mathematics::float3x3,
        rhs: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryNegation(
        val: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_UnaryNegation", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryPlus(
        val: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_UnaryPlus", (val))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+float3x3")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Mathematics::float3x3>>
for crate::Unity::Mathematics::float3x3 {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Mathematics::float3x3> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+float3x3")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Mathematics::float3x3>>
for crate::Unity::Mathematics::float3x3 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Mathematics::float3x3> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+float3x3")]
impl AsRef<crate::System::IFormattable> for crate::Unity::Mathematics::float3x3 {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+float3x3")]
impl AsMut<crate::System::IFormattable> for crate::Unity::Mathematics::float3x3 {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
