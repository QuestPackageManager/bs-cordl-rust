#[cfg(feature = "UnityEngine+TerrainUtils+TerrainTileCoord")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TerrainTileCoord {
    pub tileX: i32,
    pub tileZ: i32,
}
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainTileCoord")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TerrainUtils::TerrainTileCoord =>
    "UnityEngine.TerrainUtils"."TerrainTileCoord"
);
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainTileCoord")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TerrainUtils::TerrainTileCoord {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainTileCoord")]
impl crate::UnityEngine::TerrainUtils::TerrainTileCoord {
    pub fn _ctor(
        &mut self,
        tileX: i32,
        tileZ: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (tileX, tileZ),
        )?;
        Ok(__cordl_ret.into())
    }
}
