#[cfg(feature = "Oculus+Platform+UserPresenceStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UserPresenceStatus {
    #[default]
    Offline = 2i32,
    Online = 1i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+UserPresenceStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::UserPresenceStatus =>
    "Oculus.Platform"."UserPresenceStatus"
);
