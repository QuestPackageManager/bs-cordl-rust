#[cfg(feature = "UnityEngine+ImagePosition")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImagePosition {
    ImageAbove = 1i32,
    ImageLeft = 0i32,
    ImageOnly = 2i32,
    TextOnly = 3i32,
}
#[cfg(feature = "UnityEngine+ImagePosition")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ImagePosition => "UnityEngine"
    ."ImagePosition"
);
