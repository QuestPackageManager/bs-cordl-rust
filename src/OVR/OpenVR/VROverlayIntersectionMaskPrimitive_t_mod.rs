#[cfg(feature = "OVR+OpenVR+VROverlayIntersectionMaskPrimitive_t")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VROverlayIntersectionMaskPrimitive_t {
    pub m_nPrimitiveType: crate::OVR::OpenVR::EVROverlayIntersectionMaskPrimitiveType,
    pub m_Primitive: crate::OVR::OpenVR::VROverlayIntersectionMaskPrimitive_Data_t,
}
#[cfg(feature = "OVR+OpenVR+VROverlayIntersectionMaskPrimitive_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::VROverlayIntersectionMaskPrimitive_t => "OVR.OpenVR"
    ."VROverlayIntersectionMaskPrimitive_t"
);
#[cfg(feature = "OVR+OpenVR+VROverlayIntersectionMaskPrimitive_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::VROverlayIntersectionMaskPrimitive_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+VROverlayIntersectionMaskPrimitive_t")]
impl crate::OVR::OpenVR::VROverlayIntersectionMaskPrimitive_t {}
