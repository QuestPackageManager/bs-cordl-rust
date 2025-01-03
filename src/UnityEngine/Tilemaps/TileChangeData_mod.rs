#[cfg(feature = "UnityEngine+Tilemaps+TileChangeData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TileChangeData {
    pub m_Position: crate::UnityEngine::Vector3Int,
    pub m_TileAsset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    pub m_Color: crate::UnityEngine::Color,
    pub m_Transform: crate::UnityEngine::Matrix4x4,
}
#[cfg(feature = "UnityEngine+Tilemaps+TileChangeData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Tilemaps::TileChangeData =>
    "UnityEngine.Tilemaps"."TileChangeData"
);
#[cfg(feature = "UnityEngine+Tilemaps+TileChangeData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Tilemaps::TileChangeData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+TileChangeData")]
impl crate::UnityEngine::Tilemaps::TileChangeData {}
