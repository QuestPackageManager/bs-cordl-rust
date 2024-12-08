#[cfg(feature = "UnityEngine+ProBuilder+BezierTangentMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BezierTangentMode {
    Aligned = 1i32,
    Free = 0i32,
    Mirrored = 2i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+BezierTangentMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::BezierTangentMode =>
    "UnityEngine.ProBuilder"."BezierTangentMode"
);
