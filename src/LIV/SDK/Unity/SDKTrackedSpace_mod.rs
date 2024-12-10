#[cfg(feature = "LIV+SDK+Unity+SDKTrackedSpace")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SDKTrackedSpace {
    pub trackedSpaceWorldPosition: crate::LIV::SDK::Unity::SDKVector3,
    pub trackedSpaceWorldRotation: crate::LIV::SDK::Unity::SDKQuaternion,
    pub trackedSpaceLocalScale: crate::LIV::SDK::Unity::SDKVector3,
    pub trackedSpaceLocalToWorldMatrix: crate::LIV::SDK::Unity::SDKMatrix4x4,
    pub trackedSpaceWorldToLocalMatrix: crate::LIV::SDK::Unity::SDKMatrix4x4,
}
#[cfg(feature = "LIV+SDK+Unity+SDKTrackedSpace")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::SDKTrackedSpace =>
    "LIV.SDK.Unity"."SDKTrackedSpace"
);
#[cfg(feature = "LIV+SDK+Unity+SDKTrackedSpace")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::LIV::SDK::Unity::SDKTrackedSpace {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKTrackedSpace")]
impl crate::LIV::SDK::Unity::SDKTrackedSpace {
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
}
