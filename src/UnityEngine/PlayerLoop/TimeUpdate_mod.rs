#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+TimeUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimeUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+TimeUpdate")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::PlayerLoop::TimeUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "TimeUpdate";
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+TimeUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::TimeUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+TimeUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::TimeUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+TimeUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::TimeUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+TimeUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::TimeUpdate {
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
#[cfg(feature = "cordl_class_UnityEngine+PlayerLoop+TimeUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::TimeUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PlayerLoop+TimeUpdate")]
impl crate::UnityEngine::PlayerLoop::TimeUpdate {
    #[cfg(
        feature = "UnityEngine+PlayerLoop+TimeUpdate+WaitForLastPresentationAndUpdateTime"
    )]
    pub type WaitForLastPresentationAndUpdateTime = crate::UnityEngine::PlayerLoop::TimeUpdate_WaitForLastPresentationAndUpdateTime;
}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+TimeUpdate+WaitForLastPresentationAndUpdateTime"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimeUpdate_WaitForLastPresentationAndUpdateTime {}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+TimeUpdate+WaitForLastPresentationAndUpdateTime"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::PlayerLoop::TimeUpdate_WaitForLastPresentationAndUpdateTime {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.PlayerLoop";
    const CLASS_NAME: &'static str = "TimeUpdate/WaitForLastPresentationAndUpdateTime";
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
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+TimeUpdate+WaitForLastPresentationAndUpdateTime"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::PlayerLoop::TimeUpdate_WaitForLastPresentationAndUpdateTime {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+TimeUpdate+WaitForLastPresentationAndUpdateTime"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::PlayerLoop::TimeUpdate_WaitForLastPresentationAndUpdateTime {
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
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+TimeUpdate+WaitForLastPresentationAndUpdateTime"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::PlayerLoop::TimeUpdate_WaitForLastPresentationAndUpdateTime {
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
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+TimeUpdate+WaitForLastPresentationAndUpdateTime"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::PlayerLoop::TimeUpdate_WaitForLastPresentationAndUpdateTime {
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
#[cfg(
    feature = "cordl_class_UnityEngine+PlayerLoop+TimeUpdate+WaitForLastPresentationAndUpdateTime"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::PlayerLoop::TimeUpdate_WaitForLastPresentationAndUpdateTime {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+PlayerLoop+TimeUpdate+WaitForLastPresentationAndUpdateTime"
)]
impl crate::UnityEngine::PlayerLoop::TimeUpdate_WaitForLastPresentationAndUpdateTime {}
