#[cfg(feature = "UnityEngine+Rendering+RenderBufferLoadAction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderBufferLoadAction {
    Clear = 1i32,
    DontCare = 2i32,
    Load = 0i32,
}
#[cfg(feature = "UnityEngine+Rendering+RenderBufferLoadAction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::RenderBufferLoadAction
    => "UnityEngine.Rendering"."RenderBufferLoadAction"
);
