#[cfg(feature = "UnityEngine+Tilemaps+TileFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileFlags {
    InstantiateGameObjectRuntimeOnly = 4i32,
    KeepGameObjectRuntimeOnly = 8i32,
    LockAll = 3i32,
    LockColor = 1i32,
    LockTransform = 2i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+Tilemaps+TileFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Tilemaps::TileFlags =>
    "UnityEngine.Tilemaps"."TileFlags"
);
