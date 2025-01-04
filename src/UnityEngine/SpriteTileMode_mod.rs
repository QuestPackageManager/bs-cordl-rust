#[cfg(feature = "UnityEngine+SpriteTileMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpriteTileMode {
    #[default]
    Adaptive = 1i32,
    Continuous = 0i32,
}
#[cfg(feature = "UnityEngine+SpriteTileMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SpriteTileMode => "UnityEngine"
    ."SpriteTileMode"
);
