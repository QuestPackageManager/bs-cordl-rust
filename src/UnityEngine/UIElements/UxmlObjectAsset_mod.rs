#[cfg(feature = "UnityEngine+UIElements+UxmlObjectAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct UxmlObjectAsset {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlAsset,
}
#[cfg(feature = "UnityEngine+UIElements+UxmlObjectAsset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UxmlObjectAsset =>
    "UnityEngine.UIElements"."UxmlObjectAsset"
);
#[cfg(feature = "UnityEngine+UIElements+UxmlObjectAsset")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UxmlObjectAsset {
    type Target = crate::UnityEngine::UIElements::UxmlAsset;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlObjectAsset")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UxmlObjectAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlObjectAsset")]
impl crate::UnityEngine::UIElements::UxmlObjectAsset {}
#[cfg(feature = "UnityEngine+UIElements+UxmlObjectAsset")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UxmlObjectAsset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
