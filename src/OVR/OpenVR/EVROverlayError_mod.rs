#[cfg(feature = "OVR+OpenVR+EVROverlayError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EVROverlayError {
    ArrayTooSmall = 22i32,
    BadMaskPrimitive = 30i32,
    InvalidHandle = 11i32,
    InvalidParameter = 20i32,
    InvalidTexture = 24i32,
    InvalidTrackedDevice = 19i32,
    KeyInUse = 17i32,
    KeyTooLong = 15i32,
    KeyboardAlreadyInUse = 26i32,
    NameTooLong = 16i32,
    NoNeighbor = 27i32,
    None = 0i32,
    OverlayLimitExceeded = 13i32,
    PermissionDenied = 12i32,
    RequestFailed = 23i32,
    TextureAlreadyLocked = 31i32,
    TextureLockCapacityReached = 32i32,
    TextureNotLocked = 33i32,
    ThumbnailCantBeDestroyed = 21i32,
    TooManyMaskPrimitives = 29i32,
    UnableToLoadFile = 25i32,
    UnknownOverlay = 10i32,
    WrongTransformType = 18i32,
    WrongVisibilityType = 14i32,
}
#[cfg(feature = "OVR+OpenVR+EVROverlayError")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVROverlayError => "OVR.OpenVR"
    ."EVROverlayError"
);
