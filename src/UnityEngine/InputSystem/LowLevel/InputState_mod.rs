#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputState")]
#[repr(C)]
#[derive(Debug)]
pub struct InputState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::LowLevel::InputState
    => "UnityEngine.InputSystem.LowLevel"."InputState"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputState")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::LowLevel::InputState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputState")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::LowLevel::InputState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputState")]
impl crate::UnityEngine::InputSystem::LowLevel::InputState {
    #[cfg(
        feature = "UnityEngine+InputSystem+LowLevel+InputState+StateChangeMonitorDelegate"
    )]
    pub type StateChangeMonitorDelegate = crate::UnityEngine::InputSystem::LowLevel::InputState_StateChangeMonitorDelegate;
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputState")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::LowLevel::InputState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputState+StateChangeMonitorDelegate"
)]
#[repr(C)]
#[derive(Debug)]
pub struct InputState_StateChangeMonitorDelegate {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub valueChangeCallback: *mut crate::System::Action_4<
        *mut crate::UnityEngine::InputSystem::InputControl,
        f64,
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        i64,
    >,
    pub timerExpiredCallback: *mut crate::System::Action_4<
        *mut crate::UnityEngine::InputSystem::InputControl,
        f64,
        i64,
        i32,
    >,
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputState+StateChangeMonitorDelegate"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::InputState_StateChangeMonitorDelegate =>
    "UnityEngine.InputSystem.LowLevel"."InputState/StateChangeMonitorDelegate"
);
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputState+StateChangeMonitorDelegate"
)]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::LowLevel::InputState_StateChangeMonitorDelegate {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputState+StateChangeMonitorDelegate"
)]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::LowLevel::InputState_StateChangeMonitorDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputState+StateChangeMonitorDelegate"
)]
impl crate::UnityEngine::InputSystem::LowLevel::InputState_StateChangeMonitorDelegate {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn NotifyControlStateChanged(
        &mut self,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
        _cordl_time: f64,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        monitorIndex: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "NotifyControlStateChanged",
                (control, _cordl_time, eventPtr, monitorIndex),
            )?;
        Ok(__cordl_ret)
    }
    pub fn NotifyTimerExpired(
        &mut self,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
        _cordl_time: f64,
        monitorIndex: i64,
        timerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "NotifyTimerExpired",
                (control, _cordl_time, monitorIndex, timerIndex),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputState+StateChangeMonitorDelegate"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::LowLevel::InputState_StateChangeMonitorDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
