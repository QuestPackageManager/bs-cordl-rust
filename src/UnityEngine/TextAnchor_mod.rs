#[cfg(feature = "UnityEngine+TextAnchor")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextAnchor {
    #[default]
    LowerCenter = 7i32,
    LowerLeft = 6i32,
    LowerRight = 8i32,
    MiddleCenter = 4i32,
    MiddleLeft = 3i32,
    MiddleRight = 5i32,
    UpperCenter = 1i32,
    UpperLeft = 0i32,
    UpperRight = 2i32,
}
#[cfg(feature = "UnityEngine+TextAnchor")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextAnchor => "UnityEngine"
    ."TextAnchor"
);
