#[cfg(feature = "UnityEngine+LineTextureMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineTextureMode {
    #[default]
    DistributePerSegment = 2i32,
    RepeatPerSegment = 3i32,
    Static = 4i32,
    Stretch = 0i32,
    Tile = 1i32,
}
#[cfg(feature = "UnityEngine+LineTextureMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::LineTextureMode => "UnityEngine"
    ."LineTextureMode"
);
