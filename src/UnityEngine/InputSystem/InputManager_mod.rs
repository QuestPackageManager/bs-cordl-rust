#[cfg(feature = "UnityEngine+InputSystem+InputManager")]
#[repr(C)]
#[derive(Debug)]
pub struct InputManager {
    __cordl_parent: crate::System::Object,
    pub m_LayoutRegistrationVersion: i32,
    pub m_PollingFrequency: f32,
    pub m_Layouts: crate::UnityEngine::InputSystem::Layouts::InputControlLayout_Collection,
    pub m_Processors: crate::UnityEngine::InputSystem::Utilities::TypeTable,
    pub m_Interactions: crate::UnityEngine::InputSystem::Utilities::TypeTable,
    pub m_Composites: crate::UnityEngine::InputSystem::Utilities::TypeTable,
    pub m_DevicesCount: i32,
    pub m_Devices: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::InputSystem::InputDevice,
    >,
    pub m_DevicesById: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::UnityEngine::InputSystem::InputDevice,
    >,
    pub m_AvailableDeviceCount: i32,
    pub m_AvailableDevices: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::InputManager_AvailableDevice,
    >,
    pub m_DisconnectedDevicesCount: i32,
    pub m_DisconnectedDevices: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::InputSystem::InputDevice,
    >,
    pub m_UpdateMask: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    pub m_CurrentUpdate: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    pub m_StateBuffers: crate::UnityEngine::InputSystem::LowLevel::InputStateBuffers,
    pub m_DeviceChangeListeners: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        *mut crate::System::Action_2<
            *mut crate::UnityEngine::InputSystem::InputDevice,
            crate::UnityEngine::InputSystem::InputDeviceChange,
        >,
    >,
    pub m_DeviceStateChangeListeners: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        *mut crate::System::Action_2<
            *mut crate::UnityEngine::InputSystem::InputDevice,
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        >,
    >,
    pub m_DeviceFindLayoutCallbacks: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        *mut crate::UnityEngine::InputSystem::Layouts::InputDeviceFindControlLayoutDelegate,
    >,
    pub m_DeviceCommandCallbacks: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        *mut crate::UnityEngine::InputSystem::LowLevel::InputDeviceCommandDelegate,
    >,
    pub m_LayoutChangeListeners: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        *mut crate::System::Action_2<
            *mut crate::System::String,
            crate::UnityEngine::InputSystem::InputControlLayoutChange,
        >,
    >,
    pub m_EventListeners: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        *mut crate::System::Action_2<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            *mut crate::UnityEngine::InputSystem::InputDevice,
        >,
    >,
    pub m_BeforeUpdateListeners: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        *mut crate::System::Action,
    >,
    pub m_AfterUpdateListeners: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        *mut crate::System::Action,
    >,
    pub m_SettingsChangedListeners: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        *mut crate::System::Action,
    >,
    pub m_NativeBeforeUpdateHooked: bool,
    pub m_HaveDevicesWithStateCallbackReceivers: bool,
    pub m_HasFocus: bool,
    pub m_InputEventStream: crate::UnityEngine::InputSystem::LowLevel::InputEventStream,
    pub m_DeviceFindExecuteCommandDelegate: *mut crate::UnityEngine::InputSystem::LowLevel::InputDeviceExecuteCommandDelegate,
    pub m_DeviceFindExecuteCommandDeviceId: i32,
    pub m_Runtime: *mut crate::UnityEngine::InputSystem::LowLevel::IInputRuntime,
    pub m_Metrics: crate::UnityEngine::InputSystem::LowLevel::InputMetrics,
    pub m_Settings: *mut crate::UnityEngine::InputSystem::InputSettings,
    pub m_ShouldMakeCurrentlyUpdatingDeviceCurrent: bool,
    pub m_StateChangeMonitors: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::InputManager_StateChangeMonitorsForDevice,
    >,
    pub m_StateChangeMonitorTimeouts: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
        crate::UnityEngine::InputSystem::InputManager_StateChangeMonitorTimeout,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputManager =>
    "UnityEngine.InputSystem"."InputManager"
);
#[cfg(feature = "UnityEngine+InputSystem+InputManager")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputManager")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputManager")]
impl crate::UnityEngine::InputSystem::InputManager {
    #[cfg(feature = "UnityEngine+InputSystem+InputManager+AvailableDevice")]
    pub type AvailableDevice = crate::UnityEngine::InputSystem::InputManager_AvailableDevice;
    #[cfg(feature = "UnityEngine+InputSystem+InputManager+DeviceDisableScope")]
    pub type DeviceDisableScope = crate::UnityEngine::InputSystem::InputManager_DeviceDisableScope;
    #[cfg(feature = "UnityEngine+InputSystem+InputManager+StateChangeMonitorListener")]
    pub type StateChangeMonitorListener = crate::UnityEngine::InputSystem::InputManager_StateChangeMonitorListener;
    #[cfg(feature = "UnityEngine+InputSystem+InputManager+StateChangeMonitorTimeout")]
    pub type StateChangeMonitorTimeout = crate::UnityEngine::InputSystem::InputManager_StateChangeMonitorTimeout;
    #[cfg(feature = "UnityEngine+InputSystem+InputManager+StateChangeMonitorsForDevice")]
    pub type StateChangeMonitorsForDevice = crate::UnityEngine::InputSystem::InputManager_StateChangeMonitorsForDevice;
    #[cfg(feature = "UnityEngine+InputSystem+InputManager+_ListControlLayouts_d__75")]
    pub type _ListControlLayouts_d__75 = crate::UnityEngine::InputSystem::InputManager__ListControlLayouts_d__75;
    #[cfg(feature = "UnityEngine+InputSystem+InputManager+__c")]
    pub type __c = crate::UnityEngine::InputSystem::InputManager___c;
    #[cfg(feature = "UnityEngine+InputSystem+InputManager+__c__60_1")]
    pub type __c__60_1<TDevice: quest_hook::libil2cpp::Type> = crate::UnityEngine::InputSystem::InputManager___c__60_1<
        TDevice,
    >;
    pub fn AddAvailableDevicesMatchingDescription(
        &mut self,
        matcher: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
        layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAvailableDevicesMatchingDescription", (matcher, layout))?;
        Ok(__cordl_ret)
    }
    pub fn AddAvailableDevicesThatAreNowRecognized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAvailableDevicesThatAreNowRecognized", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddDeviceUsage(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
        usage: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddDeviceUsage", (device, usage))?;
        Ok(__cordl_ret)
    }
    pub fn AddDevice_InputDevice3(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddDevice", (device))?;
        Ok(__cordl_ret)
    }
    pub fn AddDevice_InputDeviceDescription4(
        &mut self,
        description: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputDevice,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputDevice = __cordl_object
            .invoke("AddDevice", (description))?;
        Ok(__cordl_ret)
    }
    pub fn AddDevice_InputDeviceDescription_InternedString_String_i32_InputDevice_DeviceFlags6(
        &mut self,
        description: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
        layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        deviceName: *mut crate::System::String,
        deviceId: i32,
        deviceFlags: crate::UnityEngine::InputSystem::InputDevice_DeviceFlags,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputDevice,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputDevice = __cordl_object
            .invoke(
                "AddDevice",
                (description, layout, deviceName, deviceId, deviceFlags),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddDevice_InputDeviceDescription__cordl_bool_String_i32_InputDevice_DeviceFlags5(
        &mut self,
        description: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
        throwIfNoLayoutFound: bool,
        deviceName: *mut crate::System::String,
        deviceId: i32,
        deviceFlags: crate::UnityEngine::InputSystem::InputDevice_DeviceFlags,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputDevice,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputDevice = __cordl_object
            .invoke(
                "AddDevice",
                (description, throwIfNoLayoutFound, deviceName, deviceId, deviceFlags),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddDevice_InternedString_i32_String_InputDeviceDescription_InputDevice_DeviceFlags_InternedString2(
        &mut self,
        layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        deviceId: i32,
        deviceName: *mut crate::System::String,
        deviceDescription: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
        deviceFlags: crate::UnityEngine::InputSystem::InputDevice_DeviceFlags,
        variants: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputDevice,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputDevice = __cordl_object
            .invoke(
                "AddDevice",
                (layout, deviceId, deviceName, deviceDescription, deviceFlags, variants),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddDevice_String_String_InternedString1(
        &mut self,
        layout: *mut crate::System::String,
        name: *mut crate::System::String,
        variants: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputDevice,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputDevice = __cordl_object
            .invoke("AddDevice", (layout, name, variants))?;
        Ok(__cordl_ret)
    }
    pub fn AddDevice_Type_String0(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputDevice,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputDevice = __cordl_object
            .invoke("AddDevice", (_cordl_type, name))?;
        Ok(__cordl_ret)
    }
    pub fn AddStateChangeMonitor(
        &mut self,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
        monitor: *mut crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
        monitorIndex: i64,
        groupIndex: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddStateChangeMonitor",
                (control, monitor, monitorIndex, groupIndex),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddStateChangeMonitorTimeout(
        &mut self,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
        monitor: *mut crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
        _cordl_time: f64,
        monitorIndex: i64,
        timerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddStateChangeMonitorTimeout",
                (control, monitor, _cordl_time, monitorIndex, timerIndex),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ApplySettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplySettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn AssignUniqueDeviceId(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AssignUniqueDeviceId", (device))?;
        Ok(__cordl_ret)
    }
    pub fn Destroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Destroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn DontMakeCurrentlyUpdatingDeviceCurrent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DontMakeCurrentlyUpdatingDeviceCurrent", ())?;
        Ok(__cordl_ret)
    }
    pub fn EnableOrDisableDevice(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
        enable: bool,
        scope: crate::UnityEngine::InputSystem::InputManager_DeviceDisableScope,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnableOrDisableDevice", (device, enable, scope))?;
        Ok(__cordl_ret)
    }
    pub fn ExecuteGlobalCommand<TCommand>(
        &mut self,
        command: quest_hook::libil2cpp::ByRefMut<TCommand>,
    ) -> quest_hook::libil2cpp::Result<i64>
    where
        TCommand: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("ExecuteGlobalCommand", (command))?;
        Ok(__cordl_ret)
    }
    pub fn FindOrRegisterDeviceLayoutForType(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString = __cordl_object
            .invoke("FindOrRegisterDeviceLayoutForType", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn FireStateChangeNotifications_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FireStateChangeNotifications", ())?;
        Ok(__cordl_ret)
    }
    pub fn FireStateChangeNotifications_i32_f64_Il2CppObject1(
        &mut self,
        deviceIndex: i32,
        internalTime: f64,
        eventPtr: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "FireStateChangeNotifications",
                (deviceIndex, internalTime, eventPtr),
            )?;
        Ok(__cordl_ret)
    }
    pub fn FlipBuffersForDeviceIfNecessary(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
        updateType: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("FlipBuffersForDeviceIfNecessary", (device, updateType))?;
        Ok(__cordl_ret)
    }
    pub fn FlushDisconnectedDevices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FlushDisconnectedDevices", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetControls<TControl>(
        &mut self,
        path: *mut crate::System::String,
        controls: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputControlList_1<TControl>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TControl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetControls", (path, controls))?;
        Ok(__cordl_ret)
    }
    pub fn GetDevice(
        &mut self,
        nameOrLayout: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputDevice,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputDevice = __cordl_object
            .invoke("GetDevice", (nameOrLayout))?;
        Ok(__cordl_ret)
    }
    pub fn GetUnsupportedDevices(
        &mut self,
        descriptions: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetUnsupportedDevices", (descriptions))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
        runtime: *mut crate::UnityEngine::InputSystem::LowLevel::IInputRuntime,
        settings: *mut crate::UnityEngine::InputSystem::InputSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", (runtime, settings))?;
        Ok(__cordl_ret)
    }
    pub fn InitializeData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeData", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitializeDefaultState(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeDefaultState", (device))?;
        Ok(__cordl_ret)
    }
    pub fn InitializeDeviceState(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeDeviceState", (device))?;
        Ok(__cordl_ret)
    }
    pub fn InstallBeforeUpdateHookIfNecessary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBeforeUpdateHookIfNecessary", ())?;
        Ok(__cordl_ret)
    }
    pub fn InstallGlobals(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallGlobals", ())?;
        Ok(__cordl_ret)
    }
    pub fn InstallRuntime(
        &mut self,
        runtime: *mut crate::UnityEngine::InputSystem::LowLevel::IInputRuntime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallRuntime", (runtime))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeAfterUpdateCallback(
        &mut self,
        updateType: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeAfterUpdateCallback", (updateType))?;
        Ok(__cordl_ret)
    }
    pub fn IsControlOrChildUsingLayoutRecursive(
        &mut self,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
        layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsControlOrChildUsingLayoutRecursive", (control, layout))?;
        Ok(__cordl_ret)
    }
    pub fn IsControlUsingLayout(
        &mut self,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
        layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsControlUsingLayout", (control, layout))?;
        Ok(__cordl_ret)
    }
    pub fn IsDeviceLayoutMarkedAsSupportedInSettings(
        &mut self,
        layoutName: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsDeviceLayoutMarkedAsSupportedInSettings", (layoutName))?;
        Ok(__cordl_ret)
    }
    pub fn ListControlLayouts(
        &mut self,
        basedOn: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("ListControlLayouts", (basedOn))?;
        Ok(__cordl_ret)
    }
    pub fn MakeDeviceNameUnique(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MakeDeviceNameUnique", (device))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn NotifyUsageChanged(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyUsageChanged", (device))?;
        Ok(__cordl_ret)
    }
    pub fn OnBeforeUpdate(
        &mut self,
        updateType: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBeforeUpdate", (updateType))?;
        Ok(__cordl_ret)
    }
    pub fn OnFocusChanged(
        &mut self,
        focus: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnFocusChanged", (focus))?;
        Ok(__cordl_ret)
    }
    pub fn OnNativeDeviceDiscovered(
        &mut self,
        deviceId: i32,
        deviceDescriptor: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNativeDeviceDiscovered", (deviceId, deviceDescriptor))?;
        Ok(__cordl_ret)
    }
    pub fn OnUpdate(
        &mut self,
        updateType: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
        eventBuffer: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::LowLevel::InputEventBuffer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnUpdate", (updateType, eventBuffer))?;
        Ok(__cordl_ret)
    }
    pub fn PerformLayoutPostRegistration(
        &mut self,
        layoutName: crate::UnityEngine::InputSystem::Utilities::InternedString,
        baseLayouts: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        >,
        isReplacement: bool,
        isKnownToBeDeviceLayout: bool,
        isOverride: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "PerformLayoutPostRegistration",
                (
                    layoutName,
                    baseLayouts,
                    isReplacement,
                    isKnownToBeDeviceLayout,
                    isOverride,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ProcessStateChangeMonitorTimeouts(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessStateChangeMonitorTimeouts", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessStateChangeMonitors(
        &mut self,
        deviceIndex: i32,
        newStateFromEvent: *mut quest_hook::libil2cpp::Il2CppObject,
        oldStateOfDevice: *mut quest_hook::libil2cpp::Il2CppObject,
        newStateSizeInBytes: u32,
        newStateOffsetInBytes: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ProcessStateChangeMonitors",
                (
                    deviceIndex,
                    newStateFromEvent,
                    oldStateOfDevice,
                    newStateSizeInBytes,
                    newStateOffsetInBytes,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn QueueEvent_ByRefMut2<TEvent>(
        &mut self,
        inputEvent: quest_hook::libil2cpp::ByRefMut<TEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TEvent: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("QueueEvent", (inputEvent))?;
        Ok(__cordl_ret)
    }
    pub fn QueueEvent_Il2CppObject0(
        &mut self,
        eventPtr: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("QueueEvent", (eventPtr))?;
        Ok(__cordl_ret)
    }
    pub fn QueueEvent_InputEventPtr1(
        &mut self,
        ptr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("QueueEvent", (ptr))?;
        Ok(__cordl_ret)
    }
    pub fn ReallocateStateBuffers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReallocateStateBuffers", ())?;
        Ok(__cordl_ret)
    }
    pub fn RecreateDevice(
        &mut self,
        oldDevice: *mut crate::UnityEngine::InputSystem::InputDevice,
        newLayout: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecreateDevice", (oldDevice, newLayout))?;
        Ok(__cordl_ret)
    }
    pub fn RecreateDevicesUsingLayout(
        &mut self,
        layout: crate::UnityEngine::InputSystem::Utilities::InternedString,
        isKnownToBeDeviceLayout: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecreateDevicesUsingLayout", (layout, isKnownToBeDeviceLayout))?;
        Ok(__cordl_ret)
    }
    pub fn RecreateDevicesUsingLayoutWithInferiorMatch(
        &mut self,
        deviceMatcher: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecreateDevicesUsingLayoutWithInferiorMatch", (deviceMatcher))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterControlLayoutBuilder(
        &mut self,
        method: *mut crate::System::Func_1<
            *mut crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
        >,
        name: *mut crate::System::String,
        baseLayout: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterControlLayoutBuilder", (method, name, baseLayout))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterControlLayoutMatcher_String0(
        &mut self,
        layoutName: *mut crate::System::String,
        matcher: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterControlLayoutMatcher", (layoutName, matcher))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterControlLayoutMatcher_Type1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        matcher: crate::UnityEngine::InputSystem::Layouts::InputDeviceMatcher,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterControlLayoutMatcher", (_cordl_type, matcher))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterControlLayout_String__cordl_bool1(
        &mut self,
        json: *mut crate::System::String,
        name: *mut crate::System::String,
        isOverride: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterControlLayout", (json, name, isOverride))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterControlLayout_Type0(
        &mut self,
        name: *mut crate::System::String,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterControlLayout", (name, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterPrecompiledLayout<TDevice>(
        &mut self,
        metadata: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TDevice: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterPrecompiledLayout", (metadata))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveControlLayout(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveControlLayout", (name))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveDevice(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
        keepOnListOfAvailableDevices: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveDevice", (device, keepOnListOfAvailableDevices))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveDeviceUsage(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
        usage: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveDeviceUsage", (device, usage))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveStateChangeMonitor(
        &mut self,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
        monitor: *mut crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
        monitorIndex: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveStateChangeMonitor", (control, monitor, monitorIndex))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveStateChangeMonitorTimeout(
        &mut self,
        monitor: *mut crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
        monitorIndex: i64,
        timerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RemoveStateChangeMonitorTimeout",
                (monitor, monitorIndex, timerIndex),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RemoveStateChangeMonitors(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveStateChangeMonitors", (device))?;
        Ok(__cordl_ret)
    }
    pub fn ResetDevice(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
        alsoResetDontResetControls: bool,
        issueResetCommand: crate::System::Nullable_1<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ResetDevice",
                (device, alsoResetDontResetControls, issueResetCommand),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RestoreDevicesAfterDomainReloadIfNecessary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RestoreDevicesAfterDomainReloadIfNecessary", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetDeviceUsage(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
        usage: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDeviceUsage", (device, usage))?;
        Ok(__cordl_ret)
    }
    pub fn ShouldRunDeviceInBackground(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldRunDeviceInBackground", (device))?;
        Ok(__cordl_ret)
    }
    pub fn ShouldRunUpdate(
        &mut self,
        updateType: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ShouldRunUpdate", (updateType))?;
        Ok(__cordl_ret)
    }
    pub fn SignalStateChangeMonitor(
        &mut self,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
        monitor: *mut crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SignalStateChangeMonitor", (control, monitor))?;
        Ok(__cordl_ret)
    }
    pub fn SortStateChangeMonitorsIfNecessary(
        &mut self,
        deviceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortStateChangeMonitorsIfNecessary", (deviceIndex))?;
        Ok(__cordl_ret)
    }
    pub fn TryFindMatchingControlLayout(
        &mut self,
        deviceDescription: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
        >,
        deviceId: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::InternedString = __cordl_object
            .invoke("TryFindMatchingControlLayout", (deviceDescription, deviceId))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetDeviceById(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputDevice,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputDevice = __cordl_object
            .invoke("TryGetDeviceById", (id))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetDevice_String0(
        &mut self,
        nameOrLayout: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputDevice,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputDevice = __cordl_object
            .invoke("TryGetDevice", (nameOrLayout))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetDevice_Type1(
        &mut self,
        layoutType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputDevice,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputDevice = __cordl_object
            .invoke("TryGetDevice", (layoutType))?;
        Ok(__cordl_ret)
    }
    pub fn TryLoadControlLayout_InternedString1(
        &mut self,
        name: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Layouts::InputControlLayout = __cordl_object
            .invoke("TryLoadControlLayout", (name))?;
        Ok(__cordl_ret)
    }
    pub fn TryLoadControlLayout_Type0(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Layouts::InputControlLayout,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Layouts::InputControlLayout = __cordl_object
            .invoke("TryLoadControlLayout", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn TryMatchDisconnectedDevice(
        &mut self,
        deviceDescriptor: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputDevice,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputDevice = __cordl_object
            .invoke("TryMatchDisconnectedDevice", (deviceDescriptor))?;
        Ok(__cordl_ret)
    }
    pub fn UninstallGlobals(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UninstallGlobals", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateState_Il2CppObject_InputUpdateType0(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
        eventPtr: *mut quest_hook::libil2cpp::Il2CppObject,
        updateType: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UpdateState", (device, eventPtr, updateType))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateState_InputUpdateType_Il2CppObject_u32_u32_f64_InputEventPtr1(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
        updateType: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
        statePtr: *mut quest_hook::libil2cpp::Il2CppObject,
        stateOffsetInDevice: u32,
        stateSize: u32,
        internalTime: f64,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UpdateState",
                (
                    device,
                    updateType,
                    statePtr,
                    stateOffsetInDevice,
                    stateSize,
                    internalTime,
                    eventPtr,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Update_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update_InputUpdateType1(
        &mut self,
        updateType: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (updateType))?;
        Ok(__cordl_ret)
    }
    pub fn WarnAboutDevicesFailingToRecreateAfterDomainReload(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WarnAboutDevicesFailingToRecreateAfterDomainReload", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteStateChange(
        &mut self,
        buffers: crate::UnityEngine::InputSystem::LowLevel::InputStateBuffers_DoubleBuffers,
        deviceIndex: i32,
        deviceStateBlock: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::LowLevel::InputStateBlock,
        >,
        stateOffsetInDevice: u32,
        statePtr: *mut quest_hook::libil2cpp::Il2CppObject,
        stateSizeInBytes: u32,
        flippedBuffers: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteStateChange",
                (
                    buffers,
                    deviceIndex,
                    deviceStateBlock,
                    stateOffsetInDevice,
                    statePtr,
                    stateSizeInBytes,
                    flippedBuffers,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _TryFindMatchingControlLayout_b__72_0(
        &mut self,
        commandRef: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::LowLevel::InputDeviceCommand,
        >,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("<TryFindMatchingControlLayout>b__72_0", (commandRef))?;
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
    pub fn add_onAfterUpdate(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onAfterUpdate", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_onBeforeUpdate(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onBeforeUpdate", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_onDeviceChange(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::UnityEngine::InputSystem::InputDevice,
            crate::UnityEngine::InputSystem::InputDeviceChange,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onDeviceChange", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_onDeviceCommand(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::LowLevel::InputDeviceCommandDelegate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onDeviceCommand", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_onDeviceStateChange(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::UnityEngine::InputSystem::InputDevice,
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onDeviceStateChange", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_onEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            *mut crate::UnityEngine::InputSystem::InputDevice,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_onFindControlLayoutForDevice(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Layouts::InputDeviceFindControlLayoutDelegate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onFindControlLayoutForDevice", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_onLayoutChange(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            crate::UnityEngine::InputSystem::InputControlLayoutChange,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onLayoutChange", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_onSettingsChange(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onSettingsChange", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_composites(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::TypeTable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::TypeTable = __cordl_object
            .invoke("get_composites", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_defaultUpdateType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType = __cordl_object
            .invoke("get_defaultUpdateType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_devices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            *mut crate::UnityEngine::InputSystem::InputDevice,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            *mut crate::UnityEngine::InputSystem::InputDevice,
        > = __cordl_object.invoke("get_devices", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_gameHasFocus(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_gameHasFocus", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_gameIsPlaying(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_gameIsPlaying", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_gameShouldGetInputRegardlessOfFocus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_gameShouldGetInputRegardlessOfFocus", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_interactions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::TypeTable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::TypeTable = __cordl_object
            .invoke("get_interactions", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isProcessingEvents(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isProcessingEvents", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_metrics(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputMetrics,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputMetrics = __cordl_object
            .invoke("get_metrics", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_pollingFrequency(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_pollingFrequency", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_processors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::TypeTable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::TypeTable = __cordl_object
            .invoke("get_processors", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_settings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputSettings,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputSettings = __cordl_object
            .invoke("get_settings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_updateMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType = __cordl_object
            .invoke("get_updateMask", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_onAfterUpdate(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onAfterUpdate", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_onBeforeUpdate(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onBeforeUpdate", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_onDeviceChange(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::UnityEngine::InputSystem::InputDevice,
            crate::UnityEngine::InputSystem::InputDeviceChange,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onDeviceChange", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_onDeviceCommand(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::LowLevel::InputDeviceCommandDelegate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onDeviceCommand", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_onDeviceStateChange(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::UnityEngine::InputSystem::InputDevice,
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onDeviceStateChange", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_onEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            *mut crate::UnityEngine::InputSystem::InputDevice,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_onFindControlLayoutForDevice(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Layouts::InputDeviceFindControlLayoutDelegate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onFindControlLayoutForDevice", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_onLayoutChange(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            crate::UnityEngine::InputSystem::InputControlLayoutChange,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onLayoutChange", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_onSettingsChange(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onSettingsChange", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_pollingFrequency(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_pollingFrequency", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_settings(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::InputSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_settings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_updateMask(
        &mut self,
        value: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_updateMask", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputManager+AvailableDevice")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputManager_AvailableDevice {
    pub description: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
    pub deviceId: i32,
    pub isNative: bool,
    pub isRemoved: bool,
}
#[cfg(feature = "UnityEngine+InputSystem+InputManager+AvailableDevice")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputManager_AvailableDevice => "UnityEngine.InputSystem"
    ."InputManager/AvailableDevice"
);
#[cfg(feature = "UnityEngine+InputSystem+InputManager+AvailableDevice")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputManager_AvailableDevice {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputManager+AvailableDevice")]
impl crate::UnityEngine::InputSystem::InputManager_AvailableDevice {}
#[cfg(feature = "UnityEngine+InputSystem+InputManager+DeviceDisableScope")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputManager_DeviceDisableScope {
    Everywhere = 0i32,
    InFrontendOnly = 1i32,
    TemporaryWhilePlayerIsInBackground = 2i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputManager+DeviceDisableScope")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputManager_DeviceDisableScope =>
    "UnityEngine.InputSystem"."InputManager/DeviceDisableScope"
);
#[cfg(feature = "UnityEngine+InputSystem+InputManager+StateChangeMonitorListener")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputManager_StateChangeMonitorListener {
    pub control: *mut crate::UnityEngine::InputSystem::InputControl,
    pub monitor: *mut crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
    pub monitorIndex: i64,
    pub groupIndex: u32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputManager+StateChangeMonitorListener")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputManager_StateChangeMonitorListener =>
    "UnityEngine.InputSystem"."InputManager/StateChangeMonitorListener"
);
#[cfg(feature = "UnityEngine+InputSystem+InputManager+StateChangeMonitorListener")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputManager_StateChangeMonitorListener {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputManager+StateChangeMonitorListener")]
impl crate::UnityEngine::InputSystem::InputManager_StateChangeMonitorListener {}
#[cfg(feature = "UnityEngine+InputSystem+InputManager+StateChangeMonitorTimeout")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputManager_StateChangeMonitorTimeout {
    pub control: *mut crate::UnityEngine::InputSystem::InputControl,
    pub _cordl_time: f64,
    pub monitor: *mut crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
    pub monitorIndex: i64,
    pub timerIndex: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputManager+StateChangeMonitorTimeout")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputManager_StateChangeMonitorTimeout =>
    "UnityEngine.InputSystem"."InputManager/StateChangeMonitorTimeout"
);
#[cfg(feature = "UnityEngine+InputSystem+InputManager+StateChangeMonitorTimeout")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputManager_StateChangeMonitorTimeout {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputManager+StateChangeMonitorTimeout")]
impl crate::UnityEngine::InputSystem::InputManager_StateChangeMonitorTimeout {}
#[cfg(feature = "UnityEngine+InputSystem+InputManager+StateChangeMonitorsForDevice")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputManager_StateChangeMonitorsForDevice {
    pub memoryRegions: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::Utilities::MemoryHelpers_BitRegion,
    >,
    pub listeners: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::InputManager_StateChangeMonitorListener,
    >,
    pub signalled: crate::UnityEngine::InputSystem::DynamicBitfield,
    pub needToUpdateOrderingOfMonitors: bool,
    pub needToCompactArrays: bool,
}
#[cfg(feature = "UnityEngine+InputSystem+InputManager+StateChangeMonitorsForDevice")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputManager_StateChangeMonitorsForDevice =>
    "UnityEngine.InputSystem"."InputManager/StateChangeMonitorsForDevice"
);
#[cfg(feature = "UnityEngine+InputSystem+InputManager+StateChangeMonitorsForDevice")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputManager_StateChangeMonitorsForDevice {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputManager+StateChangeMonitorsForDevice")]
impl crate::UnityEngine::InputSystem::InputManager_StateChangeMonitorsForDevice {
    pub fn Add(
        &mut self,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
        monitor: *mut crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
        monitorIndex: i64,
        groupIndex: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Add",
            (control, monitor, monitorIndex, groupIndex),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clear",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CompactArrays(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompactArrays",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Remove(
        &mut self,
        monitor: *mut crate::UnityEngine::InputSystem::LowLevel::IInputStateChangeMonitor,
        monitorIndex: i64,
        deferRemoval: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Remove",
            (monitor, monitorIndex, deferRemoval),
        )?;
        Ok(__cordl_ret)
    }
    pub fn RemoveAt(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RemoveAt",
            (i),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SortMonitorsByIndex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SortMonitorsByIndex",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_count",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
