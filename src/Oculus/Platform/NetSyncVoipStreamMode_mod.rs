#[cfg(feature = "Oculus+Platform+NetSyncVoipStreamMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetSyncVoipStreamMode {
    Ambisonic = 1i32,
    Mono = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+NetSyncVoipStreamMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::NetSyncVoipStreamMode =>
    "Oculus.Platform"."NetSyncVoipStreamMode"
);
