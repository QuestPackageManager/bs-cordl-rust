#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventStream")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputEventStream {
    pub m_NativeBuffer: crate::UnityEngine::InputSystem::LowLevel::InputEventBuffer,
    pub m_CurrentNativeEventReadPtr: *mut quest_hook::libil2cpp::Il2CppObject,
    pub m_CurrentNativeEventWritePtr: *mut quest_hook::libil2cpp::Il2CppObject,
    pub m_RemainingNativeEventCount: i32,
    pub m_MaxAppendedEvents: i32,
    pub m_AppendBuffer: crate::UnityEngine::InputSystem::LowLevel::InputEventBuffer,
    pub m_CurrentAppendEventReadPtr: *mut quest_hook::libil2cpp::Il2CppObject,
    pub m_CurrentAppendEventWritePtr: *mut quest_hook::libil2cpp::Il2CppObject,
    pub m_RemainingAppendEventCount: i32,
    pub m_NumEventsRetainedInBuffer: i32,
    pub m_IsOpen: bool,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventStream")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::InputEventStream =>
    "UnityEngine.InputSystem.LowLevel"."InputEventStream"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventStream")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::InputEventStream {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputEventStream")]
impl crate::UnityEngine::InputSystem::LowLevel::InputEventStream {
    pub fn Advance(
        &mut self,
        leaveEventInBuffer: bool,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Advance",
            (leaveEventInBuffer),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CleanUpAfterException(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CleanUpAfterException",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Close(
        &mut self,
        eventBuffer: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::LowLevel::InputEventBuffer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Close",
            (eventBuffer),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Peek(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Peek",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        eventPtr: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Write",
            (eventPtr),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        eventBuffer: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::LowLevel::InputEventBuffer,
        >,
        maxAppendedEvents: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (eventBuffer, maxAppendedEvents),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_currentEventPtr(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_currentEventPtr",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isOpen(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isOpen",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_numBytesRetainedInBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_numBytesRetainedInBuffer",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_numEventsRetainedInBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_numEventsRetainedInBuffer",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_remainingEventCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_remainingEventCount",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
