#[cfg(feature = "UnityEngine+Experimental+Rendering+FormatUsage")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FormatUsage {
    #[default]
    Blend = 5i32,
    GetPixels = 6i32,
    Linear = 1i32,
    LoadStore = 10i32,
    MSAA2x = 11i32,
    MSAA4x = 12i32,
    MSAA8x = 13i32,
    ReadPixels = 9i32,
    Render = 4i32,
    Sample = 0i32,
    SetPixels = 7i32,
    SetPixels32 = 8i32,
    Sparse = 2i32,
    StencilSampling = 16i32,
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+FormatUsage")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::Rendering::FormatUsage =>
    "UnityEngine.Experimental.Rendering"."FormatUsage"
);
