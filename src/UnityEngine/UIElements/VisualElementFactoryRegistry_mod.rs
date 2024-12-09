#[cfg(feature = "UnityEngine+UIElements+VisualElementFactoryRegistry")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualElementFactoryRegistry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementFactoryRegistry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualElementFactoryRegistry => "UnityEngine.UIElements"
    ."VisualElementFactoryRegistry"
);
#[cfg(feature = "UnityEngine+UIElements+VisualElementFactoryRegistry")]
impl std::ops::Deref for crate::UnityEngine::UIElements::VisualElementFactoryRegistry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementFactoryRegistry")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::VisualElementFactoryRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementFactoryRegistry")]
impl crate::UnityEngine::UIElements::VisualElementFactoryRegistry {}
#[cfg(feature = "UnityEngine+UIElements+VisualElementFactoryRegistry")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualElementFactoryRegistry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
