#[cfg(feature = "UnityEngine+DetailRenderMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DetailRenderMode {
    #[default]
    Grass = 2i32,
    GrassBillboard = 0i32,
    VertexLit = 1i32,
}
#[cfg(feature = "UnityEngine+DetailRenderMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::DetailRenderMode => "UnityEngine"
    ."DetailRenderMode"
);
