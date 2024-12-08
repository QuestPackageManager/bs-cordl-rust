#[cfg(feature = "UnityEngine+Rendering+BlendShapeBufferLayout")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendShapeBufferLayout {
    PerShape = 0i32,
    PerVertex = 1i32,
}
#[cfg(feature = "UnityEngine+Rendering+BlendShapeBufferLayout")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::BlendShapeBufferLayout
    => "UnityEngine.Rendering"."BlendShapeBufferLayout"
);
