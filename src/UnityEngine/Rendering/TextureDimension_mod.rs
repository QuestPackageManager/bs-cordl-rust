#[cfg(feature = "UnityEngine+Rendering+TextureDimension")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextureDimension {
    #[default]
    Any = 1i32,
    Cube = 4i32,
    CubeArray = 6i32,
    None = 0i32,
    Tex2D = 2i32,
    Tex2DArray = 5i32,
    Tex3D = 3i32,
    Unknown = -1i32,
}
#[cfg(feature = "UnityEngine+Rendering+TextureDimension")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::TextureDimension =>
    "UnityEngine.Rendering"."TextureDimension"
);
