#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+NativeTypes+XrViewConfigurationType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XrViewConfigurationType {
    #[default]
    PrimaryMono = 1i32,
    PrimaryQuadVarjo = 1000037000i32,
    PrimaryStereo = 2i32,
    SecondaryMonoFirstPersonObserver = 1000054000i32,
    SecondaryMonoThirdPersonObserver = 1000145000i32,
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+NativeTypes+XrViewConfigurationType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::XR::OpenXR::NativeTypes::XrViewConfigurationType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.XR.OpenXR.NativeTypes";
    const CLASS_NAME: &'static str = "XrViewConfigurationType";
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+NativeTypes+XrViewConfigurationType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::XR::OpenXR::NativeTypes::XrViewConfigurationType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+NativeTypes+XrViewConfigurationType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::XR::OpenXR::NativeTypes::XrViewConfigurationType {
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+NativeTypes+XrViewConfigurationType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::XR::OpenXR::NativeTypes::XrViewConfigurationType {
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+NativeTypes+XrViewConfigurationType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::XR::OpenXR::NativeTypes::XrViewConfigurationType {
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
