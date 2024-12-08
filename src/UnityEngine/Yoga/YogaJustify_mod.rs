#[cfg(feature = "UnityEngine+Yoga+YogaJustify")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YogaJustify {
    Center = 1i32,
    FlexEnd = 2i32,
    FlexStart = 0i32,
    SpaceAround = 4i32,
    SpaceBetween = 3i32,
}
#[cfg(feature = "UnityEngine+Yoga+YogaJustify")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Yoga::YogaJustify =>
    "UnityEngine.Yoga"."YogaJustify"
);
