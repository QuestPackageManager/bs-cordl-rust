#[cfg(feature = "Oculus+Platform+NetSyncVoipMicSource")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetSyncVoipMicSource {
    Internal = 2i32,
    None = 1i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+NetSyncVoipMicSource")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::NetSyncVoipMicSource =>
    "Oculus.Platform"."NetSyncVoipMicSource"
);
