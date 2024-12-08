#[cfg(feature = "UnityEngine+Yoga+YogaAlign")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YogaAlign {
    Auto = 0i32,
    Baseline = 5i32,
    Center = 2i32,
    FlexEnd = 3i32,
    FlexStart = 1i32,
    SpaceAround = 7i32,
    SpaceBetween = 6i32,
    Stretch = 4i32,
}
#[cfg(feature = "UnityEngine+Yoga+YogaAlign")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Yoga::YogaAlign =>
    "UnityEngine.Yoga"."YogaAlign"
);
