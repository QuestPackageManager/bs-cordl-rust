#[cfg(feature = "OVR+OpenVR+EVRCompositorError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVRCompositorError {
    #[default]
    AlreadySubmitted = 108i32,
    DoNotHaveFocus = 101i32,
    IncompatibleVersion = 100i32,
    IndexOutOfRange = 107i32,
    InvalidBounds = 109i32,
    InvalidTexture = 102i32,
    IsNotSceneApplication = 103i32,
    None = 0i32,
    RequestFailed = 1i32,
    SharedTexturesNotSupported = 106i32,
    TextureIsOnWrongDevice = 104i32,
    TextureUsesUnsupportedFormat = 105i32,
}
#[cfg(feature = "OVR+OpenVR+EVRCompositorError")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRCompositorError => "OVR.OpenVR"
    ."EVRCompositorError"
);
