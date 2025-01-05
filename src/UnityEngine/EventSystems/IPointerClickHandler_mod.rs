#[cfg(feature = "UnityEngine+EventSystems+IPointerClickHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct IPointerClickHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+EventSystems+IPointerClickHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EventSystems::IPointerClickHandler
    => "UnityEngine.EventSystems"."IPointerClickHandler"
);
#[cfg(feature = "UnityEngine+EventSystems+IPointerClickHandler")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::IPointerClickHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+IPointerClickHandler")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::IPointerClickHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+IPointerClickHandler")]
impl crate::UnityEngine::EventSystems::IPointerClickHandler {
    pub fn OnPointerClick(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerClick", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+IPointerClickHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::IPointerClickHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+EventSystems+IPointerClickHandler")]
impl AsRef<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::UnityEngine::EventSystems::IPointerClickHandler {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+IPointerClickHandler")]
impl AsMut<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::UnityEngine::EventSystems::IPointerClickHandler {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
