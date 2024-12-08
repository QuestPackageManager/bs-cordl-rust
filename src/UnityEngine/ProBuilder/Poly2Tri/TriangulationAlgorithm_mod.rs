#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+TriangulationAlgorithm")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriangulationAlgorithm {
    DTSweep = 0i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+TriangulationAlgorithm")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::Poly2Tri::TriangulationAlgorithm =>
    "UnityEngine.ProBuilder.Poly2Tri"."TriangulationAlgorithm"
);
