#[cfg(feature = "Oculus+Platform+VoipSampleRate")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoipSampleRate {
    HZ24000 = 1i32,
    HZ44100 = 2i32,
    HZ48000 = 3i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+VoipSampleRate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::VoipSampleRate =>
    "Oculus.Platform"."VoipSampleRate"
);
