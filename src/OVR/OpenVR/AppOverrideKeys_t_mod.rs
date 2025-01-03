#[cfg(feature = "OVR+OpenVR+AppOverrideKeys_t")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct AppOverrideKeys_t {
    pub pchKey: crate::System::IntPtr,
    pub pchValue: crate::System::IntPtr,
}
#[cfg(feature = "OVR+OpenVR+AppOverrideKeys_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::AppOverrideKeys_t => "OVR.OpenVR"
    ."AppOverrideKeys_t"
);
#[cfg(feature = "OVR+OpenVR+AppOverrideKeys_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::AppOverrideKeys_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+AppOverrideKeys_t")]
impl crate::OVR::OpenVR::AppOverrideKeys_t {}
