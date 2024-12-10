#[cfg(feature = "UnityEngine+InputSystem+Controls+DiscreteButtonControl")]
#[repr(C)]
#[derive(Debug)]
pub struct DiscreteButtonControl {
    __cordl_parent: crate::UnityEngine::InputSystem::Controls::ButtonControl,
    pub minValue: i32,
    pub maxValue: i32,
    pub wrapAtValue: i32,
    pub nullValue: i32,
    pub writeMode: crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl_WriteMode,
}
#[cfg(feature = "UnityEngine+InputSystem+Controls+DiscreteButtonControl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Controls::DiscreteButtonControl =>
    "UnityEngine.InputSystem.Controls"."DiscreteButtonControl"
);
#[cfg(feature = "UnityEngine+InputSystem+Controls+DiscreteButtonControl")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl {
    type Target = crate::UnityEngine::InputSystem::Controls::ButtonControl;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Controls+DiscreteButtonControl")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Controls+DiscreteButtonControl")]
impl crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl {
    #[cfg(feature = "UnityEngine+InputSystem+Controls+DiscreteButtonControl+WriteMode")]
    pub type WriteMode = crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl_WriteMode;
    pub fn FinishSetup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishSetup", ())?;
        Ok(__cordl_ret.into())
    }
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
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("ReadUnprocessedValueFromState", (statePtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueIntoState(
        &mut self,
        value: f32,
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
#[cfg(feature = "UnityEngine+InputSystem+Controls+DiscreteButtonControl")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Controls+DiscreteButtonControl+WriteMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscreteButtonControl_WriteMode {
    WriteDisabled = 0i32,
    WriteNullAndMaxValue = 1i32,
}
#[cfg(feature = "UnityEngine+InputSystem+Controls+DiscreteButtonControl+WriteMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Controls::DiscreteButtonControl_WriteMode =>
    "UnityEngine.InputSystem.Controls"."DiscreteButtonControl/WriteMode"
);
