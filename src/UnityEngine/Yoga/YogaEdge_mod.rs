#[cfg(feature = "UnityEngine+Yoga+YogaEdge")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum YogaEdge {
    #[default]
    All = 8i32,
    Bottom = 3i32,
    End = 5i32,
    Horizontal = 6i32,
    Left = 0i32,
    Right = 2i32,
    Start = 4i32,
    Top = 1i32,
    Vertical = 7i32,
}
#[cfg(feature = "UnityEngine+Yoga+YogaEdge")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Yoga::YogaEdge =>
    "UnityEngine.Yoga"."YogaEdge"
);
