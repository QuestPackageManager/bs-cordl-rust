#[cfg(feature = "Oculus+Platform+PlatformInitializeResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlatformInitializeResult {
    #[default]
    FileInvalid = -3i32,
    InvalidCredentials = -8i32,
    NotEntitled = -9i32,
    PreLoaded = -2i32,
    SignatureInvalid = -4i32,
    Success = 0i32,
    UnableToVerify = -5i32,
    Uninitialized = -1i32,
    Unknown = -7i32,
    VersionMismatch = -6i32,
}
#[cfg(feature = "Oculus+Platform+PlatformInitializeResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::PlatformInitializeResult =>
    "Oculus.Platform"."PlatformInitializeResult"
);
