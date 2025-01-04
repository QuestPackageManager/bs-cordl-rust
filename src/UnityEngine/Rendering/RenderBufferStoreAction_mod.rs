#[cfg(feature = "UnityEngine+Rendering+RenderBufferStoreAction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderBufferStoreAction {
    #[default]
    DontCare = 3i32,
    Resolve = 1i32,
    Store = 0i32,
    StoreAndResolve = 2i32,
}
#[cfg(feature = "UnityEngine+Rendering+RenderBufferStoreAction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::RenderBufferStoreAction
    => "UnityEngine.Rendering"."RenderBufferStoreAction"
);
