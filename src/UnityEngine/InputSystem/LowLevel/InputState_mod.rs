#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputState")]
#[repr(C)]
#[derive(Debug)]
pub struct InputState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::LowLevel::InputState {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.LowLevel";
    const CLASS_NAME: &'static str = "InputState";
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
    pub fn AddChangeMonitorTimeout(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        monitor: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
        >,
        _cordl_time: f64,
        monitorIndex: i64,
        timerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AddChangeMonitorTimeout",
                (control, monitor, _cordl_time, monitorIndex, timerIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddChangeMonitor_Action_4_i32_Action_4_1(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        valueChangeCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
                f64,
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                i64,
            >,
        >,
        monitorIndex: i32,
        timerExpiredCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
                f64,
                i64,
                i32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AddChangeMonitor",
                (control, valueChangeCallback, monitorIndex, timerExpiredCallback),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddChangeMonitor_IInputStateChangeMonitor_i64_u32_0(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        monitor: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
        >,
        monitorIndex: i64,
        groupIndex: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddChangeMonitor", (control, monitor, monitorIndex, groupIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn Change_InputControl_ByRefMut_InputEventPtr2<TState>(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        state: quest_hook::libil2cpp::ByRefMut<TState>,
        updateType: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TState: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Change", (control, state, updateType, eventPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn Change_InputControl_TState_InputEventPtr1<TState>(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        state: TState,
        updateType: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TState: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Change", (control, state, updateType, eventPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn Change_InputDevice_InputEventPtr0(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        updateType: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Change", (device, eventPtr, updateType))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsIntegerFormat(
        format: crate::UnityEngine::InputSystem::Utilities::FourCC,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsIntegerFormat", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveChangeMonitor(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        monitor: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
        >,
        monitorIndex: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveChangeMonitor", (control, monitor, monitorIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveChangeMonitorTimeout(
        monitor: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
        >,
        monitorIndex: i64,
        timerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveChangeMonitorTimeout", (monitor, monitorIndex, timerIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onChange(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_onChange", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentTime() -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_currentTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentUpdateType() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_currentUpdateType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_updateCount() -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_updateCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onChange(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_onChange", (value))?;
        Ok(__cordl_ret.into())
    }
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
    pub valueChangeCallback: quest_hook::libil2cpp::Gc<
        crate::System::Action_4<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
            f64,
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            i64,
        >,
    >,
    pub timerExpiredCallback: quest_hook::libil2cpp::Gc<
        crate::System::Action_4<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
            f64,
            i64,
            i32,
        >,
    >,
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputState+StateChangeMonitorDelegate"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::LowLevel::InputState_StateChangeMonitorDelegate {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.LowLevel";
    const CLASS_NAME: &'static str = "StateChangeMonitorDelegate";
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NotifyControlStateChanged(
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
                "NotifyControlStateChanged",
                (control, _cordl_time, eventPtr, monitorIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn NotifyTimerExpired(
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
                "NotifyTimerExpired",
                (control, _cordl_time, monitorIndex, timerIndex),
            )?;
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
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputState+StateChangeMonitorDelegate"
)]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor>
for crate::UnityEngine::InputSystem::LowLevel::InputState_StateChangeMonitorDelegate {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InputState+StateChangeMonitorDelegate"
)]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor>
for crate::UnityEngine::InputSystem::LowLevel::InputState_StateChangeMonitorDelegate {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor {
        unsafe { std::mem::transmute(self) }
    }
}
