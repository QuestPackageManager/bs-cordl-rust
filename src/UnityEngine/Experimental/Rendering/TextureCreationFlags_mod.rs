#[cfg(feature = "UnityEngine+Experimental+Rendering+TextureCreationFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextureCreationFlags {
    #[default]
    Crunch = 64i32,
    DontInitializePixels = 4i32,
    DontUploadUponCreate = 1024i32,
    IgnoreMipmapLimit = 2048i32,
    MipChain = 1i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+TextureCreationFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::Rendering::TextureCreationFlags =>
    "UnityEngine.Experimental.Rendering"."TextureCreationFlags"
);
