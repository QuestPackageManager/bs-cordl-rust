#[cfg(feature = "UnityEngine+Yoga+YogaPositionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum YogaPositionType {
    #[default]
    Absolute = 1i32,
    Relative = 0i32,
}
#[cfg(feature = "UnityEngine+Yoga+YogaPositionType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Yoga::YogaPositionType =>
    "UnityEngine.Yoga"."YogaPositionType"
);
