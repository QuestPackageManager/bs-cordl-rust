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
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::EventSystems::BaseEventData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.EventSystems";
    const CLASS_NAME: &'static str = "BaseEventData";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
