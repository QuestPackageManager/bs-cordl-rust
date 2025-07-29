#[cfg(feature = "cordl_class_UnityEngine+UIElements+EasingMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EasingMode {
    #[default]
    Ease = 0i32,
    EaseIn = 1i32,
    EaseInBack = 17i32,
    EaseInBounce = 20i32,
    EaseInCirc = 11i32,
    EaseInCubic = 8i32,
    EaseInElastic = 14i32,
    EaseInOut = 3i32,
    EaseInOutBack = 19i32,
    EaseInOutBounce = 22i32,
    EaseInOutCirc = 13i32,
    EaseInOutCubic = 10i32,
    EaseInOutElastic = 16i32,
    EaseInOutSine = 7i32,
    EaseInSine = 5i32,
    EaseOut = 2i32,
    EaseOutBack = 18i32,
    EaseOutBounce = 21i32,
    EaseOutCirc = 12i32,
    EaseOutCubic = 9i32,
    EaseOutElastic = 15i32,
    EaseOutSine = 6i32,
    Linear = 4i32,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+EasingMode")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UIElements::EasingMode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "EasingMode";
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+EasingMode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::EasingMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+EasingMode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::EasingMode {
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+EasingMode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::EasingMode {
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+EasingMode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::EasingMode {
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
