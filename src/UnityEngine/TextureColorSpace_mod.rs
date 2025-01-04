#[cfg(feature = "UnityEngine+TextureColorSpace")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextureColorSpace {
    #[default]
    Linear = 0i32,
    sRGB = 1i32,
}
#[cfg(feature = "UnityEngine+TextureColorSpace")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextureColorSpace => "UnityEngine"
    ."TextureColorSpace"
);
