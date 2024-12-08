#[cfg(feature = "Oculus+Platform+LaunchType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchType {
    Coordinated = 3i32,
    Deeplink = 4i32,
    Invite = 2i32,
    Normal = 1i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+LaunchType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::LaunchType =>
    "Oculus.Platform"."LaunchType"
);
