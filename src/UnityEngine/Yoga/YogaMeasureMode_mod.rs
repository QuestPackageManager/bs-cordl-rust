#[cfg(feature = "UnityEngine+Yoga+YogaMeasureMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum YogaMeasureMode {
    #[default]
    AtMost = 2i32,
    Exactly = 1i32,
    Undefined = 0i32,
}
#[cfg(feature = "UnityEngine+Yoga+YogaMeasureMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Yoga::YogaMeasureMode =>
    "UnityEngine.Yoga"."YogaMeasureMode"
);
