#[cfg(feature = "UnityEngine+UIElements+VisualElement")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualElement {
    __cordl_parent: crate::UnityEngine::UIElements::Focusable,
    pub _UnityEngine_UIElements_IStylePropertyAnimations_runningAnimationCount_k__BackingField: i32,
    pub _UnityEngine_UIElements_IStylePropertyAnimations_completedAnimationCount_k__BackingField: i32,
    pub m_Name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_ClassList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    >,
    pub m_PropertyBag: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::System::Collections::Generic::KeyValuePair_2<
                crate::UnityEngine::PropertyName,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    >,
    pub m_Flags: crate::UnityEngine::UIElements::VisualElementFlags,
    pub m_ViewDataKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_RenderHints: crate::UnityEngine::UIElements::RenderHints,
    pub lastLayout: crate::UnityEngine::Rect,
    pub lastPseudoPadding: crate::UnityEngine::Rect,
    pub renderChainData: crate::UnityEngine::UIElements::UIR::RenderChainVEData,
    pub m_Layout: crate::UnityEngine::Rect,
    pub m_BoundingBox: crate::UnityEngine::Rect,
    pub m_WorldBoundingBox: crate::UnityEngine::Rect,
    pub m_WorldTransformCache: crate::UnityEngine::Matrix4x4,
    pub m_WorldTransformInverseCache: crate::UnityEngine::Matrix4x4,
    pub m_WorldClip: crate::UnityEngine::Rect,
    pub m_WorldClipMinusGroup: crate::UnityEngine::Rect,
    pub m_WorldClipIsInfinite: bool,
    pub triggerPseudoMask: crate::UnityEngine::UIElements::PseudoStates,
    pub dependencyPseudoMask: crate::UnityEngine::UIElements::PseudoStates,
    pub m_PseudoStates: crate::UnityEngine::UIElements::PseudoStates,
    pub _containedPointerIds_k__BackingField: i32,
    pub m_PickingMode: crate::UnityEngine::UIElements::PickingMode,
    pub _yogaNode_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Yoga::YogaNode,
    >,
    pub m_Style: crate::UnityEngine::UIElements::ComputedStyle,
    pub variableContext: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StyleVariableContext,
    >,
    pub inheritedStylesHash: i32,
    pub controlid: u32,
    pub imguiContainerDescendantCount: i32,
    pub _enabledSelf_k__BackingField: bool,
    pub m_LanguageDirection: crate::UnityEngine::UIElements::LanguageDirection,
    pub m_LocalLanguageDirection: crate::UnityEngine::UIElements::LanguageDirection,
    pub _generateVisualContent_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            *mut crate::UnityEngine::UIElements::MeshGenerationContext,
        >,
    >,
    pub m_SubRenderTargetMode: crate::UnityEngine::UIElements::VisualElement_RenderTargetMode,
    pub m_defaultMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub m_RunningAnimations: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::Experimental::IValueAnimationUpdate,
        >,
    >,
    pub m_NextParentCachedVersion: u32,
    pub m_NextParentRequiredVersion: u32,
    pub m_CachedNextParentWithEventCallback: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_EventCallbackCategories: i32,
    pub m_CachedEventCallbackParentCategories: i32,
    pub m_DefaultActionEventCategories: i32,
    pub m_DefaultActionAtTargetEventCategories: i32,
    pub _hierarchy_k__BackingField: crate::UnityEngine::UIElements::VisualElement_Hierarchy,
    pub _isRootVisualContainer_k__BackingField: bool,
    pub _cacheAsBitmap_k__BackingField: bool,
    pub m_PhysicalParent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_LogicalParent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_Children: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::VisualElement,
        >,
    >,
    pub _elementPanel_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::BaseVisualElementPanel,
    >,
    pub m_VisualTreeAssetSource: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualTreeAsset,
    >,
    pub inlineStyleAccess: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::InlineStyleAccess,
    >,
    pub styleSheetList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::StyleSheet,
        >,
    >,
    pub m_TypeData: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement_TypeData,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::VisualElement =>
    "UnityEngine.UIElements"."VisualElement"
);
#[cfg(feature = "UnityEngine+UIElements+VisualElement")]
impl std::ops::Deref for crate::UnityEngine::UIElements::VisualElement {
    type Target = crate::UnityEngine::UIElements::Focusable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::VisualElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement")]
impl crate::UnityEngine::UIElements::VisualElement {
    pub const k_RootVisualContainerName: &'static str = "rootVisualContainer";
    #[cfg(
        feature = "UnityEngine+UIElements+VisualElement+BaseVisualElementScheduledItem"
    )]
    pub type BaseVisualElementScheduledItem = crate::UnityEngine::UIElements::VisualElement_BaseVisualElementScheduledItem;
    #[cfg(feature = "UnityEngine+UIElements+VisualElement+CustomStyleAccess")]
    pub type CustomStyleAccess = crate::UnityEngine::UIElements::VisualElement_CustomStyleAccess;
    #[cfg(feature = "UnityEngine+UIElements+VisualElement+Hierarchy")]
    pub type Hierarchy = crate::UnityEngine::UIElements::VisualElement_Hierarchy;
    #[cfg(feature = "UnityEngine+UIElements+VisualElement+MeasureMode")]
    pub type MeasureMode = crate::UnityEngine::UIElements::VisualElement_MeasureMode;
    #[cfg(feature = "UnityEngine+UIElements+VisualElement+RenderTargetMode")]
    pub type RenderTargetMode = crate::UnityEngine::UIElements::VisualElement_RenderTargetMode;
    #[cfg(feature = "UnityEngine+UIElements+VisualElement+SimpleScheduledItem")]
    pub type SimpleScheduledItem = crate::UnityEngine::UIElements::VisualElement_SimpleScheduledItem;
    #[cfg(feature = "UnityEngine+UIElements+VisualElement+TimerStateScheduledItem")]
    pub type TimerStateScheduledItem = crate::UnityEngine::UIElements::VisualElement_TimerStateScheduledItem;
    #[cfg(feature = "UnityEngine+UIElements+VisualElement+TypeData")]
    pub type TypeData = crate::UnityEngine::UIElements::VisualElement_TypeData;
    #[cfg(feature = "UnityEngine+UIElements+VisualElement+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::VisualElement_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+VisualElement+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::VisualElement_UxmlTraits;
    #[cfg(feature = "UnityEngine+UIElements+VisualElement+VisualElementScheduledItem_1")]
    pub type VisualElementScheduledItem_1<ActionType: quest_hook::libil2cpp::Type> = crate::UnityEngine::UIElements::VisualElement_VisualElementScheduledItem_1<
        ActionType,
    >;
    pub fn Add(
        &mut self,
        child: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (child))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddStyleSheetPath(
        &mut self,
        sheetPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddStyleSheetPath", (sheetPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddToClassList(
        &mut self,
        className: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddToClassList", (className))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssignMeasureFunction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AssignMeasureFunction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AssignStyleValues(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        src: crate::UnityEngine::UIElements::Experimental::StyleValues,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssignStyleValues", (ve, src))?;
        Ok(__cordl_ret.into())
    }
    pub fn BringToFront(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BringToFront", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateConservativeRect(
        matrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        rect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_ret: crate::UnityEngine::Rect = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateConservativeRect", (matrix, rect))?;
        Ok(__cordl_ret.into())
    }
    pub fn ChangeIMGUIContainerCount(
        &mut self,
        delta: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChangeIMGUIContainerCount", (delta))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckUserKeyArgument(
        key: crate::UnityEngine::PropertyName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckUserKeyArgument", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn Children(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::UIElements::VisualElement,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::UIElements::VisualElement,
            >,
        > = __cordl_object.invoke("Children", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClassListContains(
        &mut self,
        cls: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ClassListContains", (cls))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearManualLayout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearManualLayout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineClipRects(
        &mut self,
        rect: crate::UnityEngine::Rect,
        parentRect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("CombineClipRects", (rect, parentRect))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeAAAlignedBound(
        position: crate::UnityEngine::Rect,
        mat: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_ret: crate::UnityEngine::Rect = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ComputeAAAlignedBound", (position, mat))?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains(
        &mut self,
        child: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (child))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContainsPoint(
        &mut self,
        localPoint: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ContainsPoint", (localPoint))?;
        Ok(__cordl_ret.into())
    }
    pub fn DirtyNextParentWithEventCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DirtyNextParentWithEventCallback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DoMeasure(
        &mut self,
        desiredWidth: f32,
        widthMode: crate::UnityEngine::UIElements::VisualElement_MeasureMode,
        desiredHeight: f32,
        heightMode: crate::UnityEngine::UIElements::VisualElement_MeasureMode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("DoMeasure", (desiredWidth, widthMode, desiredHeight, heightMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn ElementAt(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("ElementAt", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn ElementAtTreePath(
        &mut self,
        childIndexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("ElementAtTreePath", (childIndexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnableInClassList(
        &mut self,
        className: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        enable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnableInClassList", (className, enable))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureWorldTransformAndClipUpToDate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureWorldTransformAndClipUpToDate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteDefaultAction(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteDefaultAction", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn FinalizeLayout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinalizeLayout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FindCommonAncestor(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("FindCommonAncestor", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindElementInTree(
        &mut self,
        element: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        outChildIndexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("FindElementInTree", (element, outChildIndexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn Focus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Focus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GatherAllChildren(
        &mut self,
        elements: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::UIElements::VisualElement,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GatherAllChildren", (elements))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAnimationSystem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElementAnimationSystem,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElementAnimationSystem,
        > = __cordl_object.invoke("GetAnimationSystem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCachedNextParentWithEventCallback(
        &mut self,
        nextParent: quest_hook::libil2cpp::ByRefMut<
            *mut crate::UnityEngine::UIElements::VisualElement,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetCachedNextParentWithEventCallback", (nextParent))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetClassesForIteration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = __cordl_object.invoke("GetClassesForIteration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFirstAncestorOfType<T>(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("GetFirstAncestorOfType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFirstAncestorWhere(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<
            crate::System::Predicate_1<
                *mut crate::UnityEngine::UIElements::VisualElement,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("GetFirstAncestorWhere", (predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFirstOfType<T>(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("GetFirstOfType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFullHierarchicalViewDataKey_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetFullHierarchicalViewDataKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFullHierarchicalViewDataKey_StringBuilder0(
        &mut self,
        key: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetFullHierarchicalViewDataKey", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextElementDepthFirst(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("GetNextElementDepthFirst", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrCreateTypeData(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement_TypeData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement_TypeData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOrCreateTypeData", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrCreateViewData<T>(
        &mut self,
        existing: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("GetOrCreateViewData", (existing, key))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParentSizeForLengthConversion(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        subPropertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<f32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<f32> = __cordl_object
            .invoke("GetParentSizeForLengthConversion", (id, subPropertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPivotedMatrixWithLayout(
        &mut self,
        result: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetPivotedMatrixWithLayout", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPreviousElementDepthFirst(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("GetPreviousElementDepthFirst", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProperty(
        &mut self,
        key: crate::UnityEngine::PropertyName,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetProperty", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRoot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("GetRoot", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRootVisualContainer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("GetRootVisualContainer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStylePropertyAnimationSystem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IStylePropertyAnimationSystem,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IStylePropertyAnimationSystem,
        > = __cordl_object.invoke("GetStylePropertyAnimationSystem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTooltipRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("GetTooltipRect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasChangedPanel(
        &mut self,
        prevPanel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseVisualElementPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HasChangedPanel", (prevPanel))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasDefaultAction(
        &mut self,
        eventCategory: crate::UnityEngine::UIElements::EventCategory,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasDefaultAction", (eventCategory))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasEventCallbacks(
        &mut self,
        eventCategory: crate::UnityEngine::UIElements::EventCategory,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasEventCallbacks", (eventCategory))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasEventCallbacksOrDefaultActionAtTarget(
        &mut self,
        eventCategory: crate::UnityEngine::UIElements::EventCategory,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasEventCallbacksOrDefaultActionAtTarget", (eventCategory))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasEventCallbacksOrDefaultActions(
        &mut self,
        eventCategory: crate::UnityEngine::UIElements::EventCategory,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasEventCallbacksOrDefaultActions", (eventCategory))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasParentEventCallbacksOrDefaultActionAtTarget(
        &mut self,
        eventCategory: crate::UnityEngine::UIElements::EventCategory,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasParentEventCallbacksOrDefaultActionAtTarget", (eventCategory))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasParentEventCallbacksOrDefaultActions(
        &mut self,
        eventCategory: crate::UnityEngine::UIElements::EventCategory,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasParentEventCallbacksOrDefaultActions", (eventCategory))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasProperty(
        &mut self,
        key: crate::UnityEngine::PropertyName,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasProperty", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn IncrementVersion(
        &mut self,
        changeType: crate::UnityEngine::UIElements::VersionChangeType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncrementVersion", (changeType))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("IndexOf", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn Insert(
        &mut self,
        index: i32,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Insert", (index, element))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeGenerateVisualContent(
        &mut self,
        mgc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MeshGenerationContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeGenerateVisualContent", (mgc))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeHierarchyChanged(
        &mut self,
        changeType: crate::UnityEngine::UIElements::HierarchyChangeType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeHierarchyChanged", (changeType))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPartOfCapturedChain(
        _cordl_self: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        capturingElement: quest_hook::libil2cpp::ByRefMut<
            *mut crate::UnityEngine::UIElements::IEventHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPartOfCapturedChain", (_cordl_self, capturingElement))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsViewDataPersitenceSupportedOnChildren(
        &mut self,
        existingState: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsViewDataPersitenceSupportedOnChildren", (existingState))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkDirtyRepaint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkDirtyRepaint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkRenderHintsClean(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkRenderHintsClean", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Max(a: f32, b: f32, c: f32, d: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (a, b, c, d))?;
        Ok(__cordl_ret.into())
    }
    pub fn Measure(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
        width: f32,
        widthMode: crate::UnityEngine::Yoga::YogaMeasureMode,
        height: f32,
        heightMode: crate::UnityEngine::Yoga::YogaMeasureMode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Yoga::YogaSize> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Yoga::YogaSize = __cordl_object
            .invoke("Measure", (node, width, widthMode, height, heightMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min(a: f32, b: f32, c: f32, d: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (a, b, c, d))?;
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyMatrix34(
        lhs: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        rhs: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        res: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MultiplyMatrix34", (lhs, rhs, res))?;
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyMatrix44Point2(
        lhs: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        point: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MultiplyMatrix44Point2", (lhs, point))?;
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyVector2(
        lhs: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        vector: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MultiplyVector2", (lhs, vector))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnViewDataReady_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnViewDataReady", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnViewDataReady__cordl_bool0(
        &mut self,
        enablePersistence: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnViewDataReady", (enablePersistence))?;
        Ok(__cordl_ret.into())
    }
    pub fn OrderMinMaxRect(
        rect: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OrderMinMaxRect", (rect))?;
        Ok(__cordl_ret.into())
    }
    pub fn OverwriteFromViewData(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OverwriteFromViewData", (obj, key))?;
        Ok(__cordl_ret.into())
    }
    pub fn PlaceBehind(
        &mut self,
        sibling: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlaceBehind", (sibling))?;
        Ok(__cordl_ret.into())
    }
    pub fn PropagateCachedNextParentWithEventCallback(
        &mut self,
        nextParent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        stopParent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "PropagateCachedNextParentWithEventCallback",
                (nextParent, stopParent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn PropagateEnabledToChildren(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PropagateEnabledToChildren", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadCurrentValues(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        targetValuesToRead: crate::UnityEngine::UIElements::Experimental::StyleValues,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Experimental::StyleValues,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::Experimental::StyleValues = __cordl_object
            .invoke("ReadCurrentValues", (ve, targetValuesToRead))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterAnimation(
        &mut self,
        anim: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Experimental::IValueAnimationUpdate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterAnimation", (anim))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterRunningAnimations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterRunningAnimations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveFromClassList(
        &mut self,
        className: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveFromClassList", (className))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveFromHierarchy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveFromHierarchy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveMeasureFunction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveMeasureFunction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveLengthValue(
        &mut self,
        length: crate::UnityEngine::UIElements::Length,
        isRow: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleFloat> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleFloat = __cordl_object
            .invoke("ResolveLengthValue", (length, isRow))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("ResolveRotation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveScale(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("ResolveScale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveTransformOrigin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("ResolveTransformOrigin", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveTranslate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("ResolveTranslate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RetargetElement(
        &mut self,
        retargetAgainst: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("RetargetElement", (retargetAgainst))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveViewData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveViewData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SendEvent_DispatchMode1(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
        dispatchMode: crate::UnityEngine::UIElements::DispatchMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendEvent", (e, dispatchMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendEvent_EventBase0(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendEvent", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendToBack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendToBack", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAsNextParentWithEventCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAsNextParentWithEventCallback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetComputedStyle(
        &mut self,
        newStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetComputedStyle", (newStyle))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetEnabled", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetEnabledFromHierarchyPrivate(
        &mut self,
        state: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetEnabledFromHierarchyPrivate", (state))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetInlineRule(
        &mut self,
        sheet: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleSheet>,
        rule: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleRule>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInlineRule", (sheet, rule))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPanel(
        &mut self,
        p: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseVisualElementPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPanel", (p))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetProperty(
        &mut self,
        key: crate::UnityEngine::PropertyName,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetProperty", (key, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPropertyInternal(
        &mut self,
        key: crate::UnityEngine::PropertyName,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPropertyInternal", (key, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTooltip(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TooltipEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTooltip", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldClip(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ShouldClip", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
        fromValueGetter: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut crate::UnityEngine::UIElements::VisualElement,
                crate::UnityEngine::UIElements::Experimental::StyleValues,
            >,
        >,
        to: crate::UnityEngine::UIElements::Experimental::StyleValues,
        durationMs: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Experimental::ValueAnimation_1<
                crate::UnityEngine::UIElements::Experimental::StyleValues,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Experimental::ValueAnimation_1<
                crate::UnityEngine::UIElements::Experimental::StyleValues,
            >,
        > = __cordl_object.invoke("Start", (fromValueGetter, to, durationMs))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartAnimation<T>(
        anim: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Experimental::ValueAnimation_1<T>,
        >,
        fromValueGetter: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<*mut crate::UnityEngine::UIElements::VisualElement, T>,
        >,
        to: T,
        durationMs: i32,
        onValueChanged: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::UnityEngine::UIElements::VisualElement,
                T,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Experimental::ValueAnimation_1<T>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Experimental::ValueAnimation_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "StartAnimation",
                (anim, fromValueGetter, to, durationMs, onValueChanged),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SubstractBorderPadding(
        &mut self,
        worldRect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("SubstractBorderPadding", (worldRect))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TransformAlignedRect(
        matrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        rect: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TransformAlignedRect", (matrix, rect))?;
        Ok(__cordl_ret.into())
    }
    pub fn TransformAlignedRectToParentSpace(
        &mut self,
        rect: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TransformAlignedRectToParentSpace", (rect))?;
        Ok(__cordl_ret.into())
    }
    pub fn TranslateMatrix34(
        lhs: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        rhs: crate::UnityEngine::Vector3,
        res: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TranslateMatrix34", (lhs, rhs, res))?;
        Ok(__cordl_ret.into())
    }
    pub fn TranslateMatrix34InPlace(
        lhs: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        rhs: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TranslateMatrix34InPlace", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryConvertBackgroundSizeUnits(
        &mut self,
        from: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::BackgroundSize,
        >,
        to: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::BackgroundSize,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryConvertBackgroundSizeUnits", (from, to))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryConvertLengthUnits(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::Length>,
        to: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::Length>,
        subPropertyIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryConvertLengthUnits", (id, from, to, subPropertyIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryConvertTransformOriginUnits(
        &mut self,
        from: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::TransformOrigin,
        >,
        to: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::TransformOrigin,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryConvertTransformOriginUnits", (from, to))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryConvertTranslateUnits(
        &mut self,
        from: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::Translate>,
        to: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::Translate>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryConvertTranslateUnits", (from, to))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetPropertyInternal(
        &mut self,
        key: crate::UnityEngine::PropertyName,
        value: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetPropertyInternal", (key, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_Experimental_ITransitionAnimations_Start(
        &mut self,
        to: crate::UnityEngine::UIElements::Experimental::StyleValues,
        durationMs: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Experimental::ValueAnimation_1<
                crate::UnityEngine::UIElements::Experimental::StyleValues,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Experimental::ValueAnimation_1<
                crate::UnityEngine::UIElements::Experimental::StyleValues,
            >,
        > = __cordl_object
            .invoke(
                "UnityEngine.UIElements.Experimental.ITransitionAnimations.Start",
                (to, durationMs),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IExperimentalFeatures_get_animation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Experimental::ITransitionAnimations,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Experimental::ITransitionAnimations,
        > = __cordl_object
            .invoke("UnityEngine.UIElements.IExperimentalFeatures.get_animation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_backgroundColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_backgroundColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_borderBottomColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_borderBottomColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_borderBottomLeftRadius(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IResolvedStyle.get_borderBottomLeftRadius",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_borderBottomRightRadius(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IResolvedStyle.get_borderBottomRightRadius",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_borderBottomWidth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_borderBottomWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_borderLeftColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_borderLeftColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_borderLeftWidth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_borderLeftWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_borderRightColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_borderRightColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_borderRightWidth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_borderRightWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_borderTopColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_borderTopColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_borderTopLeftRadius(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IResolvedStyle.get_borderTopLeftRadius",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_borderTopRightRadius(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IResolvedStyle.get_borderTopRightRadius",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_borderTopWidth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_borderTopWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_bottom(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_bottom", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_color", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_display(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::DisplayStyle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::DisplayStyle = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_display", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_flexDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::FlexDirection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::FlexDirection = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_flexDirection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_flexGrow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_flexGrow", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_flexShrink(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_flexShrink", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_height(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_height", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_left(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_left", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_marginBottom(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_marginBottom", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_marginLeft(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_marginLeft", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_marginRight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_marginRight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_marginTop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_marginTop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_minHeight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleFloat> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleFloat = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_minHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_minWidth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleFloat> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleFloat = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_minWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_opacity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_opacity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_paddingBottom(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_paddingBottom", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_paddingLeft(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_paddingLeft", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_paddingRight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_paddingRight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_paddingTop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_paddingTop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_right(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_right", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_scale(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Scale> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::Scale = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_scale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_top(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_top", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_transformOrigin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_transformOrigin", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_translate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_translate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_unityBackgroundImageTintColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IResolvedStyle.get_unityBackgroundImageTintColor",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_unitySliceLeft(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_unitySliceLeft", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_unitySliceRight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_unitySliceRight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_unitySliceScale(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_unitySliceScale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_unityTextOutlineColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IResolvedStyle.get_unityTextOutlineColor",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_unityTextOutlineWidth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IResolvedStyle.get_unityTextOutlineWidth",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_visibility(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Visibility> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::Visibility = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_visibility", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IResolvedStyle_get_width(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("UnityEngine.UIElements.IResolvedStyle.get_width", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStylePropertyAnimations_CancelAllAnimations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStylePropertyAnimations.CancelAllAnimations",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStylePropertyAnimations_CancelAnimation(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStylePropertyAnimations.CancelAnimation",
                (id),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStylePropertyAnimations_GetAllAnimations(
        &mut self,
        outPropertyIds: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStylePropertyAnimations.GetAllAnimations",
                (outPropertyIds),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStylePropertyAnimations_StartEnum(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: i32,
        to: i32,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStylePropertyAnimations.StartEnum",
                (id, from, to, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStylePropertyAnimations_Start_BackgroundPosition_BackgroundPosition12(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::BackgroundPosition,
        to: crate::UnityEngine::UIElements::BackgroundPosition,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStylePropertyAnimations.Start",
                (id, from, to, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStylePropertyAnimations_Start_BackgroundRepeat_BackgroundRepeat13(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::BackgroundRepeat,
        to: crate::UnityEngine::UIElements::BackgroundRepeat,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStylePropertyAnimations.Start",
                (id, from, to, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStylePropertyAnimations_Start_BackgroundSize_BackgroundSize14(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::BackgroundSize,
        to: crate::UnityEngine::UIElements::BackgroundSize,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStylePropertyAnimations.Start",
                (id, from, to, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStylePropertyAnimations_Start_Background_Background4(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::Background,
        to: crate::UnityEngine::UIElements::Background,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStylePropertyAnimations.Start",
                (id, from, to, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStylePropertyAnimations_Start_Color_Color3(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::Color,
        to: crate::UnityEngine::Color,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStylePropertyAnimations.Start",
                (id, from, to, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStylePropertyAnimations_Start_FontDefinition_FontDefinition5(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::FontDefinition,
        to: crate::UnityEngine::UIElements::FontDefinition,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStylePropertyAnimations.Start",
                (id, from, to, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStylePropertyAnimations_Start_Font_Font6(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
        to: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStylePropertyAnimations.Start",
                (id, from, to, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStylePropertyAnimations_Start_Length_Length2(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::Length,
        to: crate::UnityEngine::UIElements::Length,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStylePropertyAnimations.Start",
                (id, from, to, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStylePropertyAnimations_Start_Rotate_Rotate10(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::Rotate,
        to: crate::UnityEngine::UIElements::Rotate,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStylePropertyAnimations.Start",
                (id, from, to, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStylePropertyAnimations_Start_Scale_Scale8(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::Scale,
        to: crate::UnityEngine::UIElements::Scale,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStylePropertyAnimations.Start",
                (id, from, to, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStylePropertyAnimations_Start_TextShadow_TextShadow7(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::TextShadow,
        to: crate::UnityEngine::UIElements::TextShadow,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStylePropertyAnimations.Start",
                (id, from, to, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStylePropertyAnimations_Start_TransformOrigin_TransformOrigin11(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::TransformOrigin,
        to: crate::UnityEngine::UIElements::TransformOrigin,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStylePropertyAnimations.Start",
                (id, from, to, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStylePropertyAnimations_Start_Translate_Translate9(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::Translate,
        to: crate::UnityEngine::UIElements::Translate,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStylePropertyAnimations.Start",
                (id, from, to, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStylePropertyAnimations_Start_f32_f32_0(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: f32,
        to: f32,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStylePropertyAnimations.Start",
                (id, from, to, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStylePropertyAnimations_Start_i32_i32_1(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: i32,
        to: i32,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStylePropertyAnimations.Start",
                (id, from, to, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStylePropertyAnimations_UpdateAnimation(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStylePropertyAnimations.UpdateAnimation",
                (id),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStylePropertyAnimations_get_completedAnimationCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStylePropertyAnimations.get_completedAnimationCount",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStylePropertyAnimations_get_runningAnimationCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStylePropertyAnimations.get_runningAnimationCount",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStylePropertyAnimations_set_completedAnimationCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStylePropertyAnimations.set_completedAnimationCount",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStylePropertyAnimations_set_runningAnimationCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStylePropertyAnimations.set_runningAnimationCount",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITransform_get_position(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("UnityEngine.UIElements.ITransform.get_position", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITransform_get_scale(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("UnityEngine.UIElements.ITransform.get_scale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITransform_set_position(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.ITransform.set_position", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IVisualElementScheduler_Execute_Action1(
        &mut self,
        updateEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IVisualElementScheduledItem,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IVisualElementScheduledItem,
        > = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IVisualElementScheduler.Execute",
                (updateEvent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IVisualElementScheduler_Execute_Action_1_0(
        &mut self,
        timerUpdateEvent: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::UIElements::TimerState>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IVisualElementScheduledItem,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IVisualElementScheduledItem,
        > = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IVisualElementScheduler.Execute",
                (timerUpdateEvent),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterAnimation(
        &mut self,
        anim: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Experimental::IValueAnimationUpdate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterAnimation", (anim))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterRunningAnimations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterRunningAnimations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateBoundingBox(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateBoundingBox", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateCallbackParentCategories(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateCallbackParentCategories", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateCursorStyle(
        &mut self,
        eventType: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateCursorStyle", (eventType))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateHoverPseudoState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateHoverPseudoState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateWorldBoundingBox(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateWorldBoundingBox", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateWorldClip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateWorldClip", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateWorldTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateWorldTransform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateWorldTransformInverse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateWorldTransformInverse", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WillChangePanel(
        &mut self,
        destinationPanel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseVisualElementPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WillChangePanel", (destinationPanel))?;
        Ok(__cordl_ret.into())
    }
    pub fn _AssignMeasureFunction_b__432_0(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
        f: f32,
        mode: crate::UnityEngine::Yoga::YogaMeasureMode,
        f1: f32,
        heightMode: crate::UnityEngine::Yoga::YogaMeasureMode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Yoga::YogaSize> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Yoga::YogaSize = __cordl_object
            .invoke("<AssignMeasureFunction>b__432_0", (node, f, mode, f1, heightMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        key: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("get_Item", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_boundingBox(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_boundingBox", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_boundingBoxInParentSpace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_boundingBoxInParentSpace", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_canGrabFocus(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_canGrabFocus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_childCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_childCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_classList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = __cordl_object.invoke("get_classList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_computedStyle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::ComputedStyle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        > = __cordl_object.invoke("get_computedStyle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_containedPointerIds(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_containedPointerIds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_contentContainer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("get_contentContainer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_contentRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_contentRect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_customStyle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ICustomStyle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ICustomStyle,
        > = __cordl_object.invoke("get_customStyle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = __cordl_object
            .invoke("get_defaultMaterial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_disableClipping(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_disableClipping", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_elementPanel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::BaseVisualElementPanel>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseVisualElementPanel,
        > = __cordl_object.invoke("get_elementPanel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enableViewDataPersistence(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_enableViewDataPersistence", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enabledInHierarchy(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enabledInHierarchy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enabledSelf(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enabledSelf", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_eventCallbackCategories(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_eventCallbackCategories", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_eventCallbackParentCategories(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_eventCallbackParentCategories", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_experimental(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IExperimentalFeatures>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IExperimentalFeatures,
        > = __cordl_object.invoke("get_experimental", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_focusController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::FocusController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::FocusController,
        > = __cordl_object.invoke("get_focusController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fullTypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_fullTypeName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_generateVisualContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::UIElements::MeshGenerationContext,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::UIElements::MeshGenerationContext,
            >,
        > = __cordl_object.invoke("get_generateVisualContent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasCompletedAnimations(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasCompletedAnimations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasDefaultRotationAndScale(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_hasDefaultRotationAndScale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasInlineStyle(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasInlineStyle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasRunningAnimations(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasRunningAnimations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hierarchy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::VisualElement_Hierarchy,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::VisualElement_Hierarchy = __cordl_object
            .invoke("get_hierarchy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isBoundingBoxDirty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isBoundingBoxDirty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isCompositeRoot(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isCompositeRoot", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isEventCallbackParentCategoriesDirty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_isEventCallbackParentCategoriesDirty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isHierarchyDisplayed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isHierarchyDisplayed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isLayoutManual(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isLayoutManual", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isParentEnabledInHierarchy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_isParentEnabledInHierarchy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isRootVisualContainer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isRootVisualContainer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isWorldBoundingBoxOrDependenciesDirty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_isWorldBoundingBoxOrDependenciesDirty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isWorldClipDirty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isWorldClipDirty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isWorldTransformDirty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isWorldTransformDirty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isWorldTransformInverseOrDependenciesDirty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_isWorldTransformInverseOrDependenciesDirty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_languageDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::LanguageDirection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::LanguageDirection = __cordl_object
            .invoke("get_languageDirection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_layout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_layout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_localBound(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_localBound", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_localLanguageDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::LanguageDirection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::LanguageDirection = __cordl_object
            .invoke("get_localLanguageDirection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_nextParentWithEventCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("get_nextParentWithEventCallback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_paddingRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_paddingRect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_panel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IPanel,
        > = __cordl_object.invoke("get_panel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_parent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("get_parent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pickingMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::PickingMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::PickingMode = __cordl_object
            .invoke("get_pickingMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_positionWithLayout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_positionWithLayout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pseudoStates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::PseudoStates> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::PseudoStates = __cordl_object
            .invoke("get_pseudoStates", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_rect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_renderHints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::RenderHints> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::RenderHints = __cordl_object
            .invoke("get_renderHints", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_requireMeasureFunction(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_requireMeasureFunction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_resolvedStyle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IResolvedStyle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IResolvedStyle,
        > = __cordl_object.invoke("get_resolvedStyle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scaledPixelsPerPoint(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_scaledPixelsPerPoint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_schedule(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IVisualElementScheduler,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IVisualElementScheduler,
        > = __cordl_object.invoke("get_schedule", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_style(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IStyle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IStyle,
        > = __cordl_object.invoke("get_style", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_styleAnimation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IStylePropertyAnimations,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IStylePropertyAnimations,
        > = __cordl_object.invoke("get_styleAnimation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_styleInitialized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_styleInitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_styleSheets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::VisualElementStyleSheetSet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::VisualElementStyleSheetSet = __cordl_object
            .invoke("get_styleSheets", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_subRenderTargetMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::VisualElement_RenderTargetMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::VisualElement_RenderTargetMode = __cordl_object
            .invoke("get_subRenderTargetMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tooltip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_tooltip", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_transform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ITransform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ITransform,
        > = __cordl_object.invoke("get_transform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_typeData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement_TypeData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement_TypeData,
        > = __cordl_object.invoke("get_typeData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_typeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_typeName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_usageHints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UsageHints> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::UsageHints = __cordl_object
            .invoke("get_usageHints", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_userData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_userData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_viewDataKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_viewDataKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_visible(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_visible", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_worldBound(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_worldBound", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_worldBoundingBox(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_worldBoundingBox", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_worldClip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_worldClip", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_worldClipIsInfinite(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_worldClipIsInfinite", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_worldClipMinusGroup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_worldClipMinusGroup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_worldTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = __cordl_object
            .invoke("get_worldTransform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_worldTransformInverse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Matrix4x4,
        > = __cordl_object.invoke("get_worldTransformInverse", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_worldTransformRef(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Matrix4x4,
        > = __cordl_object.invoke("get_worldTransformRef", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yogaNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode> = __cordl_object
            .invoke("get_yogaNode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_containedPointerIds(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_containedPointerIds", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_disableClipping(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_disableClipping", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_elementPanel(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseVisualElementPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_elementPanel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_enableViewDataPersistence(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_enableViewDataPersistence", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_enabledSelf(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_enabledSelf", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_eventCallbackCategories(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_eventCallbackCategories", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_generateVisualContent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::UIElements::MeshGenerationContext,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_generateVisualContent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_hierarchy(
        &mut self,
        value: crate::UnityEngine::UIElements::VisualElement_Hierarchy,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hierarchy", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isBoundingBoxDirty(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isBoundingBoxDirty", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isCompositeRoot(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isCompositeRoot", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isEventCallbackParentCategoriesDirty(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isEventCallbackParentCategoriesDirty", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isHierarchyDisplayed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isHierarchyDisplayed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isLayoutManual(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isLayoutManual", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isRootVisualContainer(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isRootVisualContainer", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isWorldBoundingBoxDirty(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isWorldBoundingBoxDirty", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isWorldClipDirty(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isWorldClipDirty", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isWorldTransformDirty(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isWorldTransformDirty", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isWorldTransformInverseDirty(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isWorldTransformInverseDirty", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_layout(
        &mut self,
        value: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_layout", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_localLanguageDirection(
        &mut self,
        value: crate::UnityEngine::UIElements::LanguageDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_localLanguageDirection", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_name(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_name", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_pickingMode(
        &mut self,
        value: crate::UnityEngine::UIElements::PickingMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_pickingMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_pseudoStates(
        &mut self,
        value: crate::UnityEngine::UIElements::PseudoStates,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_pseudoStates", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_renderHints(
        &mut self,
        value: crate::UnityEngine::UIElements::RenderHints,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_renderHints", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_requireMeasureFunction(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_requireMeasureFunction", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_styleInitialized(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_styleInitialized", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_tooltip(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_tooltip", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_usageHints(
        &mut self,
        value: crate::UnityEngine::UIElements::UsageHints,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_usageHints", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_userData(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_userData", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_viewDataKey(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_viewDataKey", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_visible(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_visible", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_visualTreeAssetSource(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualTreeAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_visualTreeAssetSource", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_yogaNode(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_yogaNode", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement")]
impl AsRef<crate::UnityEngine::UIElements::Experimental::ITransitionAnimations>
for crate::UnityEngine::UIElements::VisualElement {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::UIElements::Experimental::ITransitionAnimations {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement")]
impl AsMut<crate::UnityEngine::UIElements::Experimental::ITransitionAnimations>
for crate::UnityEngine::UIElements::VisualElement {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::UIElements::Experimental::ITransitionAnimations {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement")]
impl AsRef<crate::UnityEngine::UIElements::IExperimentalFeatures>
for crate::UnityEngine::UIElements::VisualElement {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IExperimentalFeatures {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement")]
impl AsMut<crate::UnityEngine::UIElements::IExperimentalFeatures>
for crate::UnityEngine::UIElements::VisualElement {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::IExperimentalFeatures {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement")]
impl AsRef<crate::UnityEngine::UIElements::IResolvedStyle>
for crate::UnityEngine::UIElements::VisualElement {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IResolvedStyle {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement")]
impl AsMut<crate::UnityEngine::UIElements::IResolvedStyle>
for crate::UnityEngine::UIElements::VisualElement {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::IResolvedStyle {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement")]
impl AsRef<crate::UnityEngine::UIElements::IStylePropertyAnimations>
for crate::UnityEngine::UIElements::VisualElement {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IStylePropertyAnimations {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement")]
impl AsMut<crate::UnityEngine::UIElements::IStylePropertyAnimations>
for crate::UnityEngine::UIElements::VisualElement {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::UIElements::IStylePropertyAnimations {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement")]
impl AsRef<crate::UnityEngine::UIElements::ITransform>
for crate::UnityEngine::UIElements::VisualElement {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::ITransform {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement")]
impl AsMut<crate::UnityEngine::UIElements::ITransform>
for crate::UnityEngine::UIElements::VisualElement {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::ITransform {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement")]
impl AsRef<crate::UnityEngine::UIElements::IVisualElementScheduler>
for crate::UnityEngine::UIElements::VisualElement {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IVisualElementScheduler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement")]
impl AsMut<crate::UnityEngine::UIElements::IVisualElementScheduler>
for crate::UnityEngine::UIElements::VisualElement {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::UIElements::IVisualElementScheduler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+BaseVisualElementScheduledItem")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualElement_BaseVisualElementScheduledItem {
    __cordl_parent: crate::UnityEngine::UIElements::ScheduledItem,
    pub _element_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub isScheduled: bool,
    pub m_Activator: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElementPanelActivator,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+BaseVisualElementScheduledItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualElement_BaseVisualElementScheduledItem =>
    "UnityEngine.UIElements"."VisualElement/BaseVisualElementScheduledItem"
);
#[cfg(feature = "UnityEngine+UIElements+VisualElement+BaseVisualElementScheduledItem")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::VisualElement_BaseVisualElementScheduledItem {
    type Target = crate::UnityEngine::UIElements::ScheduledItem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+BaseVisualElementScheduledItem")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::VisualElement_BaseVisualElementScheduledItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+BaseVisualElementScheduledItem")]
impl crate::UnityEngine::UIElements::VisualElement_BaseVisualElementScheduledItem {
    pub fn CanBeActivated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanBeActivated", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Every(
        &mut self,
        intervalMs: i64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IVisualElementScheduledItem,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IVisualElementScheduledItem,
        > = __cordl_object.invoke("Every", (intervalMs))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteLater(
        &mut self,
        delayMs: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteLater", (delayMs))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (handler))?;
        Ok(__cordl_object.into())
    }
    pub fn OnItemUnscheduled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnItemUnscheduled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPanelActivate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPanelActivate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPanelDeactivate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPanelDeactivate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Pause(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Pause", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Resume(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Resume", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartingIn(
        &mut self,
        delayMs: i64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IVisualElementScheduledItem,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IVisualElementScheduledItem,
        > = __cordl_object.invoke("StartingIn", (delayMs))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        handler: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (handler))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_element(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("get_element", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_element(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_element", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+BaseVisualElementScheduledItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualElement_BaseVisualElementScheduledItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+BaseVisualElementScheduledItem")]
impl AsRef<crate::UnityEngine::UIElements::IVisualElementPanelActivatable>
for crate::UnityEngine::UIElements::VisualElement_BaseVisualElementScheduledItem {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IVisualElementPanelActivatable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+BaseVisualElementScheduledItem")]
impl AsMut<crate::UnityEngine::UIElements::IVisualElementPanelActivatable>
for crate::UnityEngine::UIElements::VisualElement_BaseVisualElementScheduledItem {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::UIElements::IVisualElementPanelActivatable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+BaseVisualElementScheduledItem")]
impl AsRef<crate::UnityEngine::UIElements::IVisualElementScheduledItem>
for crate::UnityEngine::UIElements::VisualElement_BaseVisualElementScheduledItem {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IVisualElementScheduledItem {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+BaseVisualElementScheduledItem")]
impl AsMut<crate::UnityEngine::UIElements::IVisualElementScheduledItem>
for crate::UnityEngine::UIElements::VisualElement_BaseVisualElementScheduledItem {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::UIElements::IVisualElementScheduledItem {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+CustomStyleAccess")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualElement_CustomStyleAccess {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_CustomProperties: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyValue,
        >,
    >,
    pub m_DpiScaling: f32,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+CustomStyleAccess")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualElement_CustomStyleAccess =>
    "UnityEngine.UIElements"."VisualElement/CustomStyleAccess"
);
#[cfg(feature = "UnityEngine+UIElements+VisualElement+CustomStyleAccess")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::VisualElement_CustomStyleAccess {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+CustomStyleAccess")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::VisualElement_CustomStyleAccess {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+CustomStyleAccess")]
impl crate::UnityEngine::UIElements::VisualElement_CustomStyleAccess {
    pub fn LogCustomPropertyWarning(
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        valueType: crate::UnityEngine::UIElements::StyleValueType,
        customProp: crate::UnityEngine::UIElements::StyleSheets::StylePropertyValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogCustomPropertyWarning", (propertyName, valueType, customProp))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetContext(
        &mut self,
        customProperties: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                crate::UnityEngine::UIElements::StyleSheets::StylePropertyValue,
            >,
        >,
        dpiScaling: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetContext", (customProperties, dpiScaling))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetValue_CustomStyleProperty_1_ByRefMut0(
        &mut self,
        property: crate::UnityEngine::UIElements::CustomStyleProperty_1<f32>,
        value: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetValue", (property, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetValue_CustomStyleProperty_1_ByRefMut1(
        &mut self,
        property: crate::UnityEngine::UIElements::CustomStyleProperty_1<i32>,
        value: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetValue", (property, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetValue_CustomStyleProperty_1_ByRefMut2(
        &mut self,
        property: crate::UnityEngine::UIElements::CustomStyleProperty_1<
            crate::UnityEngine::Color,
        >,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetValue", (property, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetValue_CustomStyleProperty_1_ByRefMut3(
        &mut self,
        property: crate::UnityEngine::UIElements::CustomStyleProperty_1<
            *mut crate::UnityEngine::Texture2D,
        >,
        value: quest_hook::libil2cpp::ByRefMut<*mut crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetValue", (property, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetValue_CustomStyleProperty_1_ByRefMut4(
        &mut self,
        property: crate::UnityEngine::UIElements::CustomStyleProperty_1<
            *mut crate::UnityEngine::Sprite,
        >,
        value: quest_hook::libil2cpp::ByRefMut<*mut crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetValue", (property, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetValue_CustomStyleProperty_1_ByRefMut5(
        &mut self,
        property: crate::UnityEngine::UIElements::CustomStyleProperty_1<
            *mut crate::UnityEngine::UIElements::VectorImage,
        >,
        value: quest_hook::libil2cpp::ByRefMut<
            *mut crate::UnityEngine::UIElements::VectorImage,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetValue", (property, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetValue_CustomStyleProperty_1_ByRefMut6(
        &mut self,
        property: crate::UnityEngine::UIElements::CustomStyleProperty_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        value: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetValue", (property, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetValue_Il2CppString_StyleValueType_ByRefMut7(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        valueType: crate::UnityEngine::UIElements::StyleValueType,
        customProp: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyValue,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetValue", (propertyName, valueType, customProp))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+CustomStyleAccess")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualElement_CustomStyleAccess {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+CustomStyleAccess")]
impl AsRef<crate::UnityEngine::UIElements::ICustomStyle>
for crate::UnityEngine::UIElements::VisualElement_CustomStyleAccess {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::ICustomStyle {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+CustomStyleAccess")]
impl AsMut<crate::UnityEngine::UIElements::ICustomStyle>
for crate::UnityEngine::UIElements::VisualElement_CustomStyleAccess {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::ICustomStyle {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+Hierarchy")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct VisualElement_Hierarchy {
    pub m_Owner: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+Hierarchy")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::VisualElement_Hierarchy
    => "UnityEngine.UIElements"."VisualElement/Hierarchy"
);
#[cfg(feature = "UnityEngine+UIElements+VisualElement+Hierarchy")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::VisualElement_Hierarchy {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+Hierarchy")]
impl crate::UnityEngine::UIElements::VisualElement_Hierarchy {
    pub const k_InvalidHierarchyChangeMsg: &'static str = "Cannot modify VisualElement hierarchy during layout calculation";
    pub fn Add(
        &mut self,
        child: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Add",
            (child),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn BringToFront(
        &mut self,
        child: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "BringToFront",
            (child),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Children(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::UIElements::VisualElement,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::UIElements::VisualElement,
            >,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "Children", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clear",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ElementAt(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ElementAt", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_VisualElement_Hierarchy0(
        &mut self,
        other: crate::UnityEngine::UIElements::VisualElement_Hierarchy,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IndexOf",
            (element),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Insert(
        &mut self,
        index: i32,
        child: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Insert",
            (index, child),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveChildElement(
        &mut self,
        child: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        currentIndex: i32,
        nextIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveChildElement",
            (child, currentIndex, nextIndex),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn PlaceBehind(
        &mut self,
        child: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        over: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PlaceBehind",
            (child, over),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn PutChildAtIndex(
        &mut self,
        child: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PutChildAtIndex",
            (child, index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseChildList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ReleaseChildList",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Remove(
        &mut self,
        child: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Remove",
            (child),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAt(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RemoveAt",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveChildAtIndex(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RemoveChildAtIndex",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SendToBack(
        &mut self,
        child: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SendToBack",
            (child),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetParent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetParent",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (element),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        key: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Item", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_childCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_childCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_children(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::UIElements::VisualElement,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::UIElements::VisualElement,
            >,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_children", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_parent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_parent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        x: crate::UnityEngine::UIElements::VisualElement_Hierarchy,
        y: crate::UnityEngine::UIElements::VisualElement_Hierarchy,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (x, y))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+MeasureMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualElement_MeasureMode {
    AtMost = 2i32,
    Exactly = 1i32,
    Undefined = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+MeasureMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualElement_MeasureMode => "UnityEngine.UIElements"
    ."VisualElement/MeasureMode"
);
#[cfg(feature = "UnityEngine+UIElements+VisualElement+RenderTargetMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualElement_RenderTargetMode {
    GammaToLinear = 3i32,
    LinearToGamma = 2i32,
    NoColorConversion = 1i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+RenderTargetMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualElement_RenderTargetMode => "UnityEngine.UIElements"
    ."VisualElement/RenderTargetMode"
);
#[cfg(feature = "UnityEngine+UIElements+VisualElement+SimpleScheduledItem")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualElement_SimpleScheduledItem {
    __cordl_parent: crate::UnityEngine::UIElements::VisualElement_VisualElementScheduledItem_1<
        *mut crate::System::Action,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+SimpleScheduledItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualElement_SimpleScheduledItem =>
    "UnityEngine.UIElements"."VisualElement/SimpleScheduledItem"
);
#[cfg(feature = "UnityEngine+UIElements+VisualElement+SimpleScheduledItem")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::VisualElement_SimpleScheduledItem {
    type Target = crate::UnityEngine::UIElements::VisualElement_VisualElementScheduledItem_1<
        *mut crate::System::Action,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+SimpleScheduledItem")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::VisualElement_SimpleScheduledItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+SimpleScheduledItem")]
impl crate::UnityEngine::UIElements::VisualElement_SimpleScheduledItem {
    pub fn New(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        updateEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (handler, updateEvent))?;
        Ok(__cordl_object.into())
    }
    pub fn PerformTimerUpdate(
        &mut self,
        state: crate::UnityEngine::UIElements::TimerState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PerformTimerUpdate", (state))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        updateEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (handler, updateEvent))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+SimpleScheduledItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualElement_SimpleScheduledItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+TimerStateScheduledItem")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualElement_TimerStateScheduledItem {
    __cordl_parent: crate::UnityEngine::UIElements::VisualElement_VisualElementScheduledItem_1<
        *mut crate::System::Action_1<crate::UnityEngine::UIElements::TimerState>,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+TimerStateScheduledItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualElement_TimerStateScheduledItem =>
    "UnityEngine.UIElements"."VisualElement/TimerStateScheduledItem"
);
#[cfg(feature = "UnityEngine+UIElements+VisualElement+TimerStateScheduledItem")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::VisualElement_TimerStateScheduledItem {
    type Target = crate::UnityEngine::UIElements::VisualElement_VisualElementScheduledItem_1<
        *mut crate::System::Action_1<crate::UnityEngine::UIElements::TimerState>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+TimerStateScheduledItem")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::VisualElement_TimerStateScheduledItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+TimerStateScheduledItem")]
impl crate::UnityEngine::UIElements::VisualElement_TimerStateScheduledItem {
    pub fn New(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        updateEvent: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::UIElements::TimerState>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (handler, updateEvent))?;
        Ok(__cordl_object.into())
    }
    pub fn PerformTimerUpdate(
        &mut self,
        state: crate::UnityEngine::UIElements::TimerState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PerformTimerUpdate", (state))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        updateEvent: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::UIElements::TimerState>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (handler, updateEvent))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+TimerStateScheduledItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualElement_TimerStateScheduledItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+TypeData")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualElement_TypeData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _type_k__BackingField: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub m_FullTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_TypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_TypeNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+TypeData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::VisualElement_TypeData
    => "UnityEngine.UIElements"."VisualElement/TypeData"
);
#[cfg(feature = "UnityEngine+UIElements+VisualElement+TypeData")]
impl std::ops::Deref for crate::UnityEngine::UIElements::VisualElement_TypeData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+TypeData")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::VisualElement_TypeData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+TypeData")]
impl crate::UnityEngine::UIElements::VisualElement_TypeData {
    pub fn New(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fullTypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_fullTypeName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_type", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_typeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_typeName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_typeNamespace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_typeNamespace", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+TypeData")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualElement_TypeData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualElement_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::VisualElement,
        *mut crate::UnityEngine::UIElements::VisualElement_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualElement_UxmlFactory => "UnityEngine.UIElements"
    ."VisualElement/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+VisualElement+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::VisualElement_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::VisualElement,
        *mut crate::UnityEngine::UIElements::VisualElement_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::VisualElement_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+UxmlFactory")]
impl crate::UnityEngine::UIElements::VisualElement_UxmlFactory {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualElement_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualElement_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlTraits,
    pub m_Name: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlStringAttributeDescription,
    >,
    pub m_ViewDataKey: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlStringAttributeDescription,
    >,
    pub m_PickingMode: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlEnumAttributeDescription_1<
            crate::UnityEngine::UIElements::PickingMode,
        >,
    >,
    pub m_Tooltip: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlStringAttributeDescription,
    >,
    pub m_UsageHints: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlEnumAttributeDescription_1<
            crate::UnityEngine::UIElements::UsageHints,
        >,
    >,
    pub _focusIndex_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
    >,
    pub m_TabIndex: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
    >,
    pub _focusable_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    >,
    pub m_Class: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlStringAttributeDescription,
    >,
    pub m_ContentContainer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlStringAttributeDescription,
    >,
    pub m_Style: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlStringAttributeDescription,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualElement_UxmlTraits => "UnityEngine.UIElements"
    ."VisualElement/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+VisualElement+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::VisualElement_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::UxmlTraits;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::VisualElement_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+UxmlTraits")]
impl crate::UnityEngine::UIElements::VisualElement_UxmlTraits {
    pub fn Init(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        bag: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IUxmlAttributes>,
        cc: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (ve, bag, cc))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_focusIndex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
        > = __cordl_object.invoke("get_focusIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_focusable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
        > = __cordl_object.invoke("get_focusable", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualElement_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+VisualElementScheduledItem_1")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualElement_VisualElementScheduledItem_1<
    ActionType: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::UnityEngine::UIElements::VisualElement_BaseVisualElementScheduledItem,
    pub updateEvent: ActionType,
    __cordl_phantom_ActionType: std::marker::PhantomData<ActionType>,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+VisualElementScheduledItem_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualElement_VisualElementScheduledItem_1 < ActionType >
    => "UnityEngine.UIElements"."VisualElement/VisualElementScheduledItem`1" < ActionType
    >
);
#[cfg(feature = "UnityEngine+UIElements+VisualElement+VisualElementScheduledItem_1")]
impl<ActionType: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::UIElements::VisualElement_VisualElementScheduledItem_1<
    ActionType,
> {
    type Target = crate::UnityEngine::UIElements::VisualElement_BaseVisualElementScheduledItem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+VisualElementScheduledItem_1")]
impl<ActionType: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::UIElements::VisualElement_VisualElementScheduledItem_1<
    ActionType,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+VisualElementScheduledItem_1")]
impl<
    ActionType: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::VisualElement_VisualElementScheduledItem_1<
    ActionType,
> {
    pub fn New(
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        upEvent: ActionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        ActionType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (handler, upEvent))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        handler: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        upEvent: ActionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        ActionType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (handler, upEvent))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElement+VisualElementScheduledItem_1")]
impl<ActionType: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualElement_VisualElementScheduledItem_1<
    ActionType,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
