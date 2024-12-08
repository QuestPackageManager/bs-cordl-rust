#[cfg(feature = "UnityEngine+Rendering+BatchCullingViewType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatchCullingViewType {
    Camera = 1i32,
    Light = 2i32,
    Picking = 3i32,
    SelectionOutline = 4i32,
    Unknown = 0i32,
}
#[cfg(feature = "UnityEngine+Rendering+BatchCullingViewType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::BatchCullingViewType =>
    "UnityEngine.Rendering"."BatchCullingViewType"
);
