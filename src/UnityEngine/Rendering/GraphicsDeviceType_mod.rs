#[cfg(feature = "UnityEngine+Rendering+GraphicsDeviceType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsDeviceType {
    Direct3D11 = 2i32,
    Direct3D12 = 18i32,
    Direct3D9 = 1i32,
    GameCoreScarlett = -1i32,
    GameCoreXboxOne = 24i32,
    GameCoreXboxSeries = 25i32,
    Metal = 16i32,
    N3DS = 19i32,
    Null = 4i32,
    OpenGL2 = 0i32,
    OpenGLCore = 17i32,
    OpenGLES2 = 8i32,
    OpenGLES3 = 11i32,
    PlayStation3 = 3i32,
    PlayStation4 = 13i32,
    PlayStation5 = 26i32,
    PlayStation5NGGC = 27i32,
    PlayStationMobile = 15i32,
    PlayStationVita = 12i32,
    Switch = 22i32,
    Vulkan = 21i32,
    Xbox360 = 6i32,
    XboxOne = 14i32,
    XboxOneD3D12 = 23i32,
}
#[cfg(feature = "UnityEngine+Rendering+GraphicsDeviceType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::GraphicsDeviceType =>
    "UnityEngine.Rendering"."GraphicsDeviceType"
);
