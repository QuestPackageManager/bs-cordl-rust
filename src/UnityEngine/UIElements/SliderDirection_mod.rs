#[cfg(feature = "UnityEngine+UIElements+SliderDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliderDirection {
    Horizontal = 0i32,
    Vertical = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+SliderDirection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::SliderDirection =>
    "UnityEngine.UIElements"."SliderDirection"
);
