#[cfg(feature = "UnityEngine+ProBuilder+ComponentMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentMode {
    Edge = 1i32,
    Face = 2i32,
    Vertex = 0i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+ComponentMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::ComponentMode =>
    "UnityEngine.ProBuilder"."ComponentMode"
);
