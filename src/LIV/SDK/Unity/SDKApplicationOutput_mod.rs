#[cfg(feature = "LIV+SDK+Unity+SDKApplicationOutput")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SDKApplicationOutput {
    pub supportedFeatures: crate::LIV::SDK::Unity::FEATURES,
    pub engineName: *mut quest_hook::libil2cpp::Il2CppString,
    pub engineVersion: *mut quest_hook::libil2cpp::Il2CppString,
    pub applicationName: *mut quest_hook::libil2cpp::Il2CppString,
    pub applicationVersion: *mut quest_hook::libil2cpp::Il2CppString,
    pub xrDeviceName: *mut quest_hook::libil2cpp::Il2CppString,
    pub graphicsAPI: *mut quest_hook::libil2cpp::Il2CppString,
    pub sdkID: *mut quest_hook::libil2cpp::Il2CppString,
    pub sdkVersion: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "LIV+SDK+Unity+SDKApplicationOutput")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::SDKApplicationOutput =>
    "LIV.SDK.Unity"."SDKApplicationOutput"
);
#[cfg(feature = "LIV+SDK+Unity+SDKApplicationOutput")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::LIV::SDK::Unity::SDKApplicationOutput {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKApplicationOutput")]
impl crate::LIV::SDK::Unity::SDKApplicationOutput {
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
