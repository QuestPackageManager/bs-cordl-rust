#[cfg(feature = "UnityEngine+Experimental+Rendering+DefaultFormat")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DefaultFormat {
    #[default]
    DepthStencil = 2i32,
    HDR = 1i32,
    LDR = 0i32,
    Shadow = 3i32,
    Video = 4i32,
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+DefaultFormat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::Rendering::DefaultFormat =>
    "UnityEngine.Experimental.Rendering"."DefaultFormat"
);
