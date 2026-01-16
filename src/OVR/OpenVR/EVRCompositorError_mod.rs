#[cfg(feature = "cordl_class_OVR+OpenVR+EVRCompositorError")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum EVRCompositorError {
    #[default]
    AlreadySubmitted = 108i32,
    DoNotHaveFocus = 101i32,
    IncompatibleVersion = 100i32,
    IndexOutOfRange = 107i32,
    InvalidBounds = 109i32,
    InvalidTexture = 102i32,
    IsNotSceneApplication = 103i32,
    None = 0i32,
    RequestFailed = 1i32,
    SharedTexturesNotSupported = 106i32,
    TextureIsOnWrongDevice = 104i32,
    TextureUsesUnsupportedFormat = 105i32,
}
#[cfg(feature = "cordl_class_OVR+OpenVR+EVRCompositorError")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::EVRCompositorError {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "EVRCompositorError";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+EVRCompositorError")]
unsafe impl quest_hook::libil2cpp::Argument for crate::OVR::OpenVR::EVRCompositorError {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+EVRCompositorError")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::OVR::OpenVR::EVRCompositorError {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+EVRCompositorError")]
unsafe impl quest_hook::libil2cpp::Returned for crate::OVR::OpenVR::EVRCompositorError {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+EVRCompositorError")]
unsafe impl quest_hook::libil2cpp::Return for crate::OVR::OpenVR::EVRCompositorError {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
