#[cfg(feature = "UnityEngine+UIElements+UxmlObjectFactoryRegistry")]
#[repr(C)]
#[derive(Debug)]
pub struct UxmlObjectFactoryRegistry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+UxmlObjectFactoryRegistry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UxmlObjectFactoryRegistry => "UnityEngine.UIElements"
    ."UxmlObjectFactoryRegistry"
);
#[cfg(feature = "UnityEngine+UIElements+UxmlObjectFactoryRegistry")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UxmlObjectFactoryRegistry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlObjectFactoryRegistry")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UxmlObjectFactoryRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlObjectFactoryRegistry")]
impl crate::UnityEngine::UIElements::UxmlObjectFactoryRegistry {}
#[cfg(feature = "UnityEngine+UIElements+UxmlObjectFactoryRegistry")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UxmlObjectFactoryRegistry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
