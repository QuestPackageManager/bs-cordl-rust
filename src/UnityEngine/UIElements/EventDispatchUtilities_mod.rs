#[cfg(feature = "UnityEngine+UIElements+EventDispatchUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct EventDispatchUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+EventDispatchUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::EventDispatchUtilities
    => "UnityEngine.UIElements"."EventDispatchUtilities"
);
#[cfg(feature = "UnityEngine+UIElements+EventDispatchUtilities")]
impl std::ops::Deref for crate::UnityEngine::UIElements::EventDispatchUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventDispatchUtilities")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::EventDispatchUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventDispatchUtilities")]
impl crate::UnityEngine::UIElements::EventDispatchUtilities {}
#[cfg(feature = "UnityEngine+UIElements+EventDispatchUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::EventDispatchUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
