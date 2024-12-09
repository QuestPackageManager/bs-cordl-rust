#[cfg(feature = "UnityEngine+UIElements+StyleValueKeywordExtension")]
#[repr(C)]
#[derive(Debug)]
pub struct StyleValueKeywordExtension {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueKeywordExtension")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleValueKeywordExtension => "UnityEngine.UIElements"
    ."StyleValueKeywordExtension"
);
#[cfg(feature = "UnityEngine+UIElements+StyleValueKeywordExtension")]
impl std::ops::Deref for crate::UnityEngine::UIElements::StyleValueKeywordExtension {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueKeywordExtension")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::StyleValueKeywordExtension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueKeywordExtension")]
impl crate::UnityEngine::UIElements::StyleValueKeywordExtension {}
#[cfg(feature = "UnityEngine+UIElements+StyleValueKeywordExtension")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleValueKeywordExtension {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
