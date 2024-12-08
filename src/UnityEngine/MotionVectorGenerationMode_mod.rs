#[cfg(feature = "UnityEngine+MotionVectorGenerationMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionVectorGenerationMode {
    Camera = 0i32,
    ForceNoMotion = 2i32,
    Object = 1i32,
}
#[cfg(feature = "UnityEngine+MotionVectorGenerationMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::MotionVectorGenerationMode =>
    "UnityEngine"."MotionVectorGenerationMode"
);
