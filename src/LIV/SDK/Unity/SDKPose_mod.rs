#[cfg(feature = "LIV+SDK+Unity+SDKPose")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SDKPose {
    pub projectionMatrix: crate::LIV::SDK::Unity::SDKMatrix4x4,
    pub localPosition: crate::LIV::SDK::Unity::SDKVector3,
    pub localRotation: crate::LIV::SDK::Unity::SDKQuaternion,
    pub verticalFieldOfView: f32,
    pub nearClipPlane: f32,
    pub farClipPlane: f32,
    pub unused0: i32,
    pub unused1: i32,
}
#[cfg(feature = "LIV+SDK+Unity+SDKPose")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::SDKPose => "LIV.SDK.Unity"
    ."SDKPose"
);
#[cfg(feature = "LIV+SDK+Unity+SDKPose")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::LIV::SDK::Unity::SDKPose {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKPose")]
impl crate::LIV::SDK::Unity::SDKPose {
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
        crate::LIV::SDK::Unity::SDKPose,
    > {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKPose = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_empty", ())?;
        Ok(__cordl_ret.into())
    }
}
