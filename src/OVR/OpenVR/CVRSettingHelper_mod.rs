#[cfg(feature = "OVR+OpenVR+CVRSettingHelper")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct CVRSettingHelper {
    pub m_pSettings: crate::System::IntPtr,
}
#[cfg(feature = "OVR+OpenVR+CVRSettingHelper")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::CVRSettingHelper => "OVR.OpenVR"
    ."CVRSettingHelper"
);
#[cfg(feature = "OVR+OpenVR+CVRSettingHelper")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::CVRSettingHelper {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSettingHelper")]
impl crate::OVR::OpenVR::CVRSettingHelper {}
