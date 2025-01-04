#[cfg(feature = "OVR+OpenVR+EVRScreenshotError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVRScreenshotError {
    #[default]
    BufferTooSmall = 102i32,
    IncompatibleVersion = 100i32,
    None = 0i32,
    NotFound = 101i32,
    RequestFailed = 1i32,
    ScreenshotAlreadyInProgress = 108i32,
}
#[cfg(feature = "OVR+OpenVR+EVRScreenshotError")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRScreenshotError => "OVR.OpenVR"
    ."EVRScreenshotError"
);
