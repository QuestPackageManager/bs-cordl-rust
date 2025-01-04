#[cfg(feature = "UnityEngine+ProBuilder+ExtrudeMethod")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExtrudeMethod {
    #[default]
    FaceNormal = 2i32,
    IndividualFaces = 0i32,
    VertexNormal = 1i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+ExtrudeMethod")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::ExtrudeMethod =>
    "UnityEngine.ProBuilder"."ExtrudeMethod"
);
