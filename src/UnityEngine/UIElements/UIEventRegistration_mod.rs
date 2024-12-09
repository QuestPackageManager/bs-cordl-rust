#[cfg(feature = "UnityEngine+UIElements+UIEventRegistration")]
#[repr(C)]
#[derive(Debug)]
pub struct UIEventRegistration {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+UIEventRegistration")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIEventRegistration =>
    "UnityEngine.UIElements"."UIEventRegistration"
);
#[cfg(feature = "UnityEngine+UIElements+UIEventRegistration")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIEventRegistration {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIEventRegistration")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIEventRegistration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIEventRegistration")]
impl crate::UnityEngine::UIElements::UIEventRegistration {
    #[cfg(feature = "UnityEngine+UIElements+UIEventRegistration+__c")]
    pub type __c = crate::UnityEngine::UIElements::UIEventRegistration___c;
}
#[cfg(feature = "UnityEngine+UIElements+UIEventRegistration")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIEventRegistration {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
