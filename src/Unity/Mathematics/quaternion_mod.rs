#[cfg(feature = "Unity+Mathematics+quaternion")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct quaternion {
    pub value: crate::Unity::Mathematics::float4,
}
#[cfg(feature = "Unity+Mathematics+quaternion")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::quaternion =>
    "Unity.Mathematics"."quaternion"
);
#[cfg(feature = "Unity+Mathematics+quaternion")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Mathematics::quaternion {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+quaternion")]
impl crate::Unity::Mathematics::quaternion {
    pub fn AxisAngle(
        axis: crate::Unity::Mathematics::float3,
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AxisAngle", (axis, angle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Gc1(
        &mut self,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (x),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_quaternion0(
        &mut self,
        x: crate::Unity::Mathematics::quaternion,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (x),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerXYZ_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerXYZ", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerXYZ_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerXYZ", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerXZY_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerXZY", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerXZY_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerXZY", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerYXZ_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerYXZ", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerYXZ_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerYXZ", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerYZX_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerYZX", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerYZX_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerYZX", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerZXY_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerZXY", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerZXY_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerZXY", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerZYX_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerZYX", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerZYX_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerZYX", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn Euler_f32_f32_f32_math_RotationOrder1(
        x: f32,
        y: f32,
        z: f32,
        order: crate::Unity::Mathematics::math_RotationOrder,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Euler", (x, y, z, order))?;
        Ok(__cordl_ret.into())
    }
    pub fn Euler_float3_math_RotationOrder0(
        xyz: crate::Unity::Mathematics::float3,
        order: crate::Unity::Mathematics::math_RotationOrder,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
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
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LookRotation", (forward, up))?;
        Ok(__cordl_ret.into())
    }
    pub fn LookRotationSafe(
        forward: crate::Unity::Mathematics::float3,
        up: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LookRotationSafe", (forward, up))?;
        Ok(__cordl_ret.into())
    }
    pub fn RotateX(
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RotateX", (angle))?;
        Ok(__cordl_ret.into())
    }
    pub fn RotateY(
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RotateY", (angle))?;
        Ok(__cordl_ret.into())
    }
    pub fn RotateZ(
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RotateZ", (angle))?;
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
    pub fn ToString_Gc_Gc1(
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
    pub fn _ctor_f32_f32_f32_f32_0(
        &mut self,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (x, y, z, w),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float3x3_2(
        &mut self,
        m: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (m),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float4_1(
        &mut self,
        value: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float4x4_3(
        &mut self,
        m: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (m),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Quaternion1(
        q: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_float4_2(
        v: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_quaternion0(
        q: crate::Unity::Mathematics::quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (q))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+quaternion")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IFormattable>>
for crate::Unity::Mathematics::quaternion {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IFormattable> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+quaternion")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IFormattable>>
for crate::Unity::Mathematics::quaternion {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IFormattable> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+quaternion")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::Unity::Mathematics::quaternion>>
for crate::Unity::Mathematics::quaternion {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::Unity::Mathematics::quaternion> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+quaternion")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::Unity::Mathematics::quaternion>>
for crate::Unity::Mathematics::quaternion {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::Unity::Mathematics::quaternion> {
        todo!()
    }
}
