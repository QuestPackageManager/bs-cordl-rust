#[cfg(feature = "UnityEngine+EventSystems+RaycasterManager")]
#[repr(C)]
#[derive(Debug)]
pub struct RaycasterManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+EventSystems+RaycasterManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EventSystems::RaycasterManager =>
    "UnityEngine.EventSystems"."RaycasterManager"
);
#[cfg(feature = "UnityEngine+EventSystems+RaycasterManager")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::RaycasterManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+RaycasterManager")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::RaycasterManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+RaycasterManager")]
impl crate::UnityEngine::EventSystems::RaycasterManager {}
#[cfg(feature = "UnityEngine+EventSystems+RaycasterManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::RaycasterManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
