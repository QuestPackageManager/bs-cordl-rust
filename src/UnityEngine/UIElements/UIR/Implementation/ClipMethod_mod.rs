#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+ClipMethod")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipMethod {
    NotClipped = 1i32,
    Scissor = 2i32,
    ShaderDiscard = 3i32,
    Stencil = 4i32,
    Undetermined = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+ClipMethod")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::Implementation::ClipMethod =>
    "UnityEngine.UIElements.UIR.Implementation"."ClipMethod"
);
