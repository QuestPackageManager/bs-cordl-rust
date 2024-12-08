#[cfg(feature = "UnityEngine+UIElements+DeltaSpeed")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeltaSpeed {
    Fast = 0i32,
    Normal = 1i32,
    Slow = 2i32,
}
#[cfg(feature = "UnityEngine+UIElements+DeltaSpeed")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::DeltaSpeed =>
    "UnityEngine.UIElements"."DeltaSpeed"
);
