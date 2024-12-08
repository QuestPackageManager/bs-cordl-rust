#[cfg(feature = "UnityEngine+EventSystems+IPointerEnterHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct IPointerEnterHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+EventSystems+IPointerEnterHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EventSystems::IPointerEnterHandler
    => "UnityEngine.EventSystems"."IPointerEnterHandler"
);
#[cfg(feature = "UnityEngine+EventSystems+IPointerEnterHandler")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::IPointerEnterHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+IPointerEnterHandler")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::IPointerEnterHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+IPointerEnterHandler")]
impl crate::UnityEngine::EventSystems::IPointerEnterHandler {
    pub fn OnPointerEnter(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerEnter", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+IPointerEnterHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::IPointerEnterHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
