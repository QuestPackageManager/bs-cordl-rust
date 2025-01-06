#[cfg(feature = "Unity+Mathematics+float4x4")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct float4x4 {
    pub c0: crate::Unity::Mathematics::float4,
    pub c1: crate::Unity::Mathematics::float4,
    pub c2: crate::Unity::Mathematics::float4,
    pub c3: crate::Unity::Mathematics::float4,
}
#[cfg(feature = "Unity+Mathematics+float4x4")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::float4x4 =>
    "Unity.Mathematics"."float4x4"
);
#[cfg(feature = "Unity+Mathematics+float4x4")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Mathematics::float4x4 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+float4x4")]
impl crate::Unity::Mathematics::float4x4 {
    pub fn AxisAngle(
        axis: crate::Unity::Mathematics::float3,
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
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
    pub fn Equals_float4x4_0(
        &mut self,
        rhs: crate::Unity::Mathematics::float4x4,
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
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerXYZ", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerXYZ_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerXYZ", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerXZY_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerXZY", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerXZY_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerXZY", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerYXZ_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerYXZ", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerYXZ_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerYXZ", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerYZX_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerYZX", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerYZX_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerYZX", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerZXY_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerZXY", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerZXY_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerZXY", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerZYX_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerZYX", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerZYX_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerZYX", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn Euler_f32_f32_f32_math_RotationOrder1(
        x: f32,
        y: f32,
        z: f32,
        order: crate::Unity::Mathematics::math_RotationOrder,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Euler", (x, y, z, order))?;
        Ok(__cordl_ret.into())
    }
    pub fn Euler_float3_math_RotationOrder0(
        xyz: crate::Unity::Mathematics::float3,
        order: crate::Unity::Mathematics::math_RotationOrder,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
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
    pub fn LookAt(
        eye: crate::Unity::Mathematics::float3,
        target: crate::Unity::Mathematics::float3,
        up: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LookAt", (eye, target, up))?;
        Ok(__cordl_ret.into())
    }
    pub fn Ortho(
        width: f32,
        height: f32,
        near: f32,
        far: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Ortho", (width, height, near, far))?;
        Ok(__cordl_ret.into())
    }
    pub fn OrthoOffCenter(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OrthoOffCenter", (left, right, bottom, top, near, far))?;
        Ok(__cordl_ret.into())
    }
    pub fn PerspectiveFov(
        verticalFov: f32,
        aspect: f32,
        near: f32,
        far: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PerspectiveFov", (verticalFov, aspect, near, far))?;
        Ok(__cordl_ret.into())
    }
    pub fn PerspectiveOffCenter(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PerspectiveOffCenter", (left, right, bottom, top, near, far))?;
        Ok(__cordl_ret.into())
    }
    pub fn RotateX(
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RotateX", (angle))?;
        Ok(__cordl_ret.into())
    }
    pub fn RotateY(
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RotateY", (angle))?;
        Ok(__cordl_ret.into())
    }
    pub fn RotateZ(
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RotateZ", (angle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Scale_f32_0(
        s: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Scale", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn Scale_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Scale", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn Scale_float3_2(
        scales: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Scale", (scales))?;
        Ok(__cordl_ret.into())
    }
    pub fn TRS(
        translation: crate::Unity::Mathematics::float3,
        rotation: crate::Unity::Mathematics::quaternion,
        scale: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TRS", (translation, rotation, scale))?;
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
    pub fn Translate(
        vector: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Translate", (vector))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_RigidTransform13(
        &mut self,
        transform: crate::Unity::Mathematics::RigidTransform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (transform),
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
    pub fn _ctor_bool4x4_4(
        &mut self,
        v: crate::Unity::Mathematics::bool4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_double4x4_10(
        &mut self,
        v: crate::Unity::Mathematics::double4x4,
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
    pub fn _ctor_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_1(
        &mut self,
        m00: f32,
        m01: f32,
        m02: f32,
        m03: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m13: f32,
        m20: f32,
        m21: f32,
        m22: f32,
        m23: f32,
        m30: f32,
        m31: f32,
        m32: f32,
        m33: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (
                m00,
                m01,
                m02,
                m03,
                m10,
                m11,
                m12,
                m13,
                m20,
                m21,
                m22,
                m23,
                m30,
                m31,
                m32,
                m33,
            ),
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
    pub fn _ctor_float3x3_float3_11(
        &mut self,
        rotation: crate::Unity::Mathematics::float3x3,
        translation: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (rotation, translation),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float4_float4_float4_float4_0(
        &mut self,
        c0: crate::Unity::Mathematics::float4,
        c1: crate::Unity::Mathematics::float4,
        c2: crate::Unity::Mathematics::float4,
        c3: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (c0, c1, c2, c3),
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
    pub fn _ctor_int4x4_6(
        &mut self,
        v: crate::Unity::Mathematics::int4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_quaternion_float3_12(
        &mut self,
        rotation: crate::Unity::Mathematics::quaternion,
        translation: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (rotation, translation),
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
    pub fn _ctor_uint4x4_8(
        &mut self,
        v: crate::Unity::Mathematics::uint4x4,
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
        quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float4>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Mathematics::float4,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Item", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_f32_float4x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_float4x4_f32_1(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_float4x4_float4x4_0(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Decrement(
        val: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Decrement", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_f32_float4x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_float4x4_f32_1(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_float4x4_float4x4_0(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_f32_float4x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_float4x4_f32_1(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_float4x4_float4x4_0(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit__cordl_bool0(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_bool4x4_1(
        v: crate::Unity::Mathematics::bool4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_double4x4_3(
        v: crate::Unity::Mathematics::double4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_f64_2(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_f32_float4x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_float4x4_f32_1(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_float4x4_float4x4_0(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_f32_float4x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_float4x4_f32_1(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_float4x4_float4x4_0(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Matrix4x4_5(
        m: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_f32_0(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_float4x4_6(
        m: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_i32_1(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_int4x4_2(
        v: crate::Unity::Mathematics::int4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_u32_3(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_uint4x4_4(
        v: crate::Unity::Mathematics::uint4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Increment(
        val: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Increment", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_f32_float4x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_float4x4_f32_1(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_float4x4_float4x4_0(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_f32_float4x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_float4x4_f32_1(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_float4x4_float4x4_0(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_f32_float4x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_float4x4_f32_1(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_float4x4_float4x4_0(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_f32_float4x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_float4x4_f32_1(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_float4x4_float4x4_0(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_f32_float4x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_float4x4_f32_1(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_float4x4_float4x4_0(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_f32_float4x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_float4x4_f32_1(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_float4x4_float4x4_0(
        lhs: crate::Unity::Mathematics::float4x4,
        rhs: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryNegation(
        val: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_UnaryNegation", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryPlus(
        val: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_UnaryPlus", (val))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+float4x4")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Mathematics::float4x4>>
for crate::Unity::Mathematics::float4x4 {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Mathematics::float4x4> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+float4x4")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Mathematics::float4x4>>
for crate::Unity::Mathematics::float4x4 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Mathematics::float4x4> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+float4x4")]
impl AsRef<crate::System::IFormattable> for crate::Unity::Mathematics::float4x4 {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+float4x4")]
impl AsMut<crate::System::IFormattable> for crate::Unity::Mathematics::float4x4 {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
