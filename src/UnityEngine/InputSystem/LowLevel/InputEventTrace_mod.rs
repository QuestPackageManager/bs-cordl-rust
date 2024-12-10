#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace")]
#[repr(C)]
#[derive(Debug)]
pub struct InputEventTrace {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_ChangeCounter: i32,
    pub m_Enabled: bool,
    pub m_OnFilterEvent: *mut crate::System::Func_3<
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        *mut crate::UnityEngine::InputSystem::InputDevice,
        bool,
    >,
    pub m_DeviceId: i32,
    pub m_EventListeners: crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
        *mut crate::System::Action_1<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        >,
    >,
    pub m_EventBufferSize: i64,
    pub m_MaxEventBufferSize: i64,
    pub m_GrowIncrementSize: i64,
    pub m_EventCount: i64,
    pub m_EventSizeInBytes: i64,
    pub m_EventBufferStorage: u64,
    pub m_EventBufferHeadStorage: u64,
    pub m_EventBufferTailStorage: u64,
    pub m_HasWrapped: bool,
    pub m_RecordFrameMarkers: bool,
    pub m_DeviceInfos: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_DeviceInfo,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::InputEventTrace =>
    "UnityEngine.InputSystem.LowLevel"."InputEventTrace"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::LowLevel::InputEventTrace {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::LowLevel::InputEventTrace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace")]
impl crate::UnityEngine::InputSystem::LowLevel::InputEventTrace {
    pub const kDefaultBufferSize: i32 = 1048576i32;
    #[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace+DeviceInfo")]
    pub type DeviceInfo = crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_DeviceInfo;
    #[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace+Enumerator")]
    pub type Enumerator = crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_Enumerator;
    #[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace+FileFlags")]
    pub type FileFlags = crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_FileFlags;
    #[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace+ReplayController")]
    pub type ReplayController = crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController;
    pub fn Allocate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Allocate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Disable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Disable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Enable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Enable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextEvent(
        &mut self,
        current: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetNextEvent", (current))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_InputDevice_i64__cordl_bool_i64_0(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        bufferSizeInBytes: i64,
        growBuffer: bool,
        maxBufferSizeInBytes: i64,
        growIncrementSizeInBytes: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    device,
                    bufferSizeInBytes,
                    growBuffer,
                    maxBufferSizeInBytes,
                    growIncrementSizeInBytes,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i64__cordl_bool_i64_1(
        bufferSizeInBytes: i64,
        growBuffer: bool,
        maxBufferSizeInBytes: i64,
        growIncrementSizeInBytes: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    bufferSizeInBytes,
                    growBuffer,
                    maxBufferSizeInBytes,
                    growIncrementSizeInBytes,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn OnBeforeUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBeforeUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnInputEvent(
        &mut self,
        inputEvent: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnInputEvent", (inputEvent, device))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadFrom_Il2CppString0(
        &mut self,
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadFrom", (filePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadFrom_Stream1(
        &mut self,
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadFrom", (stream))?;
        Ok(__cordl_ret.into())
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Replay(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController,
        > = __cordl_object.invoke("Replay", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Resize(
        &mut self,
        newBufferSize: i64,
        newMaxBufferSize: i64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Resize", (newBufferSize, newMaxBufferSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteTo_Il2CppString0(
        &mut self,
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteTo", (filePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteTo_Stream1(
        &mut self,
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteTo", (stream))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_InputDevice_i64__cordl_bool_i64_0(
        &mut self,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        bufferSizeInBytes: i64,
        growBuffer: bool,
        maxBufferSizeInBytes: i64,
        growIncrementSizeInBytes: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    device,
                    bufferSizeInBytes,
                    growBuffer,
                    maxBufferSizeInBytes,
                    growIncrementSizeInBytes,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i64__cordl_bool_i64_1(
        &mut self,
        bufferSizeInBytes: i64,
        growBuffer: bool,
        maxBufferSizeInBytes: i64,
        growIncrementSizeInBytes: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    bufferSizeInBytes,
                    growBuffer,
                    maxBufferSizeInBytes,
                    growIncrementSizeInBytes,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn add_onEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_allocatedSizeInBytes(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_allocatedSizeInBytes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deviceId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_deviceId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deviceInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_DeviceInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_DeviceInfo,
        > = __cordl_object.invoke("get_deviceInfos", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_eventCount(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_eventCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_m_EventBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_m_EventBuffer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_m_EventBufferHead(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_m_EventBufferHead", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_m_EventBufferTail(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_m_EventBufferTail", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxSizeInBytes(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_maxSizeInBytes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_onFilterEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                *mut crate::UnityEngine::InputSystem::InputDevice,
                bool,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                *mut crate::UnityEngine::InputSystem::InputDevice,
                bool,
            >,
        > = __cordl_object.invoke("get_onFilterEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_recordFrameMarkers(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_recordFrameMarkers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_totalEventSizeInBytes(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_totalEventSizeInBytes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_deviceId(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_deviceId", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_m_EventBuffer(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_m_EventBuffer", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_m_EventBufferHead(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_m_EventBufferHead", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_m_EventBufferTail(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_m_EventBufferTail", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_onFilterEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                *mut crate::UnityEngine::InputSystem::InputDevice,
                bool,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onFilterEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_recordFrameMarkers(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_recordFrameMarkers", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::LowLevel::InputEventTrace {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace+DeviceInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputEventTrace_DeviceInfo {
    pub m_DeviceId: i32,
    pub m_Layout: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_StateFormat: crate::UnityEngine::InputSystem::Utilities::FourCC,
    pub m_StateSizeInBytes: i32,
    pub m_FullLayoutJson: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace+DeviceInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::InputEventTrace_DeviceInfo =>
    "UnityEngine.InputSystem.LowLevel"."InputEventTrace/DeviceInfo"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace+DeviceInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_DeviceInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace+DeviceInfo")]
impl crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_DeviceInfo {
    pub fn get_deviceId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_deviceId",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_layout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_layout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_stateFormat(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_stateFormat",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_stateSizeInBytes(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_stateSizeInBytes",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_deviceId(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_deviceId",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_layout(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_layout",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_stateFormat(
        &mut self,
        value: crate::UnityEngine::InputSystem::Utilities::FourCC,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_stateFormat",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_stateSizeInBytes(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_stateSizeInBytes",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace+Enumerator")]
#[repr(C)]
#[derive(Debug)]
pub struct InputEventTrace_Enumerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Trace: *mut crate::UnityEngine::InputSystem::LowLevel::InputEventTrace,
    pub m_ChangeCounter: i32,
    pub m_Current: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace+Enumerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::InputEventTrace_Enumerator =>
    "UnityEngine.InputSystem.LowLevel"."InputEventTrace/Enumerator"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace+Enumerator")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_Enumerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace+Enumerator")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_Enumerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace+Enumerator")]
impl crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_Enumerator {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        trace: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (trace))?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("System.Collections.IEnumerator.get_Current", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        trace: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (trace))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr = __cordl_object
            .invoke("get_Current", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace+Enumerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_Enumerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace+FileFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEventTrace_FileFlags {
    FixedUpdate = 1i32,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace+FileFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::InputEventTrace_FileFlags =>
    "UnityEngine.InputSystem.LowLevel"."InputEventTrace/FileFlags"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace+ReplayController")]
#[repr(C)]
#[derive(Debug)]
pub struct InputEventTrace_ReplayController {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _finished_k__BackingField: bool,
    pub _paused_k__BackingField: bool,
    pub _position_k__BackingField: i32,
    pub m_EventTrace: *mut crate::UnityEngine::InputSystem::LowLevel::InputEventTrace,
    pub m_Enumerator: *mut crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_Enumerator,
    pub m_DeviceIDMappings: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
        crate::System::Collections::Generic::KeyValuePair_2<i32, i32>,
    >,
    pub m_CreateNewDevices: bool,
    pub m_CreatedDevices: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
        *mut crate::UnityEngine::InputSystem::InputDevice,
    >,
    pub m_OnFinished: *mut crate::System::Action,
    pub m_OnEvent: *mut crate::System::Action_1<
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    >,
    pub m_StartTimeAsPerFirstEvent: f64,
    pub m_StartTimeAsPerRuntime: f64,
    pub m_AllEventsByTimeIndex: i32,
    pub m_AllEventsByTime: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace+ReplayController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController =>
    "UnityEngine.InputSystem.LowLevel"."InputEventTrace/ReplayController"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace+ReplayController")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace+ReplayController")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace+ReplayController")]
impl crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController {
    #[cfg(
        feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace+ReplayController+__c"
    )]
    pub type __c = crate::UnityEngine::InputSystem::LowLevel::ReplayController_InputEventTrace___c;
    #[cfg(
        feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace+ReplayController+__c__DisplayClass43_0"
    )]
    pub type __c__DisplayClass43_0 = crate::UnityEngine::InputSystem::LowLevel::ReplayController_InputEventTrace___c__DisplayClass43_0;
    pub fn ApplyDeviceMapping(
        &mut self,
        originalDeviceId: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ApplyDeviceMapping", (originalDeviceId))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Finished(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finished", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(
        &mut self,
        skipFrameEvents: bool,
        eventPtr: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MoveNext", (skipFrameEvents, eventPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        trace: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (trace))?;
        Ok(__cordl_object.into())
    }
    pub fn OnBeginFrame(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBeginFrame", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEvent(
        &mut self,
        action: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController,
        > = __cordl_object.invoke("OnEvent", (action))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnFinished(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController,
        > = __cordl_object.invoke("OnFinished", (action))?;
        Ok(__cordl_ret.into())
    }
    pub fn PlayAllEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController,
        > = __cordl_object.invoke("PlayAllEvents", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PlayAllEventsAccordingToTimestamps(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController,
        > = __cordl_object.invoke("PlayAllEventsAccordingToTimestamps", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PlayAllFramesOneByOne(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController,
        > = __cordl_object.invoke("PlayAllFramesOneByOne", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PlayOneEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController,
        > = __cordl_object.invoke("PlayOneEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn QueueEvent(
        &mut self,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("QueueEvent", (eventPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn Rewind(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController,
        > = __cordl_object.invoke("Rewind", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WithAllDevicesMappedToNewInstances(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController,
        > = __cordl_object.invoke("WithAllDevicesMappedToNewInstances", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WithDeviceMappedFromTo_InputDevice_InputDevice0(
        &mut self,
        recordedDevice: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputDevice,
        >,
        playbackDevice: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputDevice,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController,
        > = __cordl_object
            .invoke("WithDeviceMappedFromTo", (recordedDevice, playbackDevice))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithDeviceMappedFromTo_i32_i32_1(
        &mut self,
        recordedDeviceId: i32,
        playbackDeviceId: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController,
        > = __cordl_object
            .invoke("WithDeviceMappedFromTo", (recordedDeviceId, playbackDeviceId))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        trace: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (trace))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_createdDevices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::InputSystem::InputDevice,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::InputSystem::InputDevice,
            >,
        > = __cordl_object.invoke("get_createdDevices", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_finished(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_finished", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_paused(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_paused", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_position(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_position", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_trace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::InputEventTrace,
        > = __cordl_object.invoke("get_trace", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_finished(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_finished", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_paused(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_paused", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_position(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_position", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventTrace+ReplayController")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::LowLevel::InputEventTrace_ReplayController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
