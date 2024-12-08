#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChainVEData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RenderChainVEData {
    pub prev: *mut crate::UnityEngine::UIElements::VisualElement,
    pub next: *mut crate::UnityEngine::UIElements::VisualElement,
    pub groupTransformAncestor: *mut crate::UnityEngine::UIElements::VisualElement,
    pub boneTransformAncestor: *mut crate::UnityEngine::UIElements::VisualElement,
    pub prevDirty: *mut crate::UnityEngine::UIElements::VisualElement,
    pub nextDirty: *mut crate::UnityEngine::UIElements::VisualElement,
    pub flags: crate::UnityEngine::UIElements::UIR::RenderDataFlags,
    pub hierarchyDepth: i32,
    pub dirtiedValues: crate::UnityEngine::UIElements::UIR::RenderDataDirtyTypes,
    pub dirtyID: u32,
    pub firstCommand: *mut crate::UnityEngine::UIElements::UIR::RenderChainCommand,
    pub lastCommand: *mut crate::UnityEngine::UIElements::UIR::RenderChainCommand,
    pub firstClosingCommand: *mut crate::UnityEngine::UIElements::UIR::RenderChainCommand,
    pub lastClosingCommand: *mut crate::UnityEngine::UIElements::UIR::RenderChainCommand,
    pub isInChain: bool,
    pub isHierarchyHidden: bool,
    pub localFlipsWinding: bool,
    pub localTransformScaleZero: bool,
    pub worldFlipsWinding: bool,
    pub worldTransformScaleZero: bool,
    pub clipMethod: crate::UnityEngine::UIElements::UIR::Implementation::ClipMethod,
    pub childrenStencilRef: i32,
    pub childrenMaskDepth: i32,
    pub disableNudging: bool,
    pub data: *mut crate::UnityEngine::UIElements::UIR::MeshHandle,
    pub closingData: *mut crate::UnityEngine::UIElements::UIR::MeshHandle,
    pub verticesSpace: crate::UnityEngine::Matrix4x4,
    pub displacementUVStart: i32,
    pub displacementUVEnd: i32,
    pub transformID: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    pub clipRectID: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    pub opacityID: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    pub textCoreSettingsID: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    pub colorID: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    pub backgroundColorID: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    pub borderLeftColorID: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    pub borderTopColorID: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    pub borderRightColorID: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    pub borderBottomColorID: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    pub tintColorID: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    pub compositeOpacity: f32,
    pub backgroundColor: crate::UnityEngine::Color,
    pub textures: *mut crate::UnityEngine::UIElements::UIR::BasicNode_1<
        crate::UnityEngine::UIElements::UIR::TextureEntry,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChainVEData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::RenderChainVEData
    => "UnityEngine.UIElements.UIR"."RenderChainVEData"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChainVEData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::RenderChainVEData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChainVEData")]
impl crate::UnityEngine::UIElements::UIR::RenderChainVEData {
    pub fn get_isIgnoringDynamicColorHint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isIgnoringDynamicColorHint",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_lastClosingOrLastCommand(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::UIR::RenderChainCommand,
    > {
        let __cordl_ret: *mut crate::UnityEngine::UIElements::UIR::RenderChainCommand = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_lastClosingOrLastCommand",
            (),
        )?;
        Ok(__cordl_ret)
    }
}