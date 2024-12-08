#[cfg(feature = "UnityEngine+Yoga+YogaUnit")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YogaUnit {
    Auto = 3i32,
    Percent = 2i32,
    Point = 1i32,
    Undefined = 0i32,
}
#[cfg(feature = "UnityEngine+Yoga+YogaUnit")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Yoga::YogaUnit =>
    "UnityEngine.Yoga"."YogaUnit"
);
