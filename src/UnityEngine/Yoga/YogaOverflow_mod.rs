#[cfg(feature = "UnityEngine+Yoga+YogaOverflow")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YogaOverflow {
    Hidden = 1i32,
    Scroll = 2i32,
    Visible = 0i32,
}
#[cfg(feature = "UnityEngine+Yoga+YogaOverflow")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Yoga::YogaOverflow =>
    "UnityEngine.Yoga"."YogaOverflow"
);
