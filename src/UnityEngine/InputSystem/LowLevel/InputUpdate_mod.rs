#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate")]
#[repr(C)]
#[derive(Debug)]
pub struct InputUpdate {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::LowLevel::InputUpdate
    => "UnityEngine.InputSystem.LowLevel"."InputUpdate"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::LowLevel::InputUpdate {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::LowLevel::InputUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate")]
impl crate::UnityEngine::InputSystem::LowLevel::InputUpdate {
    #[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+SerializedState")]
    pub type SerializedState = crate::UnityEngine::InputSystem::LowLevel::InputUpdate_SerializedState;
    #[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+UpdateStepCount")]
    pub type UpdateStepCount = crate::UnityEngine::InputSystem::LowLevel::InputUpdate_UpdateStepCount;
    pub fn GetUpdateTypeForPlayer(
        mask: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUpdateTypeForPlayer", (mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPlayerUpdate(
        updateType: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPlayerUpdate", (updateType))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnBeforeUpdate(
        _cordl_type: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnBeforeUpdate", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnUpdate(
        _cordl_type: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnUpdate", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn Restore(
        state: crate::UnityEngine::InputSystem::LowLevel::InputUpdate_SerializedState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Restore", (state))?;
        Ok(__cordl_ret.into())
    }
    pub fn Save() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputUpdate_SerializedState,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputUpdate_SerializedState = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Save", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::LowLevel::InputUpdate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+SerializedState")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputUpdate_SerializedState {
    pub lastUpdateType: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    pub playerUpdateStepCount: crate::UnityEngine::InputSystem::LowLevel::InputUpdate_UpdateStepCount,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+SerializedState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::InputUpdate_SerializedState =>
    "UnityEngine.InputSystem.LowLevel"."InputUpdate/SerializedState"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+SerializedState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::InputUpdate_SerializedState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+SerializedState")]
impl crate::UnityEngine::InputSystem::LowLevel::InputUpdate_SerializedState {}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+UpdateStepCount")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputUpdate_UpdateStepCount {
    pub m_WasUpdated: bool,
    pub _value_k__BackingField: u32,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+UpdateStepCount")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::InputUpdate_UpdateStepCount =>
    "UnityEngine.InputSystem.LowLevel"."InputUpdate/UpdateStepCount"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+UpdateStepCount")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::InputUpdate_UpdateStepCount {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+UpdateStepCount")]
impl crate::UnityEngine::InputSystem::LowLevel::InputUpdate_UpdateStepCount {
    pub fn OnBeforeUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OnBeforeUpdate",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn OnUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OnUpdate",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_value(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_value",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_value(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_value",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
