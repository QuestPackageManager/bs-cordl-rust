#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeltaStateEvent")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DeltaStateEvent {
    padding: quest_hook::libil2cpp::ValueTypePadding<29usize>,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeltaStateEvent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::DeltaStateEvent =>
    "UnityEngine.InputSystem.LowLevel"."DeltaStateEvent"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeltaStateEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::DeltaStateEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeltaStateEvent")]
impl crate::UnityEngine::InputSystem::LowLevel::DeltaStateEvent {
    pub const Type: i32 = 1145852993i32;
    #[cfg(
        feature = "UnityEngine+InputSystem+LowLevel+DeltaStateEvent+_stateData_e__FixedBuffer"
    )]
    pub type _stateData_e__FixedBuffer = crate::UnityEngine::InputSystem::LowLevel::DeltaStateEvent__stateData_e__FixedBuffer;
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
    pub fn From_InputControl_ByRefMut_Allocator1(
        control: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputControl,
        >,
        eventPtr: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        >,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<u8>> {
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<u8> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("From", (control, eventPtr, allocator))?;
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
    pub fn get_deltaState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_deltaState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deltaStateSizeInBytes(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_deltaStateSizeInBytes",
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeltaStateEvent")]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IInputEventTypeInfo>
for crate::UnityEngine::InputSystem::LowLevel::DeltaStateEvent {
    fn as_ref(&self) -> &crate::UnityEngine::InputSystem::LowLevel::IInputEventTypeInfo {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeltaStateEvent")]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IInputEventTypeInfo>
for crate::UnityEngine::InputSystem::LowLevel::DeltaStateEvent {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::IInputEventTypeInfo {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+DeltaStateEvent+_stateData_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DeltaStateEvent__stateData_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+DeltaStateEvent+_stateData_e__FixedBuffer"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::DeltaStateEvent__stateData_e__FixedBuffer =>
    "UnityEngine.InputSystem.LowLevel"."DeltaStateEvent/<stateData>e__FixedBuffer"
);
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+DeltaStateEvent+_stateData_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::DeltaStateEvent__stateData_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+DeltaStateEvent+_stateData_e__FixedBuffer"
)]
impl crate::UnityEngine::InputSystem::LowLevel::DeltaStateEvent__stateData_e__FixedBuffer {}
