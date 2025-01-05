#[cfg(feature = "UnityEngine+Yoga+Native")]
#[repr(C)]
#[derive(Debug)]
pub struct Native {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+Yoga+Native")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Yoga::Native => "UnityEngine.Yoga"
    ."Native"
);
#[cfg(feature = "UnityEngine+Yoga+Native")]
impl std::ops::Deref for crate::UnityEngine::Yoga::Native {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Yoga+Native")]
impl std::ops::DerefMut for crate::UnityEngine::Yoga::Native {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Yoga+Native")]
impl crate::UnityEngine::Yoga::Native {
    pub fn YGConfigFree(
        config: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGConfigFree", (config))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGConfigFreeInternal(
        config: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGConfigFreeInternal", (config))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGConfigGetDefault() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGConfigGetDefault", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn YGConfigGetUseWebDefaults(
        config: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGConfigGetUseWebDefaults", (config))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGConfigNew() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGConfigNew", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn YGConfigSetPointScaleFactor(
        config: crate::System::IntPtr,
        pixelsInPoint: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGConfigSetPointScaleFactor", (config, pixelsInPoint))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGConfigSetUseWebDefaults(
        config: crate::System::IntPtr,
        useWebDefaults: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGConfigSetUseWebDefaults", (config, useWebDefaults))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeBaselineInvoke(
        node: quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
        width: f32,
        height: f32,
        returnValueAddress: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeBaselineInvoke", (node, width, height, returnValueAddress))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeCalculateLayout(
        node: crate::System::IntPtr,
        availableWidth: f32,
        availableHeight: f32,
        parentDirection: crate::UnityEngine::Yoga::YogaDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "YGNodeCalculateLayout",
                (node, availableWidth, availableHeight, parentDirection),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeCopyStyle(
        dstNode: crate::System::IntPtr,
        srcNode: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeCopyStyle", (dstNode, srcNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeFree(
        ygNode: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeFree", (ygNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeFreeInternal(
        ygNode: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeFreeInternal", (ygNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeGetHasNewLayout(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeGetHasNewLayout", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeInsertChild(
        node: crate::System::IntPtr,
        child: crate::System::IntPtr,
        index: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeInsertChild", (node, child, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeIsDirty(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeIsDirty", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeLayoutGetBorder(
        node: crate::System::IntPtr,
        edge: crate::UnityEngine::Yoga::YogaEdge,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeLayoutGetBorder", (node, edge))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeLayoutGetBottom(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeLayoutGetBottom", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeLayoutGetHeight(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeLayoutGetHeight", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeLayoutGetLeft(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeLayoutGetLeft", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeLayoutGetMargin(
        node: crate::System::IntPtr,
        edge: crate::UnityEngine::Yoga::YogaEdge,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeLayoutGetMargin", (node, edge))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeLayoutGetPadding(
        node: crate::System::IntPtr,
        edge: crate::UnityEngine::Yoga::YogaEdge,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeLayoutGetPadding", (node, edge))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeLayoutGetRight(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeLayoutGetRight", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeLayoutGetTop(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeLayoutGetTop", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeLayoutGetWidth(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeLayoutGetWidth", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeMarkDirty(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeMarkDirty", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeMeasureInvoke(
        node: quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
        width: f32,
        widthMode: crate::UnityEngine::Yoga::YogaMeasureMode,
        height: f32,
        heightMode: crate::UnityEngine::Yoga::YogaMeasureMode,
        returnValueAddress: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "YGNodeMeasureInvoke",
                (node, width, widthMode, height, heightMode, returnValueAddress),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeNewWithConfig(
        config: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeNewWithConfig", (config))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeRemoveChild(
        node: crate::System::IntPtr,
        child: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeRemoveChild", (node, child))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeRemoveMeasureFunc(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeRemoveMeasureFunc", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeSetConfig(
        ygNode: crate::System::IntPtr,
        config: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeSetConfig", (ygNode, config))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeSetHasNewLayout(
        node: crate::System::IntPtr,
        hasNewLayout: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeSetHasNewLayout", (node, hasNewLayout))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeSetMeasureFunc(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeSetMeasureFunc", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleGetDirection(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Yoga::YogaDirection> {
        let __cordl_ret: crate::UnityEngine::Yoga::YogaDirection = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleGetDirection", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetAlignContent(
        node: crate::System::IntPtr,
        alignContent: crate::UnityEngine::Yoga::YogaAlign,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetAlignContent", (node, alignContent))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetAlignItems(
        node: crate::System::IntPtr,
        alignItems: crate::UnityEngine::Yoga::YogaAlign,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetAlignItems", (node, alignItems))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetAlignSelf(
        node: crate::System::IntPtr,
        alignSelf: crate::UnityEngine::Yoga::YogaAlign,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetAlignSelf", (node, alignSelf))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetBorder(
        node: crate::System::IntPtr,
        edge: crate::UnityEngine::Yoga::YogaEdge,
        border: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetBorder", (node, edge, border))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetDisplay(
        node: crate::System::IntPtr,
        display: crate::UnityEngine::Yoga::YogaDisplay,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetDisplay", (node, display))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetFlex(
        node: crate::System::IntPtr,
        flex: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetFlex", (node, flex))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetFlexBasis(
        node: crate::System::IntPtr,
        flexBasis: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetFlexBasis", (node, flexBasis))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetFlexBasisAuto(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetFlexBasisAuto", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetFlexBasisPercent(
        node: crate::System::IntPtr,
        flexBasis: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetFlexBasisPercent", (node, flexBasis))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetFlexDirection(
        node: crate::System::IntPtr,
        flexDirection: crate::UnityEngine::Yoga::YogaFlexDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetFlexDirection", (node, flexDirection))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetFlexGrow(
        node: crate::System::IntPtr,
        flexGrow: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetFlexGrow", (node, flexGrow))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetFlexShrink(
        node: crate::System::IntPtr,
        flexShrink: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetFlexShrink", (node, flexShrink))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetFlexWrap(
        node: crate::System::IntPtr,
        flexWrap: crate::UnityEngine::Yoga::YogaWrap,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetFlexWrap", (node, flexWrap))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetHeight(
        node: crate::System::IntPtr,
        height: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetHeight", (node, height))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetHeightAuto(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetHeightAuto", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetHeightPercent(
        node: crate::System::IntPtr,
        height: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetHeightPercent", (node, height))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetJustifyContent(
        node: crate::System::IntPtr,
        justifyContent: crate::UnityEngine::Yoga::YogaJustify,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetJustifyContent", (node, justifyContent))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetMargin(
        node: crate::System::IntPtr,
        edge: crate::UnityEngine::Yoga::YogaEdge,
        margin: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetMargin", (node, edge, margin))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetMarginAuto(
        node: crate::System::IntPtr,
        edge: crate::UnityEngine::Yoga::YogaEdge,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetMarginAuto", (node, edge))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetMarginPercent(
        node: crate::System::IntPtr,
        edge: crate::UnityEngine::Yoga::YogaEdge,
        margin: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetMarginPercent", (node, edge, margin))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetMaxHeight(
        node: crate::System::IntPtr,
        maxHeight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetMaxHeight", (node, maxHeight))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetMaxHeightPercent(
        node: crate::System::IntPtr,
        maxHeight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetMaxHeightPercent", (node, maxHeight))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetMaxWidth(
        node: crate::System::IntPtr,
        maxWidth: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetMaxWidth", (node, maxWidth))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetMaxWidthPercent(
        node: crate::System::IntPtr,
        maxWidth: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetMaxWidthPercent", (node, maxWidth))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetMinHeight(
        node: crate::System::IntPtr,
        minHeight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetMinHeight", (node, minHeight))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetMinHeightPercent(
        node: crate::System::IntPtr,
        minHeight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetMinHeightPercent", (node, minHeight))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetMinWidth(
        node: crate::System::IntPtr,
        minWidth: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetMinWidth", (node, minWidth))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetMinWidthPercent(
        node: crate::System::IntPtr,
        minWidth: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetMinWidthPercent", (node, minWidth))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetOverflow(
        node: crate::System::IntPtr,
        flexWrap: crate::UnityEngine::Yoga::YogaOverflow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetOverflow", (node, flexWrap))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetPadding(
        node: crate::System::IntPtr,
        edge: crate::UnityEngine::Yoga::YogaEdge,
        padding: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetPadding", (node, edge, padding))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetPaddingPercent(
        node: crate::System::IntPtr,
        edge: crate::UnityEngine::Yoga::YogaEdge,
        padding: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetPaddingPercent", (node, edge, padding))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetPosition(
        node: crate::System::IntPtr,
        edge: crate::UnityEngine::Yoga::YogaEdge,
        position: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetPosition", (node, edge, position))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetPositionPercent(
        node: crate::System::IntPtr,
        edge: crate::UnityEngine::Yoga::YogaEdge,
        position: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetPositionPercent", (node, edge, position))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetPositionType(
        node: crate::System::IntPtr,
        positionType: crate::UnityEngine::Yoga::YogaPositionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetPositionType", (node, positionType))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetWidth(
        node: crate::System::IntPtr,
        width: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetWidth", (node, width))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetWidthAuto(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetWidthAuto", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetWidthPercent(
        node: crate::System::IntPtr,
        width: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGNodeStyleSetWidthPercent", (node, width))?;
        Ok(__cordl_ret.into())
    }
    pub fn YGSetManagedObject(
        ygNode: crate::System::IntPtr,
        node: quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YGSetManagedObject", (ygNode, node))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Yoga+Native")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Yoga::Native {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
