#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventBuffer")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputEventBuffer {
    pub m_Buffer: crate::Unity::Collections::NativeArray_1<u8>,
    pub m_SizeInBytes: i64,
    pub m_EventCount: i32,
    pub m_WeOwnTheBuffer: bool,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::InputEventBuffer =>
    "UnityEngine.InputSystem.LowLevel"."InputEventBuffer"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::InputEventBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventBuffer")]
impl crate::UnityEngine::InputSystem::LowLevel::InputEventBuffer {
    pub const BufferSizeUnknown: i64 = -1i64;
    #[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventBuffer+Enumerator")]
    pub type Enumerator = crate::UnityEngine::InputSystem::LowLevel::InputEventBuffer_Enumerator;
    pub fn AdvanceToNextEvent(
        &mut self,
        currentReadPos: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
        currentWritePos: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
        numEventsRetainedInBuffer: quest_hook::libil2cpp::ByRefMut<i32>,
        numRemainingEvents: quest_hook::libil2cpp::ByRefMut<i32>,
        leaveEventInBuffer: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AdvanceToNextEvent",
            (
                currentReadPos,
                currentWritePos,
                numEventsRetainedInBuffer,
                numRemainingEvents,
                leaveEventInBuffer,
            ),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AllocateEvent(
        &mut self,
        sizeInBytes: i32,
        capacityIncrementInBytes: i32,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AllocateEvent",
            (sizeInBytes, capacityIncrementInBytes, allocator),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AppendEvent(
        &mut self,
        eventPtr: *mut quest_hook::libil2cpp::Il2CppObject,
        capacityIncrementInBytes: i32,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AppendEvent",
            (eventPtr, capacityIncrementInBytes, allocator),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputEventBuffer,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputEventBuffer = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clone",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Contains(
        &mut self,
        eventPtr: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Contains",
            (eventPtr),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerator_1<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        >,
    > {
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerator_1<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Reset",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerable.GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_ICloneable_Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_ret: *mut crate::System::Object = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ICloneable.Clone",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppObject_i32_0(
        &mut self,
        eventPtr: *mut quest_hook::libil2cpp::Il2CppObject,
        eventCount: i32,
        sizeInBytes: i32,
        capacityInBytes: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (eventPtr, eventCount, sizeInBytes, capacityInBytes),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_NativeArray_1__cordl_bool1(
        &mut self,
        buffer: crate::Unity::Collections::NativeArray_1<u8>,
        eventCount: i32,
        sizeInBytes: i32,
        transferNativeArrayOwnership: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (buffer, eventCount, sizeInBytes, transferNativeArrayOwnership),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_bufferPtr(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_bufferPtr",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_capacityInBytes(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_capacityInBytes",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_data(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<u8>> {
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<u8> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_data",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_eventCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_eventCount",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_sizeInBytes(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_sizeInBytes",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventBuffer+Enumerator")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputEventBuffer_Enumerator {
    pub m_Buffer: *mut quest_hook::libil2cpp::Il2CppObject,
    pub m_EventCount: i32,
    pub m_CurrentEvent: *mut quest_hook::libil2cpp::Il2CppObject,
    pub m_CurrentIndex: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventBuffer+Enumerator")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::InputEventBuffer_Enumerator =>
    "UnityEngine.InputSystem.LowLevel"."InputEventBuffer/Enumerator"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventBuffer+Enumerator")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::InputEventBuffer_Enumerator {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventBuffer+Enumerator")]
impl crate::UnityEngine::InputSystem::LowLevel::InputEventBuffer_Enumerator {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Reset",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_ret: *mut crate::System::Object = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerator.get_Current",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        buffer: crate::UnityEngine::InputSystem::LowLevel::InputEventBuffer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (buffer),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Current",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
