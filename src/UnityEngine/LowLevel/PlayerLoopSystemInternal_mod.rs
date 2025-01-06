#[cfg(feature = "UnityEngine+LowLevel+PlayerLoopSystemInternal")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerLoopSystemInternal {
    pub _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub updateDelegate: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::LowLevel::PlayerLoopSystem_UpdateFunction,
    >,
    pub updateFunction: crate::System::IntPtr,
    pub loopConditionFunction: crate::System::IntPtr,
    pub numSubSystems: i32,
}
#[cfg(feature = "UnityEngine+LowLevel+PlayerLoopSystemInternal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::LowLevel::PlayerLoopSystemInternal
    => "UnityEngine.LowLevel"."PlayerLoopSystemInternal"
);
#[cfg(feature = "UnityEngine+LowLevel+PlayerLoopSystemInternal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::LowLevel::PlayerLoopSystemInternal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+LowLevel+PlayerLoopSystemInternal")]
impl crate::UnityEngine::LowLevel::PlayerLoopSystemInternal {}
