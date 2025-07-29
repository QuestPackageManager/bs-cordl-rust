#[cfg(feature = "cordl_class_OVR+OpenVR+ETrackedPropertyError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ETrackedPropertyError {
    #[default]
    TrackedProp_BufferTooSmall = 3i32,
    TrackedProp_CannotWriteToWildcards = 12i32,
    TrackedProp_CouldNotContactServer = 6i32,
    TrackedProp_InvalidDevice = 5i32,
    TrackedProp_InvalidOperation = 11i32,
    TrackedProp_NotYetAvailable = 9i32,
    TrackedProp_PermissionDenied = 10i32,
    TrackedProp_StringExceedsMaximumLength = 8i32,
    TrackedProp_Success = 0i32,
    TrackedProp_UnknownProperty = 4i32,
    TrackedProp_ValueNotProvidedByDevice = 7i32,
    TrackedProp_WrongDataType = 1i32,
    TrackedProp_WrongDeviceClass = 2i32,
}
#[cfg(feature = "cordl_class_OVR+OpenVR+ETrackedPropertyError")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::ETrackedPropertyError {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "ETrackedPropertyError";
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
#[cfg(feature = "cordl_class_OVR+OpenVR+ETrackedPropertyError")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::OVR::OpenVR::ETrackedPropertyError {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+ETrackedPropertyError")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::OVR::OpenVR::ETrackedPropertyError {
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
#[cfg(feature = "cordl_class_OVR+OpenVR+ETrackedPropertyError")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::OVR::OpenVR::ETrackedPropertyError {
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
#[cfg(feature = "cordl_class_OVR+OpenVR+ETrackedPropertyError")]
unsafe impl quest_hook::libil2cpp::Return for crate::OVR::OpenVR::ETrackedPropertyError {
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
