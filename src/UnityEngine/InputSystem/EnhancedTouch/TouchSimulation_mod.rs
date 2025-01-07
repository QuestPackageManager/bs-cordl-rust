#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchSimulation")]
#[repr(C)]
#[derive(Debug)]
pub struct TouchSimulation {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _simulatedTouchscreen_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Touchscreen,
    >,
    pub m_NumPointers: i32,
    pub m_Pointers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Pointer>,
        >,
    >,
    pub m_CurrentPositions: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
    >,
    pub m_CurrentDisplayIndices: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub m_Touches: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::InputSystem::Controls::ButtonControl,
            >,
        >,
    >,
    pub m_LastTouchId: i32,
    pub m_PrimaryTouchIndex: i32,
    pub m_OnDeviceChange: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
            crate::UnityEngine::InputSystem::InputDeviceChange,
        >,
    >,
    pub m_OnEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        >,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchSimulation")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::EnhancedTouch::TouchSimulation {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.EnhancedTouch";
    const CLASS_NAME: &'static str = "TouchSimulation";
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
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchSimulation")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::EnhancedTouch::TouchSimulation {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchSimulation")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::EnhancedTouch::TouchSimulation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchSimulation")]
impl crate::UnityEngine::InputSystem::EnhancedTouch::TouchSimulation {
    pub fn AddPointer(
        &mut self,
        pointer: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Pointer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddPointer", (pointer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Destroy() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Destroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Disable() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Disable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Enable() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Enable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InstallStateChangeMonitors(
        &mut self,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallStateChangeMonitors", (startIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDeviceChange(
        &mut self,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        change: crate::UnityEngine::InputSystem::InputDeviceChange,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDeviceChange", (device, change))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEvent(
        &mut self,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEvent", (eventPtr, device))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSourceControlChangedValue(
        &mut self,
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        _cordl_time: f64,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        sourceDeviceAndButtonIndex: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OnSourceControlChangedValue",
                (control, _cordl_time, eventPtr, sourceDeviceAndButtonIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RemovePointer(
        &mut self,
        pointer: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Pointer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemovePointer", (pointer))?;
        Ok(__cordl_ret.into())
    }
    pub fn UninstallStateChangeMonitors(
        &mut self,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UninstallStateChangeMonitors", (startIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_InputSystem_LowLevel_IInputStateChangeMonitor_NotifyControlStateChanged(
        &mut self,
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        _cordl_time: f64,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        monitorIndex: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.InputSystem.LowLevel.IInputStateChangeMonitor.NotifyControlStateChanged",
                (control, _cordl_time, eventPtr, monitorIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_InputSystem_LowLevel_IInputStateChangeMonitor_NotifyTimerExpired(
        &mut self,
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        _cordl_time: f64,
        monitorIndex: i64,
        timerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.InputSystem.LowLevel.IInputStateChangeMonitor.NotifyTimerExpired",
                (control, _cordl_time, monitorIndex, timerIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateTouch(
        &mut self,
        touchIndex: i32,
        pointerIndex: i32,
        phase: crate::UnityEngine::InputSystem::TouchPhase,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateTouch", (touchIndex, pointerIndex, phase, eventPtr))?;
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
    pub fn get_instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::EnhancedTouch::TouchSimulation,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::EnhancedTouch::TouchSimulation,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_instance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_simulatedTouchscreen(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Touchscreen>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Touchscreen,
        > = __cordl_object.invoke("get_simulatedTouchscreen", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_simulatedTouchscreen(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Touchscreen>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_simulatedTouchscreen", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchSimulation")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::EnhancedTouch::TouchSimulation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchSimulation")]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor>
for crate::UnityEngine::InputSystem::EnhancedTouch::TouchSimulation {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+TouchSimulation")]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor>
for crate::UnityEngine::InputSystem::EnhancedTouch::TouchSimulation {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor {
        unsafe { std::mem::transmute(self) }
    }
}
