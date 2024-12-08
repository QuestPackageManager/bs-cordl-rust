#[cfg(feature = "OVR+OpenVR+EVRCompositorTimingMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EVRCompositorTimingMode {
    Explicit_ApplicationPerformsPostPresentHandoff = 2i32,
    Explicit_RuntimePerformsPostPresentHandoff = 1i32,
    Implicit = 0i32,
}
#[cfg(feature = "OVR+OpenVR+EVRCompositorTimingMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRCompositorTimingMode =>
    "OVR.OpenVR"."EVRCompositorTimingMode"
);
