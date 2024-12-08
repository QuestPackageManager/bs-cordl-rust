#[cfg(feature = "Oculus+Platform+UserOrdering")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserOrdering {
    None = 1i32,
    PresenceAlphabetical = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+UserOrdering")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::UserOrdering =>
    "Oculus.Platform"."UserOrdering"
);
