#[cfg(feature = "LIV+SDK+Unity+SDKMatrix4x4")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SDKMatrix4x4 {
    pub m00: f32,
    pub m01: f32,
    pub m02: f32,
    pub m03: f32,
    pub m10: f32,
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,
    pub m20: f32,
    pub m21: f32,
    pub m22: f32,
    pub m23: f32,
    pub m30: f32,
    pub m31: f32,
    pub m32: f32,
    pub m33: f32,
}
#[cfg(feature = "LIV+SDK+Unity+SDKMatrix4x4")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::SDKMatrix4x4 => "LIV.SDK.Unity"
    ."SDKMatrix4x4"
);
#[cfg(feature = "LIV+SDK+Unity+SDKMatrix4x4")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::LIV::SDK::Unity::SDKMatrix4x4 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKMatrix4x4")]
impl crate::LIV::SDK::Unity::SDKMatrix4x4 {
    pub fn Perspective(
        vFov: f32,
        aspect: f32,
        zNear: f32,
        zFar: f32,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::SDKMatrix4x4> {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKMatrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Perspective", (vFov, aspect, zNear, zFar))?;
        Ok(__cordl_ret.into())
    }
    pub fn Rotate(
        value: crate::LIV::SDK::Unity::SDKQuaternion,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::SDKMatrix4x4> {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKMatrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Rotate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Scale(
        value: crate::LIV::SDK::Unity::SDKVector3,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::SDKMatrix4x4> {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKMatrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Scale", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TRS(
        translation: crate::LIV::SDK::Unity::SDKVector3,
        rotation: crate::LIV::SDK::Unity::SDKQuaternion,
        scale: crate::LIV::SDK::Unity::SDKVector3,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::SDKMatrix4x4> {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKMatrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TRS", (translation, rotation, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Translate(
        value: crate::LIV::SDK::Unity::SDKVector3,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::SDKMatrix4x4> {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKMatrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Translate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_identity() -> quest_hook::libil2cpp::Result<
        crate::LIV::SDK::Unity::SDKMatrix4x4,
    > {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKMatrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_identity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Matrix4x4_1(
        v: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::SDKMatrix4x4> {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKMatrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_SDKMatrix4x4_0(
        v: crate::LIV::SDK::Unity::SDKMatrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_SDKMatrix4x4_0(
        lhs: crate::LIV::SDK::Unity::SDKMatrix4x4,
        rhs: crate::LIV::SDK::Unity::SDKMatrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::SDKMatrix4x4> {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKMatrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_SDKVector3_1(
        lhs: crate::LIV::SDK::Unity::SDKMatrix4x4,
        rhs: crate::LIV::SDK::Unity::SDKVector3,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::SDKVector3> {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKVector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
}
