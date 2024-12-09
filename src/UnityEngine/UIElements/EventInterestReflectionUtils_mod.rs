#[cfg(feature = "UnityEngine+UIElements+EventInterestReflectionUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct EventInterestReflectionUtils {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+EventInterestReflectionUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::EventInterestReflectionUtils => "UnityEngine.UIElements"
    ."EventInterestReflectionUtils"
);
#[cfg(feature = "UnityEngine+UIElements+EventInterestReflectionUtils")]
impl std::ops::Deref for crate::UnityEngine::UIElements::EventInterestReflectionUtils {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventInterestReflectionUtils")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::EventInterestReflectionUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventInterestReflectionUtils")]
impl crate::UnityEngine::UIElements::EventInterestReflectionUtils {
    #[cfg(
        feature = "UnityEngine+UIElements+EventInterestReflectionUtils+DefaultEventInterests"
    )]
    pub type DefaultEventInterests = crate::UnityEngine::UIElements::EventInterestReflectionUtils_DefaultEventInterests;
}
#[cfg(feature = "UnityEngine+UIElements+EventInterestReflectionUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::EventInterestReflectionUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+EventInterestReflectionUtils+DefaultEventInterests"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct EventInterestReflectionUtils_DefaultEventInterests {
    pub DefaultActionCategories: i32,
    pub DefaultActionAtTargetCategories: i32,
}
#[cfg(
    feature = "UnityEngine+UIElements+EventInterestReflectionUtils+DefaultEventInterests"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::EventInterestReflectionUtils_DefaultEventInterests =>
    "UnityEngine.UIElements"."EventInterestReflectionUtils/DefaultEventInterests"
);
#[cfg(
    feature = "UnityEngine+UIElements+EventInterestReflectionUtils+DefaultEventInterests"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::EventInterestReflectionUtils_DefaultEventInterests {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+EventInterestReflectionUtils+DefaultEventInterests"
)]
impl crate::UnityEngine::UIElements::EventInterestReflectionUtils_DefaultEventInterests {}
