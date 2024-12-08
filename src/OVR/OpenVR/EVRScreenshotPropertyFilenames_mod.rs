#[cfg(feature = "OVR+OpenVR+EVRScreenshotPropertyFilenames")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EVRScreenshotPropertyFilenames {
    Preview = 0i32,
    VR = 1i32,
}
#[cfg(feature = "OVR+OpenVR+EVRScreenshotPropertyFilenames")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRScreenshotPropertyFilenames =>
    "OVR.OpenVR"."EVRScreenshotPropertyFilenames"
);
