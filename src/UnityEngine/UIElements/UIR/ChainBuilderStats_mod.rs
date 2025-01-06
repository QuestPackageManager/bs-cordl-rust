#[cfg(feature = "UnityEngine+UIElements+UIR+ChainBuilderStats")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChainBuilderStats {
    pub elementsAdded: u32,
    pub elementsRemoved: u32,
    pub recursiveClipUpdates: u32,
    pub recursiveClipUpdatesExpanded: u32,
    pub nonRecursiveClipUpdates: u32,
    pub recursiveTransformUpdates: u32,
    pub recursiveTransformUpdatesExpanded: u32,
    pub recursiveOpacityUpdates: u32,
    pub recursiveOpacityUpdatesExpanded: u32,
    pub opacityIdUpdates: u32,
    pub colorUpdates: u32,
    pub colorUpdatesExpanded: u32,
    pub recursiveVisualUpdates: u32,
    pub recursiveVisualUpdatesExpanded: u32,
    pub nonRecursiveVisualUpdates: u32,
    pub dirtyProcessed: u32,
    pub nudgeTransformed: u32,
    pub boneTransformed: u32,
    pub skipTransformed: u32,
    pub visualUpdateTransformed: u32,
    pub updatedMeshAllocations: u32,
    pub newMeshAllocations: u32,
    pub groupTransformElementsChanged: u32,
    pub immedateRenderersActive: u32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+ChainBuilderStats")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::ChainBuilderStats
    => "UnityEngine.UIElements.UIR"."ChainBuilderStats"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+ChainBuilderStats")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::ChainBuilderStats {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+ChainBuilderStats")]
impl crate::UnityEngine::UIElements::UIR::ChainBuilderStats {}
