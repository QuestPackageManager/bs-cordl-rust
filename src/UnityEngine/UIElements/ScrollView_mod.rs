#[cfg(feature = "UnityEngine+UIElements+ScrollView")]
#[repr(C)]
#[derive(Debug)]
pub struct ScrollView {
    __cordl_parent: crate::UnityEngine::UIElements::VisualElement,
    pub m_FirstLayoutPass: i32,
    pub m_HorizontalScrollerVisibility: crate::UnityEngine::UIElements::ScrollerVisibility,
    pub m_VerticalScrollerVisibility: crate::UnityEngine::UIElements::ScrollerVisibility,
    pub m_AttachedRootVisualContainer: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_SingleLineHeight: f32,
    pub m_MouseWheelScrollSizeIsInline: bool,
    pub m_HorizontalPageSize: f32,
    pub m_VerticalPageSize: f32,
    pub m_MouseWheelScrollSize: f32,
    pub m_ScrollDecelerationRate: f32,
    pub k_ScaledPixelsPerPointMultiplier: f32,
    pub k_TouchScrollInertiaBaseTimeInterval: f32,
    pub m_Elasticity: f32,
    pub m_TouchScrollBehavior: crate::UnityEngine::UIElements::ScrollView_TouchScrollBehavior,
    pub m_NestedInteractionKind: crate::UnityEngine::UIElements::ScrollView_NestedInteractionKind,
    pub m_ElasticAnimationIntervalMs: i64,
    pub _contentViewport_k__BackingField: *mut crate::UnityEngine::UIElements::VisualElement,
    pub _horizontalScroller_k__BackingField: *mut crate::UnityEngine::UIElements::Scroller,
    pub _verticalScroller_k__BackingField: *mut crate::UnityEngine::UIElements::Scroller,
    pub m_ContentContainer: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_ContentAndVerticalScrollContainer: *mut crate::UnityEngine::UIElements::VisualElement,
    pub previousVerticalTouchScrollTimeStamp: f32,
    pub previousHorizontalTouchScrollTimeStamp: f32,
    pub elapsedTimeSinceLastVerticalTouchScroll: f32,
    pub elapsedTimeSinceLastHorizontalTouchScroll: f32,
    pub m_Mode: crate::UnityEngine::UIElements::ScrollViewMode,
    pub m_ScheduledLayoutPassResetItem: *mut crate::UnityEngine::UIElements::IVisualElementScheduledItem,
    pub m_StartPosition: crate::UnityEngine::Vector2,
    pub m_PointerStartPosition: crate::UnityEngine::Vector2,
    pub m_Velocity: crate::UnityEngine::Vector2,
    pub m_SpringBackVelocity: crate::UnityEngine::Vector2,
    pub m_LowBounds: crate::UnityEngine::Vector2,
    pub m_HighBounds: crate::UnityEngine::Vector2,
    pub m_LastVelocityLerpTime: f32,
    pub m_StartedMoving: bool,
    pub m_TouchPointerMoveAllowed: bool,
    pub m_TouchStoppedVelocity: bool,
    pub m_CapturedTarget: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_CapturedTargetPointerMoveCallback: *mut crate::UnityEngine::UIElements::EventCallback_1<
        *mut crate::UnityEngine::UIElements::PointerMoveEvent,
    >,
    pub m_CapturedTargetPointerUpCallback: *mut crate::UnityEngine::UIElements::EventCallback_1<
        *mut crate::UnityEngine::UIElements::PointerUpEvent,
    >,
    pub m_PostPointerUpAnimation: *mut crate::UnityEngine::UIElements::IVisualElementScheduledItem,
}
#[cfg(feature = "UnityEngine+UIElements+ScrollView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ScrollView =>
    "UnityEngine.UIElements"."ScrollView"
);
#[cfg(feature = "UnityEngine+UIElements+ScrollView")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ScrollView {
    type Target = crate::UnityEngine::UIElements::VisualElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ScrollView")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ScrollView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ScrollView")]
