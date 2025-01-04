#[cfg(feature = "OVR+OpenVR+EVRRenderModelError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVRRenderModelError {
    #[default]
    BufferTooSmall = 306i32,
    InvalidArg = 300i32,
    InvalidModel = 301i32,
    InvalidTexture = 400i32,
    Loading = 100i32,
    MultipleShapes = 303i32,
    MultipleTextures = 305i32,
    NoShapes = 302i32,
    None = 0i32,
    NotEnoughNormals = 307i32,
    NotEnoughTexCoords = 308i32,
    NotSupported = 200i32,
    TooManyVertices = 304i32,
}
#[cfg(feature = "OVR+OpenVR+EVRRenderModelError")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRRenderModelError => "OVR.OpenVR"
    ."EVRRenderModelError"
);
