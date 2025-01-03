#[cfg(feature = "UnityEngine+Rendering+BatchCullingOutput")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BatchCullingOutput {
    pub drawCommands: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::BatchCullingOutputDrawCommands,
    >,
}
#[cfg(feature = "UnityEngine+Rendering+BatchCullingOutput")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::BatchCullingOutput =>
    "UnityEngine.Rendering"."BatchCullingOutput"
);
#[cfg(feature = "UnityEngine+Rendering+BatchCullingOutput")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::BatchCullingOutput {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchCullingOutput")]
impl crate::UnityEngine::Rendering::BatchCullingOutput {}
