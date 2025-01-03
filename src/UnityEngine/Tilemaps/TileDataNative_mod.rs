#[cfg(feature = "UnityEngine+Tilemaps+TileDataNative")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TileDataNative {
    pub m_Sprite: i32,
    pub m_Color: crate::UnityEngine::Color,
    pub m_Transform: crate::UnityEngine::Matrix4x4,
    pub m_GameObject: i32,
    pub m_Flags: crate::UnityEngine::Tilemaps::TileFlags,
    pub m_ColliderType: crate::UnityEngine::Tilemaps::Tile_ColliderType,
}
#[cfg(feature = "UnityEngine+Tilemaps+TileDataNative")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Tilemaps::TileDataNative =>
    "UnityEngine.Tilemaps"."TileDataNative"
);
#[cfg(feature = "UnityEngine+Tilemaps+TileDataNative")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Tilemaps::TileDataNative {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+TileDataNative")]
impl crate::UnityEngine::Tilemaps::TileDataNative {}
