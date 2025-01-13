#[cfg(feature = "UnityEngine+XR+InputDeviceCharacteristics")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputDeviceCharacteristics {
    #[default]
    Camera = 2u32,
    Controller = 64u32,
    EyeTracking = 16u32,
    HandTracking = 8u32,
    HeadMounted = 1u32,
    HeldInHand = 4u32,
    Left = 256u32,
    None = 0u32,
    Right = 512u32,
    Simulated6DOF = 1024u32,
    TrackedDevice = 32u32,
    TrackingReference = 128u32,
}
#[cfg(feature = "UnityEngine+XR+InputDeviceCharacteristics")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::XR::InputDeviceCharacteristics {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.XR";
    const CLASS_NAME: &'static str = "InputDeviceCharacteristics";
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
#[cfg(feature = "UnityEngine+XR+InputDeviceCharacteristics")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::XR::InputDeviceCharacteristics {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+XR+InputDeviceCharacteristics")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::XR::InputDeviceCharacteristics {
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
#[cfg(feature = "UnityEngine+XR+InputDeviceCharacteristics")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::XR::InputDeviceCharacteristics {
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
#[cfg(feature = "UnityEngine+XR+InputDeviceCharacteristics")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::XR::InputDeviceCharacteristics {
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
