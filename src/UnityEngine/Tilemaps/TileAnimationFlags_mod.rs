#[cfg(feature = "UnityEngine+Tilemaps+TileAnimationFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TileAnimationFlags {
    #[default]
    LoopOnce = 1i32,
    None = 0i32,
    PauseAnimation = 2i32,
    UpdatePhysics = 4i32,
}
#[cfg(feature = "UnityEngine+Tilemaps+TileAnimationFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Tilemaps::TileAnimationFlags =>
    "UnityEngine.Tilemaps"."TileAnimationFlags"
);
