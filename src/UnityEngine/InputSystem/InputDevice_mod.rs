#[cfg(feature = "UnityEngine+InputSystem+InputDevice+ControlBitRangeNode")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputDevice_ControlBitRangeNode {
    pub endBitOffset: u16,
    pub leftChildIndex: i16,
    pub controlStartIndex: u16,
    pub controlCount: u8,
}
#[cfg(feature = "UnityEngine+InputSystem+InputDevice+ControlBitRangeNode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode =>
    "UnityEngine.InputSystem"."InputDevice/ControlBitRangeNode"
);
#[cfg(feature = "UnityEngine+InputSystem+InputDevice+ControlBitRangeNode")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputDevice+ControlBitRangeNode")]
impl crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode {
    pub fn _ctor(
        &mut self,
        endOffset: u16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (endOffset),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputDevice+DeviceFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputDevice_DeviceFlags {
    CanRunInBackground = 2048i32,
    CanRunInBackgroundHasBeenQueried = 4096i32,
    DisabledInFrontend = 32i32,
    DisabledInRuntime = 128i32,
    DisabledStateHasBeenQueriedFromRuntime = 64i32,
    DisabledWhileInBackground = 256i32,
    HasControlsWithDefaultState = 4i32,
    HasDontResetControls = 1024i32,
    HasEventMerger = 8192i32,
    HasEventPreProcessor = 16384i32,
    HasStateCallbacks = 2i32,
    Native = 16i32,
    Remote = 8i32,
    UpdateBeforeRender = 1i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputDevice+DeviceFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputDevice_DeviceFlags => "UnityEngine.InputSystem"
    ."InputDevice/DeviceFlags"
);
#[cfg(feature = "UnityEngine+InputSystem+InputDevice")]
#[repr(C)]
#[derive(Debug)]
pub struct InputDevice {
    __cordl_parent: crate::UnityEngine::InputSystem::InputControl,
    pub m_DeviceFlags: crate::UnityEngine::InputSystem::InputDevice_DeviceFlags,
    pub m_DeviceId: i32,
    pub m_ParticipantId: i32,
    pub m_DeviceIndex: i32,
    pub m_Description: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
    pub m_LastUpdateTimeInternal: f64,
    pub m_CurrentUpdateStepCount: u32,
    pub m_AliasesForEachControl: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    >,
    pub m_UsagesForEachControl: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    >,
    pub m_UsageToControl: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::InputSystem::InputControl,
    >,
    pub m_ChildrenForEachControl: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::InputSystem::InputControl,
    >,
    pub m_StateOffsetToControlMap: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    pub m_ControlTreeNodes: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode,
    >,
    pub m_ControlTreeIndices: *mut quest_hook::libil2cpp::Il2CppArray<u16>,
}
#[cfg(feature = "UnityEngine+InputSystem+InputDevice")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputDevice =>
    "UnityEngine.InputSystem"."InputDevice"
);
#[cfg(feature = "UnityEngine+InputSystem+InputDevice")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputDevice {
    type Target = crate::UnityEngine::InputSystem::InputControl;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputDevice")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputDevice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputDevice")]
