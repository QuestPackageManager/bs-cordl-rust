#[cfg(feature = "UnityEngine+FindObjectsInactive")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindObjectsInactive {
    Exclude = 0i32,
    Include = 1i32,
}
#[cfg(feature = "UnityEngine+FindObjectsInactive")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::FindObjectsInactive =>
    "UnityEngine"."FindObjectsInactive"
);
