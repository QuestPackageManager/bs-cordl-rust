#[cfg(feature = "LIV+SDK+Unity+SDKQuaternion")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SDKQuaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[cfg(feature = "LIV+SDK+Unity+SDKQuaternion")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::SDKQuaternion =>
    "LIV.SDK.Unity"."SDKQuaternion"
);
#[cfg(feature = "LIV+SDK+Unity+SDKQuaternion")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::LIV::SDK::Unity::SDKQuaternion {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKQuaternion")]
impl crate::LIV::SDK::Unity::SDKQuaternion {
    pub fn Euler(
        pitch: f32,
        yaw: f32,
        roll: f32,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::SDKQuaternion> {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKQuaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Euler", (pitch, yaw, roll))?;
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
    pub fn get_identity() -> quest_hook::libil2cpp::Result<
        crate::LIV::SDK::Unity::SDKQuaternion,
    > {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKQuaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_identity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Quaternion1(
        v: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::SDKQuaternion> {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKQuaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_SDKQuaternion0(
        v: crate::LIV::SDK::Unity::SDKQuaternion,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_SDKQuaternion0(
        lhs: crate::LIV::SDK::Unity::SDKQuaternion,
        rhs: crate::LIV::SDK::Unity::SDKQuaternion,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::SDKQuaternion> {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKQuaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_SDKVector3_1(
        lhs: crate::LIV::SDK::Unity::SDKQuaternion,
        rhs: crate::LIV::SDK::Unity::SDKVector3,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::SDKVector3> {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKVector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
}
