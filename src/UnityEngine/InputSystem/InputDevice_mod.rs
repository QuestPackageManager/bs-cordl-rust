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
    pub m_AliasesForEachControl: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        >,
    >,
    pub m_UsagesForEachControl: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::InputSystem::Utilities::InternedString,
        >,
    >,
    pub m_UsageToControl: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        >,
    >,
    pub m_ChildrenForEachControl: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        >,
    >,
    pub m_StateOffsetToControlMap: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u32>,
    >,
    pub m_ControlTreeNodes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode,
        >,
    >,
    pub m_ControlTreeIndices: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u16>,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputDevice")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputDevice {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputDevice";
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
    #[cfg(feature = "UnityEngine+InputSystem+InputDevice+ControlBitRangeNode")]
    pub type ControlBitRangeNode = crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode;
    #[cfg(feature = "UnityEngine+InputSystem+InputDevice+DeviceFlags")]
    pub type DeviceFlags = crate::UnityEngine::InputSystem::InputDevice_DeviceFlags;
    pub fn AddDeviceUsage(
        &mut self,
        usage: crate::UnityEngine::InputSystem::Utilities::InternedString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddDeviceUsage", (usage))?;
        Ok(__cordl_ret.into())
    }
    pub fn Build<TDevice>(
        layoutName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        layoutVariants: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        deviceDescription: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
        noPrecompiledLayouts: bool,
    ) -> quest_hook::libil2cpp::Result<TDevice>
    where
        TDevice: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TDevice = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Build",
                (layoutName, layoutVariants, deviceDescription, noPrecompiledLayouts),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearDeviceUsages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearDeviceUsages", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareValue(
        &mut self,
        firstStatePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        secondStatePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CompareValue", (firstStatePtr, secondStatePtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecodeStateOffsetToControlMapEntry(
        entry: u32,
        controlIndex: quest_hook::libil2cpp::ByRefMut<u32>,
        stateOffset: quest_hook::libil2cpp::ByRefMut<u32>,
        stateSize: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DecodeStateOffsetToControlMapEntry",
                (entry, controlIndex, stateOffset, stateSize),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DumpControlBitRangeNode(
        &mut self,
        nodeIndex: i32,
        node: crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode,
        startOffset: u32,
        sizeInBits: u32,
        output: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn DumpControlTree_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("DumpControlTree", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DumpControlTree_InputDevice_ControlBitRangeNode_u32_List_1_0(
        &mut self,
        parentNode: crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode,
        startOffset: u32,
        output: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DumpControlTree", (parentNode, startOffset, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn EncodeStateOffsetToControlMapEntry(
        controlIndex: u32,
        stateOffsetInBits: u32,
        stateSizeInBits: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "EncodeStateOffsetToControlMapEntry",
                (controlIndex, stateOffsetInBits, stateSizeInBits),
            )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteCommand_Il2CppObject1(
        &mut self,
        commandPtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("ExecuteCommand", (commandPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteDisableCommand(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ExecuteDisableCommand", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteEnableCommand(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ExecuteEnableCommand", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasDataChangedInRange(
        deviceStatePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        startOffset: u32,
        sizeInBits: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "HasDataChangedInRange",
                (deviceStatePtr, statePtr, startOffset, sizeInBits),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeCurrent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MakeCurrent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NotifyAdded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyAdded", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn NotifyConfigurationChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyConfigurationChanged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn NotifyRemoved(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyRemoved", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnAdded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAdded", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnConfigurationChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnConfigurationChanged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnRemoved(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRemoved", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn QueryEnabledStateFromRuntime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("QueryEnabledStateFromRuntime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadValueFromBufferAsObject(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ReadValueFromBufferAsObject", (buffer, bufferSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadValueFromStateAsObject(
        &mut self,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ReadValueFromStateAsObject", (statePtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadValueFromStateIntoBuffer(
        &mut self,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferPtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadValueFromStateIntoBuffer", (statePtr, bufferPtr, bufferSize))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn RequestReset(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RequestReset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RequestSync(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RequestSync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteChangedControlStates(
        &mut self,
        deviceStateBuffer: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
        Ok(__cordl_ret.into())
    }
    pub fn WriteChangedControlStatesInternal(
        &mut self,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        stateSizeInBits: u32,
        deviceStatePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
        Ok(__cordl_ret.into())
    }
    pub fn WritePartialChangedControlStatesInternal(
        &mut self,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        stateSizeInBits: u32,
        stateOffsetInDeviceInBits: u32,
        deviceStatePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
    pub fn get_added(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_added", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_all() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_all", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_allControls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
        > = __cordl_object.invoke("get_allControls", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_canRunInBackground(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_canRunInBackground", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_deviceId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_deviceId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_disabledInFrontend(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_disabledInFrontend", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_disabledInRuntime(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_disabledInRuntime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_disabledWhileInBackground(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_disabledWhileInBackground", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasControlsWithDefaultState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_hasControlsWithDefaultState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasDontResetControls(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasDontResetControls", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasEventMerger(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasEventMerger", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasEventPreProcessor(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasEventPreProcessor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasStateCallbacks(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasStateCallbacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lastUpdateTime(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_lastUpdateTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_native(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_native", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_remote(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_remote", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_updateBeforeRender(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_updateBeforeRender", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_valueSizeInBytes(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_valueSizeInBytes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_valueType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_valueType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wasUpdatedThisFrame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_wasUpdatedThisFrame", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
#[cfg(feature = "UnityEngine+InputSystem+InputDevice+ControlBitRangeNode")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputDevice_ControlBitRangeNode {
    pub endBitOffset: u16,
    pub leftChildIndex: i16,
    pub controlStartIndex: u16,
    pub controlCount: u8,
}
#[cfg(feature = "UnityEngine+InputSystem+InputDevice+ControlBitRangeNode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputDevice/ControlBitRangeNode";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputDevice+ControlBitRangeNode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputDevice+ControlBitRangeNode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputDevice+ControlBitRangeNode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputDevice+ControlBitRangeNode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputDevice_ControlBitRangeNode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputDevice+DeviceFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputDevice_DeviceFlags {
    #[default]
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
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::InputDevice_DeviceFlags {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "InputDevice/DeviceFlags";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputDevice+DeviceFlags")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::InputDevice_DeviceFlags {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputDevice+DeviceFlags")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::InputDevice_DeviceFlags {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputDevice+DeviceFlags")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::InputDevice_DeviceFlags {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputDevice+DeviceFlags")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::InputDevice_DeviceFlags {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
