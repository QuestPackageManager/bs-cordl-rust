#[cfg(feature = "UnityEngine+Rendering+ReflectionProbeTimeSlicingMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReflectionProbeTimeSlicingMode {
    AllFacesAtOnce = 0i32,
    IndividualFaces = 1i32,
    NoTimeSlicing = 2i32,
}
#[cfg(feature = "UnityEngine+Rendering+ReflectionProbeTimeSlicingMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Rendering::ReflectionProbeTimeSlicingMode => "UnityEngine.Rendering"
    ."ReflectionProbeTimeSlicingMode"
);
