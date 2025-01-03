#[cfg(feature = "Unity+Mathematics+RigidTransform")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct RigidTransform {
    pub rot: crate::Unity::Mathematics::quaternion,
    pub pos: crate::Unity::Mathematics::float3,
}
#[cfg(feature = "Unity+Mathematics+RigidTransform")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::RigidTransform =>
    "Unity.Mathematics"."RigidTransform"
);
#[cfg(feature = "Unity+Mathematics+RigidTransform")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Mathematics::RigidTransform {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+RigidTransform")]
impl crate::Unity::Mathematics::RigidTransform {
    pub fn AxisAngle(
        axis: crate::Unity::Mathematics::float3,
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::RigidTransform> {
        let __cordl_ret: crate::Unity::Mathematics::RigidTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AxisAngle", (axis, angle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
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
    pub fn Equals_RigidTransform0(
        &mut self,
        x: crate::Unity::Mathematics::RigidTransform,
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
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::RigidTransform> {
        let __cordl_ret: crate::Unity::Mathematics::RigidTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerXYZ", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerXYZ_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::RigidTransform> {
        let __cordl_ret: crate::Unity::Mathematics::RigidTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerXYZ", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerXZY_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::RigidTransform> {
        let __cordl_ret: crate::Unity::Mathematics::RigidTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerXZY", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerXZY_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::RigidTransform> {
        let __cordl_ret: crate::Unity::Mathematics::RigidTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerXZY", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerYXZ_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::RigidTransform> {
        let __cordl_ret: crate::Unity::Mathematics::RigidTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerYXZ", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerYXZ_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::RigidTransform> {
        let __cordl_ret: crate::Unity::Mathematics::RigidTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerYXZ", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerYZX_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::RigidTransform> {
        let __cordl_ret: crate::Unity::Mathematics::RigidTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerYZX", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerYZX_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::RigidTransform> {
        let __cordl_ret: crate::Unity::Mathematics::RigidTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerYZX", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerZXY_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::RigidTransform> {
        let __cordl_ret: crate::Unity::Mathematics::RigidTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerZXY", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerZXY_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::RigidTransform> {
        let __cordl_ret: crate::Unity::Mathematics::RigidTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerZXY", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerZYX_f32_f32_f32_1(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::RigidTransform> {
        let __cordl_ret: crate::Unity::Mathematics::RigidTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerZYX", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn EulerZYX_float3_0(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::RigidTransform> {
        let __cordl_ret: crate::Unity::Mathematics::RigidTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EulerZYX", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn Euler_f32_f32_f32_math_RotationOrder1(
        x: f32,
        y: f32,
        z: f32,
        order: crate::Unity::Mathematics::math_RotationOrder,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::RigidTransform> {
        let __cordl_ret: crate::Unity::Mathematics::RigidTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Euler", (x, y, z, order))?;
        Ok(__cordl_ret.into())
    }
    pub fn Euler_float3_math_RotationOrder0(
        xyz: crate::Unity::Mathematics::float3,
        order: crate::Unity::Mathematics::math_RotationOrder,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::RigidTransform> {
        let __cordl_ret: crate::Unity::Mathematics::RigidTransform = <Self as quest_hook::libil2cpp::Type>::class()
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
    pub fn RotateX(
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::RigidTransform> {
        let __cordl_ret: crate::Unity::Mathematics::RigidTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RotateX", (angle))?;
        Ok(__cordl_ret.into())
    }
    pub fn RotateY(
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::RigidTransform> {
        let __cordl_ret: crate::Unity::Mathematics::RigidTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RotateY", (angle))?;
        Ok(__cordl_ret.into())
    }
    pub fn RotateZ(
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::RigidTransform> {
        let __cordl_ret: crate::Unity::Mathematics::RigidTransform = <Self as quest_hook::libil2cpp::Type>::class()
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
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::RigidTransform> {
        let __cordl_ret: crate::Unity::Mathematics::RigidTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Translate", (vector))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float3x3_float3_1(
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
    pub fn _ctor_float4x4_2(
        &mut self,
        transform: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (transform),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_quaternion_float3_0(
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
}
