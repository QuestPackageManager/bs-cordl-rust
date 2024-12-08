#[cfg(feature = "Oculus+Platform+VoipDtxState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoipDtxState {
    Disabled = 2i32,
    Enabled = 1i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+VoipDtxState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::VoipDtxState =>
    "Oculus.Platform"."VoipDtxState"
);
