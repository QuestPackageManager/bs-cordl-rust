#[cfg(feature = "UnityEngine+UIElements+KeyboardNavigationOperation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KeyboardNavigationOperation {
    #[default]
    Begin = 10i32,
    Cancel = 2i32,
    End = 11i32,
    MoveLeft = 7i32,
    MoveRight = 6i32,
    Next = 5i32,
    None = 0i32,
    PageDown = 9i32,
    PageUp = 8i32,
    Previous = 4i32,
    SelectAll = 1i32,
    Submit = 3i32,
}
#[cfg(feature = "UnityEngine+UIElements+KeyboardNavigationOperation")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::KeyboardNavigationOperation {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "KeyboardNavigationOperation";
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
#[cfg(feature = "UnityEngine+UIElements+KeyboardNavigationOperation")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::KeyboardNavigationOperation {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+KeyboardNavigationOperation")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::KeyboardNavigationOperation {
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
#[cfg(feature = "UnityEngine+UIElements+KeyboardNavigationOperation")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::KeyboardNavigationOperation {
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
#[cfg(feature = "UnityEngine+UIElements+KeyboardNavigationOperation")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::KeyboardNavigationOperation {
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
