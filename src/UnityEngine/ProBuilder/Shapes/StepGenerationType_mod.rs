#[cfg(feature = "UnityEngine+ProBuilder+Shapes+StepGenerationType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepGenerationType {
    Count = 1i32,
    Height = 0i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+StepGenerationType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::Shapes::StepGenerationType =>
    "UnityEngine.ProBuilder.Shapes"."StepGenerationType"
);
