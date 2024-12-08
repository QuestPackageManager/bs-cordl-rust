#[cfg(feature = "Oculus+Platform+PartyMicState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartyMicState {
    App = 2i32,
    Inactive = 4i32,
    InputShared = 5i32,
    Mute = 3i32,
    Party = 1i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+PartyMicState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::PartyMicState =>
    "Oculus.Platform"."PartyMicState"
);
