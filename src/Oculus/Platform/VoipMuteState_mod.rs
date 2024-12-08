#[cfg(feature = "Oculus+Platform+VoipMuteState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoipMuteState {
    Muted = 1i32,
    Unknown = 0i32,
    Unmuted = 2i32,
}
#[cfg(feature = "Oculus+Platform+VoipMuteState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::VoipMuteState =>
    "Oculus.Platform"."VoipMuteState"
);
