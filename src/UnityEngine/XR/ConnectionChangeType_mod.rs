#[cfg(feature = "UnityEngine+XR+ConnectionChangeType")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConnectionChangeType {
    #[default]
    ConfigChange = 33619970u32,
    Connected = 131328u32,
    Disconnected = 16777729u32,
}
#[cfg(feature = "UnityEngine+XR+ConnectionChangeType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::ConnectionChangeType =>
    "UnityEngine.XR"."ConnectionChangeType"
);
