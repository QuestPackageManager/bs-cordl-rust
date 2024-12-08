#[cfg(feature = "UnityEngine+FindObjectsSortMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindObjectsSortMode {
    InstanceID = 1i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+FindObjectsSortMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::FindObjectsSortMode =>
    "UnityEngine"."FindObjectsSortMode"
);
