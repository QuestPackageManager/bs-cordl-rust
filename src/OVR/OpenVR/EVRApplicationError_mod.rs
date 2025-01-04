#[cfg(feature = "OVR+OpenVR+EVRApplicationError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVRApplicationError {
    #[default]
    AppKeyAlreadyExists = 100i32,
    ApplicationAlreadyRunning = 106i32,
    ApplicationAlreadyStarting = 110i32,
    BufferTooSmall = 200i32,
    IPCFailed = 105i32,
    InvalidApplication = 108i32,
    InvalidIndex = 103i32,
    InvalidManifest = 107i32,
    InvalidParameter = 203i32,
    IsTemplate = 114i32,
    LaunchFailed = 109i32,
    LaunchInProgress = 111i32,
    NoApplication = 102i32,
    NoManifest = 101i32,
    None = 0i32,
    OldApplicationQuitting = 112i32,
    PropertyNotSet = 201i32,
    SteamVRIsExiting = 115i32,
    TransitionAborted = 113i32,
    UnknownApplication = 104i32,
    UnknownProperty = 202i32,
}
#[cfg(feature = "OVR+OpenVR+EVRApplicationError")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRApplicationError => "OVR.OpenVR"
    ."EVRApplicationError"
);
