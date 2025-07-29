#[cfg(feature = "cordl_class_OVR+OpenVR+EVRTrackedCameraError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVRTrackedCameraError {
    #[default]
    FailedToGetGLTextureId = 111i32,
    FrameBufferingFailure = 107i32,
    IPCFailure = 104i32,
    InvalidArgument = 114i32,
    InvalidFrameBufferSize = 115i32,
    InvalidFrameHeaderVersion = 102i32,
    InvalidGLTextureId = 109i32,
    InvalidHandle = 101i32,
    InvalidSharedTextureHandle = 110i32,
    NoFrameAvailable = 113i32,
    None = 0i32,
    NotSupportedForThisDevice = 105i32,
    OperationFailed = 100i32,
    OutOfHandles = 103i32,
    SharedMemoryFailure = 106i32,
    SharedTextureFailure = 112i32,
    StreamSetupFailure = 108i32,
}
#[cfg(feature = "cordl_class_OVR+OpenVR+EVRTrackedCameraError")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::EVRTrackedCameraError {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "EVRTrackedCameraError";
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
#[cfg(feature = "cordl_class_OVR+OpenVR+EVRTrackedCameraError")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::OVR::OpenVR::EVRTrackedCameraError {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+EVRTrackedCameraError")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::OVR::OpenVR::EVRTrackedCameraError {
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
#[cfg(feature = "cordl_class_OVR+OpenVR+EVRTrackedCameraError")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::OVR::OpenVR::EVRTrackedCameraError {
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
#[cfg(feature = "cordl_class_OVR+OpenVR+EVRTrackedCameraError")]
unsafe impl quest_hook::libil2cpp::Return for crate::OVR::OpenVR::EVRTrackedCameraError {
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
