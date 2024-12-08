#[cfg(feature = "Oculus+Platform+PartyUpdateAction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartyUpdateAction {
    Invite = 3i32,
    Join = 1i32,
    Leave = 2i32,
    Uninvite = 4i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+PartyUpdateAction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::PartyUpdateAction =>
    "Oculus.Platform"."PartyUpdateAction"
);
