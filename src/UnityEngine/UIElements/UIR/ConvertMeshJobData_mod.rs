#[cfg(feature = "UnityEngine+UIElements+UIR+ConvertMeshJobData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ConvertMeshJobData {
    pub vertSrc: crate::System::IntPtr,
    pub vertDst: crate::System::IntPtr,
    pub vertCount: i32,
    pub transform: crate::UnityEngine::Matrix4x4,
    pub transformUVs: i32,
    pub xformClipPages: crate::UnityEngine::Color32,
    pub ids: crate::UnityEngine::Color32,
    pub addFlags: crate::UnityEngine::Color32,
    pub opacityPage: crate::UnityEngine::Color32,
    pub textCoreSettingsPage: crate::UnityEngine::Color32,
    pub isText: i32,
    pub textureId: f32,
    pub indexSrc: crate::System::IntPtr,
    pub indexDst: crate::System::IntPtr,
    pub indexCount: i32,
    pub indexOffset: i32,
    pub flipIndices: i32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+ConvertMeshJobData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::ConvertMeshJobData
    => "UnityEngine.UIElements.UIR"."ConvertMeshJobData"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+ConvertMeshJobData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::ConvertMeshJobData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+ConvertMeshJobData")]
impl crate::UnityEngine::UIElements::UIR::ConvertMeshJobData {}
