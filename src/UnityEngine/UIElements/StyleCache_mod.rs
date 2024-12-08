#[cfg(feature = "UnityEngine+UIElements+StyleCache")]
#[repr(C)]
#[derive(Debug)]
pub struct StyleCache {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+StyleCache")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleCache =>
    "UnityEngine.UIElements"."StyleCache"
);
#[cfg(feature = "UnityEngine+UIElements+StyleCache")]
impl std::ops::Deref for crate::UnityEngine::UIElements::StyleCache {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleCache")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::StyleCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleCache")]
impl crate::UnityEngine::UIElements::StyleCache {}
#[cfg(feature = "UnityEngine+UIElements+StyleCache")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::StyleCache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
