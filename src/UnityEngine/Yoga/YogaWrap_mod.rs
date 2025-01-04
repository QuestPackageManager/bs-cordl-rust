#[cfg(feature = "UnityEngine+Yoga+YogaWrap")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum YogaWrap {
    #[default]
    NoWrap = 0i32,
    Wrap = 1i32,
    WrapReverse = 2i32,
}
#[cfg(feature = "UnityEngine+Yoga+YogaWrap")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Yoga::YogaWrap =>
    "UnityEngine.Yoga"."YogaWrap"
);
