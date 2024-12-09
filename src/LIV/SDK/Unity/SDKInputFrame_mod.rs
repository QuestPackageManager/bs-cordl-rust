#[cfg(feature = "LIV+SDK+Unity+SDKInputFrame")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SDKInputFrame {
    pub pose: crate::LIV::SDK::Unity::SDKPose,
    pub clipPlane: crate::LIV::SDK::Unity::SDKClipPlane,
    pub stageTransform: crate::LIV::SDK::Unity::SDKTransform,
    pub features: crate::LIV::SDK::Unity::FEATURES,
    pub groundClipPlane: crate::LIV::SDK::Unity::SDKClipPlane,
    pub frameid: u64,
    pub referenceframe: u64,
    pub priority: crate::LIV::SDK::Unity::SDKPriority,
}
#[cfg(feature = "LIV+SDK+Unity+SDKInputFrame")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::SDKInputFrame =>
    "LIV.SDK.Unity"."SDKInputFrame"
);
#[cfg(feature = "LIV+SDK+Unity+SDKInputFrame")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::LIV::SDK::Unity::SDKInputFrame {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKInputFrame")]
impl crate::LIV::SDK::Unity::SDKInputFrame {
    pub fn ObtainControl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ObtainControl",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ReleaseControl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ReleaseControl",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
