#[cfg(feature = "UnityEngine+UIElements+PickingMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PickingMode {
    Ignore = 1i32,
    Position = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+PickingMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::PickingMode =>
    "UnityEngine.UIElements"."PickingMode"
);
