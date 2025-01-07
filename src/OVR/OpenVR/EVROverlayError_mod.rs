#[cfg(feature = "OVR+OpenVR+EVROverlayError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVROverlayError {
    #[default]
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
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::EVROverlayError {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "EVROverlayError";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "OVR+OpenVR+EVROverlayError")]
unsafe impl quest_hook::libil2cpp::Argument for crate::OVR::OpenVR::EVROverlayError {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVR+OpenVR+EVROverlayError")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::OVR::OpenVR::EVROverlayError {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "OVR+OpenVR+EVROverlayError")]
unsafe impl quest_hook::libil2cpp::Returned for crate::OVR::OpenVR::EVROverlayError {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "OVR+OpenVR+EVROverlayError")]
unsafe impl quest_hook::libil2cpp::Return for crate::OVR::OpenVR::EVROverlayError {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
