#[cfg(feature = "UnityEngine+EventSystems+IMoveHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct IMoveHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+EventSystems+IMoveHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EventSystems::IMoveHandler =>
    "UnityEngine.EventSystems"."IMoveHandler"
);
#[cfg(feature = "UnityEngine+EventSystems+IMoveHandler")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::IMoveHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+IMoveHandler")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::IMoveHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+IMoveHandler")]
impl crate::UnityEngine::EventSystems::IMoveHandler {
    pub fn OnMove(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::AxisEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnMove", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+IMoveHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::IMoveHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+EventSystems+IMoveHandler")]
impl AsRef<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::UnityEngine::EventSystems::IMoveHandler {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+IMoveHandler")]
impl AsMut<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::UnityEngine::EventSystems::IMoveHandler {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
