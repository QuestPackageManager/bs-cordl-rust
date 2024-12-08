#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+TriangulationMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriangulationMode {
    Constrained = 1i32,
    Polygon = 2i32,
    Unconstrained = 0i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+TriangulationMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::Poly2Tri::TriangulationMode =>
    "UnityEngine.ProBuilder.Poly2Tri"."TriangulationMode"
);
