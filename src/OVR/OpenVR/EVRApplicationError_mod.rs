#[cfg(feature = "cordl_class_OVR+OpenVR+EVRApplicationError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVRApplicationError {
    #[default]
    AppKeyAlreadyExists = 100i32,
    ApplicationAlreadyRunning = 106i32,
    ApplicationAlreadyStarting = 110i32,
    BufferTooSmall = 200i32,
    IPCFailed = 105i32,
    InvalidApplication = 108i32,
    InvalidIndex = 103i32,
    InvalidManifest = 107i32,
    InvalidParameter = 203i32,
    IsTemplate = 114i32,
    LaunchFailed = 109i32,
    LaunchInProgress = 111i32,
    NoApplication = 102i32,
    NoManifest = 101i32,
    None = 0i32,
    OldApplicationQuitting = 112i32,
    PropertyNotSet = 201i32,
    SteamVRIsExiting = 115i32,
    TransitionAborted = 113i32,
    UnknownApplication = 104i32,
    UnknownProperty = 202i32,
}
#[cfg(feature = "cordl_class_OVR+OpenVR+EVRApplicationError")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::EVRApplicationError {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "EVRApplicationError";
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
#[cfg(feature = "cordl_class_OVR+OpenVR+EVRApplicationError")]
unsafe impl quest_hook::libil2cpp::Argument for crate::OVR::OpenVR::EVRApplicationError {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+EVRApplicationError")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::OVR::OpenVR::EVRApplicationError {
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
#[cfg(feature = "cordl_class_OVR+OpenVR+EVRApplicationError")]
unsafe impl quest_hook::libil2cpp::Returned for crate::OVR::OpenVR::EVRApplicationError {
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
#[cfg(feature = "cordl_class_OVR+OpenVR+EVRApplicationError")]
unsafe impl quest_hook::libil2cpp::Return for crate::OVR::OpenVR::EVRApplicationError {
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
