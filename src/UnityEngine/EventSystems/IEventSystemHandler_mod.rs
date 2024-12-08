#[cfg(feature = "UnityEngine+EventSystems+IEventSystemHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct IEventSystemHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+EventSystems+IEventSystemHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EventSystems::IEventSystemHandler
    => "UnityEngine.EventSystems"."IEventSystemHandler"
);
#[cfg(feature = "UnityEngine+EventSystems+IEventSystemHandler")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::IEventSystemHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+IEventSystemHandler")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::IEventSystemHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+IEventSystemHandler")]
impl crate::UnityEngine::EventSystems::IEventSystemHandler {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+IEventSystemHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::IEventSystemHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
