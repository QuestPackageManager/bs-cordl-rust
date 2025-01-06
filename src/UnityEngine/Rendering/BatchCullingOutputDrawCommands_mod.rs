#[cfg(feature = "UnityEngine+Rendering+BatchCullingOutputDrawCommands")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BatchCullingOutputDrawCommands {
    pub drawCommands: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub visibleInstances: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub drawRanges: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub instanceSortingPositions: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub drawCommandPickingInstanceIDs: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub drawCommandCount: i32,
    pub visibleInstanceCount: i32,
    pub drawRangeCount: i32,
    pub instanceSortingPositionFloatCount: i32,
}
#[cfg(feature = "UnityEngine+Rendering+BatchCullingOutputDrawCommands")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Rendering::BatchCullingOutputDrawCommands => "UnityEngine.Rendering"
    ."BatchCullingOutputDrawCommands"
);
#[cfg(feature = "UnityEngine+Rendering+BatchCullingOutputDrawCommands")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::BatchCullingOutputDrawCommands {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchCullingOutputDrawCommands")]
impl crate::UnityEngine::Rendering::BatchCullingOutputDrawCommands {}
