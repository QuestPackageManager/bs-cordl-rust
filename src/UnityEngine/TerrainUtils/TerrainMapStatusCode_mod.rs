#[cfg(feature = "UnityEngine+TerrainUtils+TerrainMapStatusCode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TerrainMapStatusCode {
    #[default]
    EdgeAlignmentMismatch = 8i32,
    OK = 0i32,
    Overlapping = 1i32,
    SizeMismatch = 4i32,
}
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainMapStatusCode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TerrainUtils::TerrainMapStatusCode
    => "UnityEngine.TerrainUtils"."TerrainMapStatusCode"
);
