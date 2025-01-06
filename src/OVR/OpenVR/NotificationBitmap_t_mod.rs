#[cfg(feature = "OVR+OpenVR+NotificationBitmap_t")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NotificationBitmap_t {
    pub m_pImageData: crate::System::IntPtr,
    pub m_nWidth: i32,
    pub m_nHeight: i32,
    pub m_nBytesPerPixel: i32,
}
#[cfg(feature = "OVR+OpenVR+NotificationBitmap_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::NotificationBitmap_t =>
    "OVR.OpenVR"."NotificationBitmap_t"
);
#[cfg(feature = "OVR+OpenVR+NotificationBitmap_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::NotificationBitmap_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+NotificationBitmap_t")]
impl crate::OVR::OpenVR::NotificationBitmap_t {}
