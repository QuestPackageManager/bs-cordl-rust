#[cfg(feature = "LIV+SDK+Unity+SDKConstants")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SDKConstants {}
#[cfg(feature = "LIV+SDK+Unity+SDKConstants")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::SDKConstants => "LIV.SDK.Unity"
    ."SDKConstants"
);
#[cfg(feature = "LIV+SDK+Unity+SDKConstants")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::LIV::SDK::Unity::SDKConstants {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKConstants")]
impl crate::LIV::SDK::Unity::SDKConstants {
    pub const ENGINE_NAME: &'static str = "unity";
    pub const SDK_ID: &'static str = "7R9Y1V7WEROGDMRF6F3ESO2C6F858GCD";
    pub const SDK_VERSION: &'static str = "1.5.4";
}
