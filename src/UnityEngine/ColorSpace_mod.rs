#[cfg(feature = "UnityEngine+ColorSpace")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpace {
    Gamma = 0i32,
    Linear = 1i32,
    Uninitialized = -1i32,
}
#[cfg(feature = "UnityEngine+ColorSpace")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ColorSpace => "UnityEngine"
    ."ColorSpace"
);
