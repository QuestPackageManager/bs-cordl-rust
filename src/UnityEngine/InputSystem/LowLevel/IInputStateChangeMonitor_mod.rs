#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IInputStateChangeMonitor")]
#[repr(C)]
#[derive(Debug)]
pub struct IInputStateChangeMonitor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IInputStateChangeMonitor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor =>
    "UnityEngine.InputSystem.LowLevel"."IInputStateChangeMonitor"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IInputStateChangeMonitor")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IInputStateChangeMonitor")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IInputStateChangeMonitor")]
impl crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor {
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
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IInputStateChangeMonitor")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}