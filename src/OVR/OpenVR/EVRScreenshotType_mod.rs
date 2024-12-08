#[cfg(feature = "OVR+OpenVR+EVRScreenshotType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EVRScreenshotType {
    Cubemap = 3i32,
    Mono = 1i32,
    MonoPanorama = 4i32,
    None = 0i32,
    Stereo = 2i32,
    StereoPanorama = 5i32,
}
#[cfg(feature = "OVR+OpenVR+EVRScreenshotType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRScreenshotType => "OVR.OpenVR"
    ."EVRScreenshotType"
);
