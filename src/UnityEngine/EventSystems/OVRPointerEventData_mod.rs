#[cfg(feature = "UnityEngine+EventSystems+OVRPointerEventData")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPointerEventData {
    __cordl_parent: crate::UnityEngine::EventSystems::PointerEventData,
    pub worldSpaceRay: crate::UnityEngine::Ray,
    pub swipeStart: crate::UnityEngine::Vector2,
}
#[cfg(feature = "UnityEngine+EventSystems+OVRPointerEventData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EventSystems::OVRPointerEventData
    => "UnityEngine.EventSystems"."OVRPointerEventData"
);
#[cfg(feature = "UnityEngine+EventSystems+OVRPointerEventData")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::OVRPointerEventData {
    type Target = crate::UnityEngine::EventSystems::PointerEventData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+OVRPointerEventData")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::OVRPointerEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+OVRPointerEventData")]
impl crate::UnityEngine::EventSystems::OVRPointerEventData {
    pub fn New(
        eventSystem: *mut crate::UnityEngine::EventSystems::EventSystem,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (eventSystem))?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        eventSystem: *mut crate::UnityEngine::EventSystems::EventSystem,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (eventSystem))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+EventSystems+OVRPointerEventData")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::OVRPointerEventData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
