#[cfg(feature = "UnityEngine+Rendering+ReflectionProbeClearFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReflectionProbeClearFlags {
    Skybox = 1i32,
    SolidColor = 2i32,
}
#[cfg(feature = "UnityEngine+Rendering+ReflectionProbeClearFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Rendering::ReflectionProbeClearFlags => "UnityEngine.Rendering"
    ."ReflectionProbeClearFlags"
);
