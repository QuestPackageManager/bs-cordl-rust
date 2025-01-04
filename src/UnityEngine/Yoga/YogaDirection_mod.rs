#[cfg(feature = "UnityEngine+Yoga+YogaDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum YogaDirection {
    #[default]
    Inherit = 0i32,
    LTR = 1i32,
    RTL = 2i32,
}
#[cfg(feature = "UnityEngine+Yoga+YogaDirection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Yoga::YogaDirection =>
    "UnityEngine.Yoga"."YogaDirection"
);
