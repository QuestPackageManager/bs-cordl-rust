#[cfg(feature = "UnityEngine+UIElements+Align")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Align {
    #[default]
    Auto = 0i32,
    Center = 2i32,
    FlexEnd = 3i32,
    FlexStart = 1i32,
    Stretch = 4i32,
}
#[cfg(feature = "UnityEngine+UIElements+Align")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Align =>
    "UnityEngine.UIElements"."Align"
);
