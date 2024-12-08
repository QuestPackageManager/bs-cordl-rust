#[cfg(feature = "UnityEngine+Yoga+YogaDisplay")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YogaDisplay {
    Flex = 0i32,
    None = 1i32,
}
#[cfg(feature = "UnityEngine+Yoga+YogaDisplay")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Yoga::YogaDisplay =>
    "UnityEngine.Yoga"."YogaDisplay"
);