impl crate::UnityEngine::UIElements::ScrollView {
    #[cfg(feature = "UnityEngine+UIElements+ScrollView+NestedInteractionKind")]
    pub type NestedInteractionKind = crate::UnityEngine::UIElements::ScrollView_NestedInteractionKind;
    #[cfg(feature = "UnityEngine+UIElements+ScrollView+TouchScrollBehavior")]
    pub type TouchScrollBehavior = crate::UnityEngine::UIElements::ScrollView_TouchScrollBehavior;
    #[cfg(feature = "UnityEngine+UIElements+ScrollView+TouchScrollingResult")]
    pub type TouchScrollingResult = crate::UnityEngine::UIElements::ScrollView_TouchScrollingResult;
    #[cfg(feature = "UnityEngine+UIElements+ScrollView+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::ScrollView_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+ScrollView+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::ScrollView_UxmlTraits;
    pub fn AdjustScrollers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AdjustScrollers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyScrollInertia(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyScrollInertia", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyTouchScrolling(
        &mut self,
        newScrollOffset: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ApplyTouchScrolling", (newScrollOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeInitialSpringBackVelocity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ComputeInitialSpringBackVelocity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeTouchScrolling(
        &mut self,
        position: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::ScrollView_TouchScrollingResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::ScrollView_TouchScrollingResult = __cordl_object
            .invoke("ComputeTouchScrolling", (position))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteElasticSpringAnimation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteElasticSpringAnimation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDeltaDistance(
        &mut self,
        viewMin: f32,
        viewMax: f32,
        childBoundaryMin: f32,
        childBoundaryMax: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "GetDeltaDistance",
                (viewMin, viewMax, childBoundaryMin, childBoundaryMax),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetXDeltaOffset(
        &mut self,
        child: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetXDeltaOffset", (child))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetYDeltaOffset(
        &mut self,
        child: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetYDeltaOffset", (child))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitTouchScrolling(
        &mut self,
        position: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitTouchScrolling", (position))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_ScrollViewMode1(
        scrollViewMode: crate::UnityEngine::UIElements::ScrollViewMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (scrollViewMode))?;
        Ok(__cordl_object.into())
    }
    pub fn OnAttachToPanel(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::AttachToPanelEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAttachToPanel", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDetachFromPanel(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::DetachFromPanelEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDetachFromPanel", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnGeometryChanged(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::GeometryChangedEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnGeometryChanged", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnHorizontalScrollDragElementChanged(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::GeometryChangedEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnHorizontalScrollDragElementChanged", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerCancel(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::PointerCancelEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerCancel", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerCapture(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::PointerCaptureEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerCapture", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerCaptureOut(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::PointerCaptureOutEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerCaptureOut", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerDown(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerDownEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerDown", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerMove(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerMoveEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerMove", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerUp(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerUpEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerUp", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnRootCustomStyleResolved(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::CustomStyleResolvedEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRootCustomStyleResolved", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnRootPointerUp(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerUpEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRootPointerUp", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnScrollWheel(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::WheelEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnScrollWheel", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnScrollersGeometryChanged(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::GeometryChangedEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnScrollersGeometryChanged", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnVerticalScrollDragElementChanged(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::GeometryChangedEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnVerticalScrollDragElementChanged", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn PostPointerUpAnimation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PostPointerUpAnimation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadSingleLineHeight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadSingleLineHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseScrolling(
        &mut self,
        pointerId: i32,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IEventHandler>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ReleaseScrolling", (pointerId, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetLayoutPass(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetLayoutPass", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleResetLayoutPass(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScheduleResetLayoutPass", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScrollTo(
        &mut self,
        child: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScrollTo", (child))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetScrollViewMode(
        &mut self,
        mode: crate::UnityEngine::UIElements::ScrollViewMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetScrollViewMode", (mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn SpringBack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SpringBack", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateContentViewTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateContentViewTransform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateElasticBehaviour(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateElasticBehaviour", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateHorizontalSliderPageSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateHorizontalSliderPageSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateScrollers(
        &mut self,
        displayHorizontal: bool,
        displayVertical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateScrollers", (displayHorizontal, displayVertical))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateVerticalSliderPageSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateVerticalSliderPageSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn __ctor_b__126_0(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<.ctor>b__126_0", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn __ctor_b__126_1(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<.ctor>b__126_1", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ScrollViewMode1(
        &mut self,
        scrollViewMode: crate::UnityEngine::UIElements::ScrollViewMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (scrollViewMode))?;
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
    pub fn get_contentViewport(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("get_contentViewport", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_elasticity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_elasticity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasInertia(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasInertia", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_horizontalScroller(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Scroller>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Scroller,
        > = __cordl_object.invoke("get_horizontalScroller", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_horizontalScrollerVisibility(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::ScrollerVisibility,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::ScrollerVisibility = __cordl_object
            .invoke("get_horizontalScrollerVisibility", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isHorizontalScrollDisplayed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_isHorizontalScrollDisplayed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isVerticalScrollDisplayed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_isVerticalScrollDisplayed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::ScrollViewMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::ScrollViewMode = __cordl_object
            .invoke("get_mode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mouseWheelScrollSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_mouseWheelScrollSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_needsHorizontal(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_needsHorizontal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_needsVertical(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_needsVertical", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_nestedInteractionKind(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::ScrollView_NestedInteractionKind,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::ScrollView_NestedInteractionKind = __cordl_object
            .invoke("get_nestedInteractionKind", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scrollDecelerationRate(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_scrollDecelerationRate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scrollOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_scrollOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scrollableHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_scrollableHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scrollableWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_scrollableWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_touchScrollBehavior(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::ScrollView_TouchScrollBehavior,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::ScrollView_TouchScrollBehavior = __cordl_object
            .invoke("get_touchScrollBehavior", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_verticalScroller(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Scroller>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Scroller,
        > = __cordl_object.invoke("get_verticalScroller", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_verticalScrollerVisibility(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::ScrollerVisibility,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::ScrollerVisibility = __cordl_object
            .invoke("get_verticalScrollerVisibility", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_elasticAnimationIntervalMs(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_elasticAnimationIntervalMs", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_elasticity(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_elasticity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_horizontalPageSize(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_horizontalPageSize", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_horizontalScrollerVisibility(
        &mut self,
        value: crate::UnityEngine::UIElements::ScrollerVisibility,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_horizontalScrollerVisibility", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_mode(
        &mut self,
        value: crate::UnityEngine::UIElements::ScrollViewMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_mode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_mouseWheelScrollSize(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_mouseWheelScrollSize", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_nestedInteractionKind(
        &mut self,
        value: crate::UnityEngine::UIElements::ScrollView_NestedInteractionKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_nestedInteractionKind", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_scrollDecelerationRate(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_scrollDecelerationRate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_scrollOffset(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_scrollOffset", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_showHorizontal(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_showHorizontal", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_showVertical(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_showVertical", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_touchScrollBehavior(
        &mut self,
        value: crate::UnityEngine::UIElements::ScrollView_TouchScrollBehavior,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_touchScrollBehavior", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_verticalPageSize(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_verticalPageSize", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_verticalScrollerVisibility(
        &mut self,
        value: crate::UnityEngine::UIElements::ScrollerVisibility,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_verticalScrollerVisibility", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+ScrollView")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::ScrollView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+ScrollView+NestedInteractionKind")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollView_NestedInteractionKind {
    Default = 0i32,
    ForwardScrolling = 2i32,
    StopScrolling = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+ScrollView+NestedInteractionKind")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::ScrollView_NestedInteractionKind =>
    "UnityEngine.UIElements"."ScrollView/NestedInteractionKind"
);
#[cfg(feature = "UnityEngine+UIElements+ScrollView+TouchScrollBehavior")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollView_TouchScrollBehavior {
    Clamped = 2i32,
    Elastic = 1i32,
    Unrestricted = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+ScrollView+TouchScrollBehavior")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::ScrollView_TouchScrollBehavior => "UnityEngine.UIElements"
    ."ScrollView/TouchScrollBehavior"
);
#[cfg(feature = "UnityEngine+UIElements+ScrollView+TouchScrollingResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollView_TouchScrollingResult {
    Apply = 0i32,
    Block = 2i32,
    Forward = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+ScrollView+TouchScrollingResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::ScrollView_TouchScrollingResult =>
    "UnityEngine.UIElements"."ScrollView/TouchScrollingResult"
);
#[cfg(feature = "UnityEngine+UIElements+ScrollView+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct ScrollView_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::ScrollView,
        *mut crate::UnityEngine::UIElements::ScrollView_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+ScrollView+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ScrollView_UxmlFactory
    => "UnityEngine.UIElements"."ScrollView/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+ScrollView+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ScrollView_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::ScrollView,
        *mut crate::UnityEngine::UIElements::ScrollView_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ScrollView+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ScrollView_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ScrollView+UxmlFactory")]
impl crate::UnityEngine::UIElements::ScrollView_UxmlFactory {
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
#[cfg(feature = "UnityEngine+UIElements+ScrollView+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ScrollView_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+ScrollView+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct ScrollView_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::VisualElement_UxmlTraits,
    pub m_ScrollViewMode: *mut crate::UnityEngine::UIElements::UxmlEnumAttributeDescription_1<
        crate::UnityEngine::UIElements::ScrollViewMode,
    >,
    pub m_NestedInteractionKind: *mut crate::UnityEngine::UIElements::UxmlEnumAttributeDescription_1<
        crate::UnityEngine::UIElements::ScrollView_NestedInteractionKind,
    >,
    pub m_ShowHorizontal: *mut crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    pub m_ShowVertical: *mut crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    pub m_HorizontalScrollerVisibility: *mut crate::UnityEngine::UIElements::UxmlEnumAttributeDescription_1<
        crate::UnityEngine::UIElements::ScrollerVisibility,
    >,
    pub m_VerticalScrollerVisibility: *mut crate::UnityEngine::UIElements::UxmlEnumAttributeDescription_1<
        crate::UnityEngine::UIElements::ScrollerVisibility,
    >,
    pub m_HorizontalPageSize: *mut crate::UnityEngine::UIElements::UxmlFloatAttributeDescription,
    pub m_VerticalPageSize: *mut crate::UnityEngine::UIElements::UxmlFloatAttributeDescription,
    pub m_MouseWheelScrollSize: *mut crate::UnityEngine::UIElements::UxmlFloatAttributeDescription,
    pub m_TouchScrollBehavior: *mut crate::UnityEngine::UIElements::UxmlEnumAttributeDescription_1<
        crate::UnityEngine::UIElements::ScrollView_TouchScrollBehavior,
    >,
    pub m_ScrollDecelerationRate: *mut crate::UnityEngine::UIElements::UxmlFloatAttributeDescription,
    pub m_Elasticity: *mut crate::UnityEngine::UIElements::UxmlFloatAttributeDescription,
    pub m_ElasticAnimationIntervalMs: *mut crate::UnityEngine::UIElements::UxmlLongAttributeDescription,
}
#[cfg(feature = "UnityEngine+UIElements+ScrollView+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ScrollView_UxmlTraits
    => "UnityEngine.UIElements"."ScrollView/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+ScrollView+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ScrollView_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::VisualElement_UxmlTraits;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ScrollView+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ScrollView_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ScrollView+UxmlTraits")]
impl crate::UnityEngine::UIElements::ScrollView_UxmlTraits {
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
}
#[cfg(feature = "UnityEngine+UIElements+ScrollView+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ScrollView_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