impl crate::UnityEngine::InputSystem::InputDevice {
    pub const InvalidDeviceId: i32 = 0i32;
    pub const kControlIndexBits: i32 = 10i32;
    pub const kInvalidDeviceIndex: i32 = -1i32;
    pub const kLocalParticipantId: i32 = 0i32;
    pub const kStateOffsetBits: i32 = 13i32;
    pub const kStateSizeBits: i32 = 9i32;
    #[cfg(feature = "UnityEngine+InputSystem+InputDevice+DeviceFlags")]
    pub type DeviceFlags = crate::UnityEngine::InputSystem::InputDevice_DeviceFlags;
    #[cfg(feature = "UnityEngine+InputSystem+InputDevice+ControlBitRangeNode")]
    pub type ControlBitRangeNode = crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode;
    pub fn OnRemoved(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRemoved", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_enabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn NotifyConfigurationChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyConfigurationChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn CompareValue(
        &mut self,
        firstStatePtr: *mut quest_hook::libil2cpp::Il2CppObject,
        secondStatePtr: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CompareValue", (firstStatePtr, secondStatePtr))?;
        Ok(__cordl_ret)
    }
    pub fn RequestSync(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RequestSync", ())?;
        Ok(__cordl_ret)
    }
    pub fn NotifyRemoved(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyRemoved", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteChangedControlStates(
        &mut self,
        deviceStateBuffer: *mut quest_hook::libil2cpp::Il2CppObject,
        statePtr: *mut quest_hook::libil2cpp::Il2CppObject,
        stateSizeInBytes: u32,
        stateOffsetInDevice: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteChangedControlStates",
                (deviceStateBuffer, statePtr, stateSizeInBytes, stateOffsetInDevice),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RequestReset(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RequestReset", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadValueFromStateIntoBuffer(
        &mut self,
        statePtr: *mut quest_hook::libil2cpp::Il2CppObject,
        bufferPtr: *mut quest_hook::libil2cpp::Il2CppObject,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadValueFromStateIntoBuffer", (statePtr, bufferPtr, bufferSize))?;
        Ok(__cordl_ret)
    }
    pub fn get_description(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription = __cordl_object
            .invoke("get_description", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadValueFromStateAsObject(
        &mut self,
        statePtr: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ReadValueFromStateAsObject", (statePtr))?;
        Ok(__cordl_ret)
    }
    pub fn get_added(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_added", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_disabledInFrontend(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_disabledInFrontend", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_hasEventMerger(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hasEventMerger", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_hasEventPreProcessor(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hasEventPreProcessor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_canRunInBackground(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_canRunInBackground", ())?;
        Ok(__cordl_ret)
    }
    pub fn ExecuteCommand_ByRefMut0<TCommand>(
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
        let __cordl_ret: i64 = __cordl_object.invoke("ExecuteCommand", (command))?;
        Ok(__cordl_ret)
    }
    pub fn ExecuteCommand_Il2CppObject1(
        &mut self,
        commandPtr: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("ExecuteCommand", (commandPtr))?;
        Ok(__cordl_ret)
    }
    pub fn ExecuteDisableCommand(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ExecuteDisableCommand", ())?;
        Ok(__cordl_ret)
    }
    pub fn ExecuteEnableCommand(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ExecuteEnableCommand", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_hasDontResetControls(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hasDontResetControls", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_disabledInRuntime(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_disabledInRuntime", ())?;
        Ok(__cordl_ret)
    }
    pub fn QueryEnabledStateFromRuntime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("QueryEnabledStateFromRuntime", ())?;
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
    pub fn get_hasDontResetControls(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasDontResetControls", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasStateCallbacks(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasStateCallbacks", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_remote(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_remote", ())?;
        Ok(__cordl_ret)
    }
    pub fn RemoveDeviceUsage(
        &mut self,
        usage: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveDeviceUsage", (usage))?;
        Ok(__cordl_ret)
    }
    pub fn NotifyAdded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyAdded", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_disabledWhileInBackground(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_disabledWhileInBackground", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_deviceId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_deviceId", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_hasStateCallbacks(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hasStateCallbacks", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_wasUpdatedThisFrame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_wasUpdatedThisFrame", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_valueType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_valueType", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnAdded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAdded", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_disabledInFrontend(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_disabledInFrontend", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_disabledInRuntime(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_disabledInRuntime", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_lastUpdateTime(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_lastUpdateTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_disabledWhileInBackground(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_disabledWhileInBackground", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_hasControlsWithDefaultState(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hasControlsWithDefaultState", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_hasEventMerger(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasEventMerger", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnConfigurationChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnConfigurationChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_allControls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            *mut crate::UnityEngine::InputSystem::InputControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            *mut crate::UnityEngine::InputSystem::InputControl,
        > = __cordl_object.invoke("get_allControls", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasEventPreProcessor(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasEventPreProcessor", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddDeviceUsage(
        &mut self,
        usage: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddDeviceUsage", (usage))?;
        Ok(__cordl_ret)
    }
    pub fn ClearDeviceUsages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearDeviceUsages", ())?;
        Ok(__cordl_ret)
    }
    pub fn WritePartialChangedControlStatesInternal(
        &mut self,
        statePtr: *mut quest_hook::libil2cpp::Il2CppObject,
        stateSizeInBits: u32,
        stateOffsetInDeviceInBits: u32,
        deviceStatePtr: *mut quest_hook::libil2cpp::Il2CppObject,
        parentNode: crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode,
        startOffset: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WritePartialChangedControlStatesInternal",
                (
                    statePtr,
                    stateSizeInBits,
                    stateOffsetInDeviceInBits,
                    deviceStatePtr,
                    parentNode,
                    startOffset,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_hasControlsWithDefaultState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_hasControlsWithDefaultState", ())?;
        Ok(__cordl_ret)
    }
    pub fn DumpControlTree_InputDevice_ControlBitRangeNode_u32_List_1_0(
        &mut self,
        parentNode: crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode,
        startOffset: u32,
        output: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DumpControlTree", (parentNode, startOffset, output))?;
        Ok(__cordl_ret)
    }
    pub fn DumpControlTree_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("DumpControlTree", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_native(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_native", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_updateBeforeRender(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_updateBeforeRender", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_valueSizeInBytes(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_valueSizeInBytes", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadValueFromBufferAsObject(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppObject,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ReadValueFromBufferAsObject", (buffer, bufferSize))?;
        Ok(__cordl_ret)
    }
    pub fn MakeCurrent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MakeCurrent", ())?;
        Ok(__cordl_ret)
    }
    pub fn DumpControlBitRangeNode(
        &mut self,
        nodeIndex: i32,
        node: crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode,
        startOffset: u32,
        sizeInBits: u32,
        output: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DumpControlBitRangeNode",
                (nodeIndex, node, startOffset, sizeInBits, output),
            )?;
        Ok(__cordl_ret)
    }
    pub fn WriteChangedControlStatesInternal(
        &mut self,
        statePtr: *mut quest_hook::libil2cpp::Il2CppObject,
        stateSizeInBits: u32,
        deviceStatePtr: *mut quest_hook::libil2cpp::Il2CppObject,
        parentNode: crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode,
        startOffset: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteChangedControlStatesInternal",
                (statePtr, stateSizeInBits, deviceStatePtr, parentNode, startOffset),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputDevice")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::InputSystem::InputDevice {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
