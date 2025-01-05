#[cfg(feature = "UnityEngine+InputSystem+Controls+TouchPhaseControl")]
#[repr(C)]
#[derive(Debug)]
pub struct TouchPhaseControl {
    __cordl_parent: crate::UnityEngine::InputSystem::InputControl_1<
        crate::UnityEngine::InputSystem::TouchPhase,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+Controls+TouchPhaseControl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Controls::TouchPhaseControl =>
    "UnityEngine.InputSystem.Controls"."TouchPhaseControl"
);
#[cfg(feature = "UnityEngine+InputSystem+Controls+TouchPhaseControl")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Controls::TouchPhaseControl {
    type Target = crate::UnityEngine::InputSystem::InputControl_1<
        crate::UnityEngine::InputSystem::TouchPhase,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Controls+TouchPhaseControl")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::Controls::TouchPhaseControl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Controls+TouchPhaseControl")]
impl crate::UnityEngine::InputSystem::Controls::TouchPhaseControl {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReadUnprocessedValueFromState(
        &mut self,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::TouchPhase> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::TouchPhase = __cordl_object
            .invoke("ReadUnprocessedValueFromState", (statePtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueIntoState(
        &mut self,
        value: crate::UnityEngine::InputSystem::TouchPhase,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValueIntoState", (value, statePtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Controls+TouchPhaseControl")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Controls::TouchPhaseControl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
