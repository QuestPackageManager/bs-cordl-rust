#[cfg(feature = "UnityEngine+EventSystems+ICancelHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct ICancelHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+EventSystems+ICancelHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EventSystems::ICancelHandler =>
    "UnityEngine.EventSystems"."ICancelHandler"
);
#[cfg(feature = "UnityEngine+EventSystems+ICancelHandler")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::ICancelHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+ICancelHandler")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::ICancelHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+ICancelHandler")]
impl crate::UnityEngine::EventSystems::ICancelHandler {
    pub fn OnCancel(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::BaseEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCancel", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+ICancelHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::ICancelHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
