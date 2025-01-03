#[cfg(feature = "OVR+OpenVR+RenderModel_ComponentState_t")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct RenderModel_ComponentState_t {
    pub mTrackingToComponentRenderModel: crate::OVR::OpenVR::HmdMatrix34_t,
    pub mTrackingToComponentLocal: crate::OVR::OpenVR::HmdMatrix34_t,
    pub uProperties: u32,
}
#[cfg(feature = "OVR+OpenVR+RenderModel_ComponentState_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::RenderModel_ComponentState_t =>
    "OVR.OpenVR"."RenderModel_ComponentState_t"
);
#[cfg(feature = "OVR+OpenVR+RenderModel_ComponentState_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::RenderModel_ComponentState_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+RenderModel_ComponentState_t")]
impl crate::OVR::OpenVR::RenderModel_ComponentState_t {}
