#[cfg(feature = "UnityEngine+UIElements+ICommandEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct ICommandEvent {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+ICommandEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ICommandEvent =>
    "UnityEngine.UIElements"."ICommandEvent"
);
#[cfg(feature = "UnityEngine+UIElements+ICommandEvent")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ICommandEvent {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ICommandEvent")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ICommandEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ICommandEvent")]
impl crate::UnityEngine::UIElements::ICommandEvent {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ICommandEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ICommandEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}