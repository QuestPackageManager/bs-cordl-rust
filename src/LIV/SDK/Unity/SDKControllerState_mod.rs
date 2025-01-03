#[cfg(feature = "LIV+SDK+Unity+SDKControllerState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SDKControllerState {
    pub hmdposition: crate::LIV::SDK::Unity::SDKVector3,
    pub hmdrotation: crate::LIV::SDK::Unity::SDKQuaternion,
    pub calibrationcameraposition: crate::LIV::SDK::Unity::SDKVector3,
    pub calibrationcamerarotation: crate::LIV::SDK::Unity::SDKQuaternion,
    pub cameraposition: crate::LIV::SDK::Unity::SDKVector3,
    pub camerarotation: crate::LIV::SDK::Unity::SDKQuaternion,
    pub leftposition: crate::LIV::SDK::Unity::SDKVector3,
    pub leftrotation: crate::LIV::SDK::Unity::SDKQuaternion,
    pub rightposition: crate::LIV::SDK::Unity::SDKVector3,
    pub rightrotation: crate::LIV::SDK::Unity::SDKQuaternion,
}
#[cfg(feature = "LIV+SDK+Unity+SDKControllerState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::SDKControllerState =>
    "LIV.SDK.Unity"."SDKControllerState"
);
#[cfg(feature = "LIV+SDK+Unity+SDKControllerState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::LIV::SDK::Unity::SDKControllerState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKControllerState")]
impl crate::LIV::SDK::Unity::SDKControllerState {
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
    pub fn get_empty() -> quest_hook::libil2cpp::Result<
        crate::LIV::SDK::Unity::SDKControllerState,
    > {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKControllerState = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_empty", ())?;
        Ok(__cordl_ret.into())
    }
}
