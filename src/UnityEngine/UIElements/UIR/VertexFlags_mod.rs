#[cfg(feature = "UnityEngine+UIElements+UIR+VertexFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VertexFlags {
    #[default]
    IsDynamic = 3i32,
    IsGraphViewEdge = 10i32,
    IsSolid = 0i32,
    IsSvgGradients = 4i32,
    IsText = 1i32,
    IsTextured = 2i32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+VertexFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::VertexFlags =>
    "UnityEngine.UIElements.UIR"."VertexFlags"
);
