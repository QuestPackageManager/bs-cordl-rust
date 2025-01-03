#[cfg(feature = "OVR+OpenVR+Compositor_OverlaySettings")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Compositor_OverlaySettings {
    pub _cordl_size: u32,
    pub curved: bool,
    pub antialias: bool,
    pub scale: f32,
    pub distance: f32,
    pub alpha: f32,
    pub uOffset: f32,
    pub vOffset: f32,
    pub uScale: f32,
    pub vScale: f32,
    pub gridDivs: f32,
    pub gridWidth: f32,
    pub gridScale: f32,
    pub transform: crate::OVR::OpenVR::HmdMatrix44_t,
}
#[cfg(feature = "OVR+OpenVR+Compositor_OverlaySettings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::Compositor_OverlaySettings =>
    "OVR.OpenVR"."Compositor_OverlaySettings"
);
#[cfg(feature = "OVR+OpenVR+Compositor_OverlaySettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::Compositor_OverlaySettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+Compositor_OverlaySettings")]
impl crate::OVR::OpenVR::Compositor_OverlaySettings {}
