#[cfg(feature = "Oculus+Platform+LaunchResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LaunchResult {
    #[default]
    FailedGameAlreadyStarted = 3i32,
    FailedOtherReason = 6i32,
    FailedRoomFull = 2i32,
    FailedRoomNotFound = 4i32,
    FailedUserDeclined = 5i32,
    Success = 1i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+LaunchResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::LaunchResult =>
    "Oculus.Platform"."LaunchResult"
);
