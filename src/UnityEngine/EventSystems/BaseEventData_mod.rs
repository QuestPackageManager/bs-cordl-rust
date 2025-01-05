#[cfg(feature = "UnityEngine+EventSystems+BaseEventData")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseEventData {
    __cordl_parent: crate::UnityEngine::EventSystems::AbstractEventData,
    pub m_EventSystem: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::EventSystem,
    >,
}
#[cfg(feature = "UnityEngine+EventSystems+BaseEventData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EventSystems::BaseEventData =>
    "UnityEngine.EventSystems"."BaseEventData"
);
#[cfg(feature = "UnityEngine+EventSystems+BaseEventData")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::BaseEventData {
    type Target = crate::UnityEngine::EventSystems::AbstractEventData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+BaseEventData")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::BaseEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+BaseEventData")]
impl crate::UnityEngine::EventSystems::BaseEventData {
    pub fn New(
        eventSystem: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::EventSystem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (eventSystem))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        eventSystem: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::EventSystem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (eventSystem))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentInputModule(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseInputModule>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseInputModule,
        > = __cordl_object.invoke("get_currentInputModule", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("get_selectedObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_selectedObject(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_selectedObject", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+EventSystems+BaseEventData")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::BaseEventData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
