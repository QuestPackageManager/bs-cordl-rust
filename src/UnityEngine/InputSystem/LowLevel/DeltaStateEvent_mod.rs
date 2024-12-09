#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeltaStateEvent")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DeltaStateEvent {
    padding: [u8; 29usize],
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
        Ok(__cordl_ret)
    }
    pub fn get_deltaState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_deltaState",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_deltaStateSizeInBytes(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_deltaStateSizeInBytes",
            (),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+DeltaStateEvent+_stateData_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone)]
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
