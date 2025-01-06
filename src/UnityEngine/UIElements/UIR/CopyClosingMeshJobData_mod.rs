#[cfg(feature = "UnityEngine+UIElements+UIR+CopyClosingMeshJobData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CopyClosingMeshJobData {
    pub vertSrc: crate::System::IntPtr,
    pub vertDst: crate::System::IntPtr,
    pub vertCount: i32,
    pub indexSrc: crate::System::IntPtr,
    pub indexDst: crate::System::IntPtr,
    pub indexCount: i32,
    pub indexOffset: i32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+CopyClosingMeshJobData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::CopyClosingMeshJobData =>
    "UnityEngine.UIElements.UIR"."CopyClosingMeshJobData"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+CopyClosingMeshJobData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::CopyClosingMeshJobData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+CopyClosingMeshJobData")]
impl crate::UnityEngine::UIElements::UIR::CopyClosingMeshJobData {}
