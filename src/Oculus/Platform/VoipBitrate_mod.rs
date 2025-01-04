#[cfg(feature = "Oculus+Platform+VoipBitrate")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VoipBitrate {
    #[default]
    B128000 = 6i32,
    B16000 = 1i32,
    B24000 = 2i32,
    B32000 = 3i32,
    B64000 = 4i32,
    B96000 = 5i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+VoipBitrate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::VoipBitrate =>
    "Oculus.Platform"."VoipBitrate"
);
