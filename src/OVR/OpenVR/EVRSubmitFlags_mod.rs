#[cfg(feature = "OVR+OpenVR+EVRSubmitFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVRSubmitFlags {
    #[default]
    Submit_Default = 0i32,
    Submit_GlRenderBuffer = 2i32,
    Submit_LensDistortionAlreadyApplied = 1i32,
    Submit_Reserved = 4i32,
    Submit_TextureWithDepth = 16i32,
    Submit_TextureWithPose = 8i32,
}
#[cfg(feature = "OVR+OpenVR+EVRSubmitFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRSubmitFlags => "OVR.OpenVR"
    ."EVRSubmitFlags"
);
