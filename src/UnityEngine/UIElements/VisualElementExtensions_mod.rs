#[cfg(feature = "UnityEngine+UIElements+VisualElementExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualElementExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::VisualElementExtensions
    => "UnityEngine.UIElements"."VisualElementExtensions"
);
#[cfg(feature = "UnityEngine+UIElements+VisualElementExtensions")]
impl std::ops::Deref for crate::UnityEngine::UIElements::VisualElementExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::VisualElementExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementExtensions")]
impl crate::UnityEngine::UIElements::VisualElementExtensions {}
#[cfg(feature = "UnityEngine+UIElements+VisualElementExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualElementExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
