#[cfg(feature = "UnityEngine+InputSystem+LowLevel+StateEvent")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct StateEvent {
    padding: [u8; 25usize],
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+StateEvent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::LowLevel::StateEvent
    => "UnityEngine.InputSystem.LowLevel"."StateEvent"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+StateEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::StateEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+StateEvent")]
impl crate::UnityEngine::InputSystem::LowLevel::StateEvent {
    pub const Type: i32 = 1398030676i32;
    pub const kStateDataSizeToSubtract: i32 = 1i32;
    #[cfg(
        feature = "UnityEngine+InputSystem+LowLevel+StateEvent+_stateData_e__FixedBuffer"
    )]
    pub type _stateData_e__FixedBuffer = crate::UnityEngine::InputSystem::LowLevel::StateEvent__stateData_e__FixedBuffer;
    pub fn FromDefaultStateFor(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        eventPtr: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        >,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<u8>> {
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<u8> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromDefaultStateFor", (device, eventPtr, allocator))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromUnchecked(
        ptr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromUnchecked", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn From_InputDevice_ByRefMut_Allocator1(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        eventPtr: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        >,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<u8>> {
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<u8> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("From", (device, eventPtr, allocator))?;
        Ok(__cordl_ret.into())
    }
    pub fn From_InputDevice_ByRefMut_Allocator__cordl_bool2(
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
        eventPtr: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        >,
        allocator: crate::Unity::Collections::Allocator,
        useDefaultState: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<u8>> {
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<u8> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("From", (device, eventPtr, allocator, useDefaultState))?;
        Ok(__cordl_ret.into())
    }
    pub fn From_InputEventPtr0(
        ptr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("From", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEventSizeWithPayload<TState>() -> quest_hook::libil2cpp::Result<i32>
    where
        TState: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEventSizeWithPayload", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetState_0<TState>(&mut self) -> quest_hook::libil2cpp::Result<TState>
    where
        TState: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TState = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetState",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetState_InputEventPtr1<TState>(
        ptr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<TState>
    where
        TState: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TState = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetState", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToEventPtr(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToEventPtr",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_state(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_state", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_stateSizeInBytes(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_stateSizeInBytes",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_typeStatic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_typeStatic",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+StateEvent")]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IInputEventTypeInfo>
for crate::UnityEngine::InputSystem::LowLevel::StateEvent {
    fn as_ref(&self) -> &crate::UnityEngine::InputSystem::LowLevel::IInputEventTypeInfo {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+StateEvent")]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IInputEventTypeInfo>
for crate::UnityEngine::InputSystem::LowLevel::StateEvent {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::IInputEventTypeInfo {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+StateEvent+_stateData_e__FixedBuffer")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct StateEvent__stateData_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+StateEvent+_stateData_e__FixedBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::StateEvent__stateData_e__FixedBuffer =>
    "UnityEngine.InputSystem.LowLevel"."StateEvent/<stateData>e__FixedBuffer"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+StateEvent+_stateData_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::StateEvent__stateData_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+StateEvent+_stateData_e__FixedBuffer")]
impl crate::UnityEngine::InputSystem::LowLevel::StateEvent__stateData_e__FixedBuffer {}
