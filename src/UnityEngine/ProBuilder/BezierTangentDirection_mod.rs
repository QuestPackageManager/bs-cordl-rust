#[cfg(feature = "UnityEngine+ProBuilder+BezierTangentDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BezierTangentDirection {
    #[default]
    In = 0i32,
    Out = 1i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+BezierTangentDirection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::BezierTangentDirection
    => "UnityEngine.ProBuilder"."BezierTangentDirection"
);
