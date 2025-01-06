#[cfg(feature = "UnityEngine+Rendering+BatchDrawCommand")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BatchDrawCommand {
    pub visibleOffset: u32,
    pub visibleCount: u32,
    pub batchID: crate::UnityEngine::Rendering::BatchID,
    pub materialID: crate::UnityEngine::Rendering::BatchMaterialID,
    pub meshID: crate::UnityEngine::Rendering::BatchMeshID,
    pub submeshIndex: u16,
    pub splitVisibilityMask: u16,
    pub flags: crate::UnityEngine::Rendering::BatchDrawCommandFlags,
    pub sortingPosition: i32,
}
#[cfg(feature = "UnityEngine+Rendering+BatchDrawCommand")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::BatchDrawCommand =>
    "UnityEngine.Rendering"."BatchDrawCommand"
);
#[cfg(feature = "UnityEngine+Rendering+BatchDrawCommand")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::BatchDrawCommand {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchDrawCommand")]
impl crate::UnityEngine::Rendering::BatchDrawCommand {}
