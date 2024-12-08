#[cfg(feature = "Oculus+Platform+RichPresenceExtraContext")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RichPresenceExtraContext {
    CurrentCapacity = 2i32,
    EndingIn = 4i32,
    LookingForAMatch = 5i32,
    None = 1i32,
    StartedAgo = 3i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+RichPresenceExtraContext")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::RichPresenceExtraContext =>
    "Oculus.Platform"."RichPresenceExtraContext"
);
