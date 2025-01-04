#[cfg(feature = "UnityEngine+FilterMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FilterMode {
    #[default]
    Bilinear = 1i32,
    Point = 0i32,
    Trilinear = 2i32,
}
#[cfg(feature = "UnityEngine+FilterMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::FilterMode => "UnityEngine"
    ."FilterMode"
);
