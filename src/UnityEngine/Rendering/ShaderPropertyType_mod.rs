#[cfg(feature = "UnityEngine+Rendering+ShaderPropertyType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShaderPropertyType {
    #[default]
    Color = 0i32,
    Float = 2i32,
    Int = 5i32,
    Range = 3i32,
    Texture = 4i32,
    Vector = 1i32,
}
#[cfg(feature = "UnityEngine+Rendering+ShaderPropertyType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::ShaderPropertyType =>
    "UnityEngine.Rendering"."ShaderPropertyType"
);
