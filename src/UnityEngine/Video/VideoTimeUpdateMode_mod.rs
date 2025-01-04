#[cfg(feature = "UnityEngine+Video+VideoTimeUpdateMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VideoTimeUpdateMode {
    #[default]
    DSPTime = 0i32,
    GameTime = 1i32,
    UnscaledGameTime = 2i32,
}
#[cfg(feature = "UnityEngine+Video+VideoTimeUpdateMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Video::VideoTimeUpdateMode =>
    "UnityEngine.Video"."VideoTimeUpdateMode"
);
