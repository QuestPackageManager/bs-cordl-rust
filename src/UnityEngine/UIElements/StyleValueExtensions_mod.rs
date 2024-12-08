#[cfg(feature = "UnityEngine+UIElements+StyleValueExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct StyleValueExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleValueExtensions =>
    "UnityEngine.UIElements"."StyleValueExtensions"
);
#[cfg(feature = "UnityEngine+UIElements+StyleValueExtensions")]
impl std::ops::Deref for crate::UnityEngine::UIElements::StyleValueExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::StyleValueExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueExtensions")]
impl crate::UnityEngine::UIElements::StyleValueExtensions {}
#[cfg(feature = "UnityEngine+UIElements+StyleValueExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleValueExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
