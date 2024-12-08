#[cfg(feature = "Oculus+Platform+SystemVoipStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemVoipStatus {
    Active = 3i32,
    Suppressed = 2i32,
    Unavailable = 1i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+SystemVoipStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::SystemVoipStatus =>
    "Oculus.Platform"."SystemVoipStatus"
);
