#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+RenderEvents")]
#[repr(C)]
#[derive(Debug)]
pub struct RenderEvents {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+RenderEvents")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::Implementation::RenderEvents {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR.Implementation";
    const CLASS_NAME: &'static str = "RenderEvents";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+RenderEvents")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::UIR::Implementation::RenderEvents {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+RenderEvents")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::UIR::Implementation::RenderEvents {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+RenderEvents")]
impl crate::UnityEngine::UIElements::UIR::Implementation::RenderEvents {
    pub fn DepthFirstOnChildAdded(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        index: i32,
        resetState: bool,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DepthFirstOnChildAdded",
                (renderChain, parent, ve, index, resetState),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DepthFirstOnChildRemoving(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DepthFirstOnChildRemoving", (renderChain, ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn DepthFirstOnClippingChanged(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        dirtyID: u32,
        hierarchical: bool,
        isRootOfChange: bool,
        isPendingHierarchicalRepaint: bool,
        inheritedClipRectIDChanged: bool,
        inheritedMaskingChanged: bool,
        device: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::UIRenderDevice,
        >,
        stats: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DepthFirstOnClippingChanged",
                (
                    renderChain,
                    parent,
                    ve,
                    dirtyID,
                    hierarchical,
                    isRootOfChange,
                    isPendingHierarchicalRepaint,
                    inheritedClipRectIDChanged,
                    inheritedMaskingChanged,
                    device,
                    stats,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DepthFirstOnOpacityChanged(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        parentCompositeOpacity: f32,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        dirtyID: u32,
        hierarchical: bool,
        stats: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
        >,
        isDoingFullVertexRegeneration: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DepthFirstOnOpacityChanged",
                (
                    renderChain,
                    parentCompositeOpacity,
                    ve,
                    dirtyID,
                    hierarchical,
                    stats,
                    isDoingFullVertexRegeneration,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DepthFirstOnTransformOrSizeChanged(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        dirtyID: u32,
        device: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::UIRenderDevice,
        >,
        isAncestorOfChangeSkinned: bool,
        transformChanged: bool,
        stats: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DepthFirstOnTransformOrSizeChanged",
                (
                    renderChain,
                    parent,
                    ve,
                    dirtyID,
                    device,
                    isAncestorOfChangeSkinned,
                    transformChanged,
                    stats,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DepthFirstOnVisualsChanged(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        dirtyID: u32,
        parentHierarchyHidden: bool,
        hierarchical: bool,
        stats: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DepthFirstOnVisualsChanged",
                (renderChain, ve, dirtyID, parentHierarchyHidden, hierarchical, stats),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DetermineSelfClipMethod(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::UIR::Implementation::ClipMethod,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::Implementation::ClipMethod = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DetermineSelfClipMethod", (renderChain, ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetClipRectIDClipInfo(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_ret: crate::UnityEngine::Vector4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetClipRectIDClipInfo", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLastDeepestChild(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLastDeepestChild", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTransformIDTransformInfo(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTransformIDTransformInfo", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitColorIDs(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitColorIDs", (renderChain, ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsElementHierarchyHidden(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsElementHierarchyHidden", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn NeedsColorID(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NeedsColorID", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn NeedsTextCoreSettings(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NeedsTextCoreSettings", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn NeedsTransformID(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NeedsTransformID", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnColorChanged(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        dirtyID: u32,
        stats: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnColorChanged", (renderChain, ve, dirtyID, stats))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessOnClippingChanged(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        dirtyID: u32,
        stats: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessOnClippingChanged", (renderChain, ve, dirtyID, stats))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessOnColorChanged(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        dirtyID: u32,
        stats: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessOnColorChanged", (renderChain, ve, dirtyID, stats))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessOnOpacityChanged(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        dirtyID: u32,
        stats: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessOnOpacityChanged", (renderChain, ve, dirtyID, stats))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessOnTransformOrSizeChanged(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        dirtyID: u32,
        stats: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ProcessOnTransformOrSizeChanged",
                (renderChain, ve, dirtyID, stats),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessOnVisualsChanged(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        dirtyID: u32,
        stats: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessOnVisualsChanged", (renderChain, ve, dirtyID, stats))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColorValues(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetColorValues", (renderChain, ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateLocalFlipsWinding(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdateLocalFlipsWinding", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateTextCoreSettings(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdateTextCoreSettings", (renderChain, ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateWorldFlipsWinding(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdateWorldFlipsWinding", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateZeroScaling(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdateZeroScaling", (ve))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+RenderEvents")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::Implementation::RenderEvents {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
