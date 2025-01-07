#[cfg(feature = "UnityEngine+EventSystems+AxisEventData")]
#[repr(C)]
#[derive(Debug)]
pub struct AxisEventData {
    __cordl_parent: crate::UnityEngine::EventSystems::BaseEventData,
    pub _moveVector_k__BackingField: crate::UnityEngine::Vector2,
    pub _moveDir_k__BackingField: crate::UnityEngine::EventSystems::MoveDirection,
}
#[cfg(feature = "UnityEngine+EventSystems+AxisEventData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::EventSystems::AxisEventData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.EventSystems";
    const CLASS_NAME: &'static str = "AxisEventData";
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
#[cfg(feature = "UnityEngine+EventSystems+AxisEventData")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::AxisEventData {
    type Target = crate::UnityEngine::EventSystems::BaseEventData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+AxisEventData")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::AxisEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+AxisEventData")]
impl crate::UnityEngine::EventSystems::AxisEventData {
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
    pub fn get_moveDir(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::EventSystems::MoveDirection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::EventSystems::MoveDirection = __cordl_object
            .invoke("get_moveDir", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_moveVector(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_moveVector", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_moveDir(
        &mut self,
        value: crate::UnityEngine::EventSystems::MoveDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_moveDir", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_moveVector(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_moveVector", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+EventSystems+AxisEventData")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::AxisEventData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
