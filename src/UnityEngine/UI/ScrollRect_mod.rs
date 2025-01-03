#[cfg(feature = "UnityEngine+UI+ScrollRect")]
#[repr(C)]
#[derive(Debug)]
pub struct ScrollRect {
    __cordl_parent: crate::UnityEngine::EventSystems::UIBehaviour,
    pub m_Content: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    pub m_Horizontal: bool,
    pub m_Vertical: bool,
    pub m_MovementType: crate::UnityEngine::UI::ScrollRect_MovementType,
    pub m_Elasticity: f32,
    pub m_Inertia: bool,
    pub m_DecelerationRate: f32,
    pub m_ScrollSensitivity: f32,
    pub m_Viewport: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    pub m_HorizontalScrollbar: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Scrollbar,
    >,
    pub m_VerticalScrollbar: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Scrollbar,
    >,
    pub m_HorizontalScrollbarVisibility: crate::UnityEngine::UI::ScrollRect_ScrollbarVisibility,
    pub m_VerticalScrollbarVisibility: crate::UnityEngine::UI::ScrollRect_ScrollbarVisibility,
    pub m_HorizontalScrollbarSpacing: f32,
    pub m_VerticalScrollbarSpacing: f32,
    pub m_OnValueChanged: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::ScrollRect_ScrollRectEvent,
    >,
    pub m_PointerStartLocalCursor: crate::UnityEngine::Vector2,
    pub m_ContentStartPosition: crate::UnityEngine::Vector2,
    pub m_ViewRect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    pub m_ContentBounds: crate::UnityEngine::Bounds,
    pub m_ViewBounds: crate::UnityEngine::Bounds,
    pub m_Velocity: crate::UnityEngine::Vector2,
    pub m_Dragging: bool,
    pub m_Scrolling: bool,
    pub m_PrevPosition: crate::UnityEngine::Vector2,
    pub m_PrevContentBounds: crate::UnityEngine::Bounds,
    pub m_PrevViewBounds: crate::UnityEngine::Bounds,
    pub m_HasRebuiltLayout: bool,
    pub m_HSliderExpand: bool,
    pub m_VSliderExpand: bool,
    pub m_HSliderHeight: f32,
    pub m_VSliderWidth: f32,
    pub m_Rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    pub m_HorizontalScrollbarRect: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::RectTransform,
    >,
    pub m_VerticalScrollbarRect: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::RectTransform,
    >,
    pub m_Tracker: crate::UnityEngine::DrivenRectTransformTracker,
    pub m_Corners: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    >,
}
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::ScrollRect => "UnityEngine.UI"
    ."ScrollRect"
);
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
impl std::ops::Deref for crate::UnityEngine::UI::ScrollRect {
    type Target = crate::UnityEngine::EventSystems::UIBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
impl std::ops::DerefMut for crate::UnityEngine::UI::ScrollRect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
impl crate::UnityEngine::UI::ScrollRect {
    #[cfg(feature = "UnityEngine+UI+ScrollRect+MovementType")]
    pub type MovementType = crate::UnityEngine::UI::ScrollRect_MovementType;
    #[cfg(feature = "UnityEngine+UI+ScrollRect+ScrollRectEvent")]
    pub type ScrollRectEvent = crate::UnityEngine::UI::ScrollRect_ScrollRectEvent;
    #[cfg(feature = "UnityEngine+UI+ScrollRect+ScrollbarVisibility")]
    pub type ScrollbarVisibility = crate::UnityEngine::UI::ScrollRect_ScrollbarVisibility;
    pub fn AdjustBounds(
        viewBounds: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
        contentPivot: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        contentSize: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        contentPos: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AdjustBounds",
                (viewBounds, contentPivot, contentSize, contentPos),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateLayoutInputHorizontal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateLayoutInputHorizontal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateLayoutInputVertical(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateLayoutInputVertical", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateOffset(
        &mut self,
        delta: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("CalculateOffset", (delta))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureLayoutHasRebuilt(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureLayoutHasRebuilt", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBounds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Bounds = __cordl_object
            .invoke("GetBounds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GraphicUpdateComplete(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GraphicUpdateComplete", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalCalculateOffset(
        viewBounds: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
        contentBounds: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
        horizontal: bool,
        vertical: bool,
        movementType: crate::UnityEngine::UI::ScrollRect_MovementType,
        delta: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InternalCalculateOffset",
                (viewBounds, contentBounds, horizontal, vertical, movementType, delta),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetBounds(
        corners: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        viewWorldToLocalMatrix: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Matrix4x4,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        let __cordl_ret: crate::UnityEngine::Bounds = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalGetBounds", (corners, viewWorldToLocalMatrix))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsActive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsActive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LayoutComplete(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LayoutComplete", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnBeginDrag(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBeginDrag", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDrag(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDrag", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEndDrag(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEndDrag", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnInitializePotentialDrag(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnInitializePotentialDrag", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnRectTransformDimensionsChange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRectTransformDimensionsChange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnScroll(
        &mut self,
        data: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnScroll", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn Rebuild(
        &mut self,
        executing: crate::UnityEngine::UI::CanvasUpdate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Rebuild", (executing))?;
        Ok(__cordl_ret.into())
    }
    pub fn RubberDelta(
        overStretching: f32,
        viewSize: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RubberDelta", (overStretching, viewSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetContentAnchoredPosition(
        &mut self,
        position: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetContentAnchoredPosition", (position))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDirty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDirty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDirtyCaching(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDirtyCaching", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetHorizontalNormalizedPosition(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHorizontalNormalizedPosition", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLayoutHorizontal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLayoutHorizontal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLayoutVertical(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLayoutVertical", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNormalizedPosition(
        &mut self,
        value: f32,
        axis: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNormalizedPosition", (value, axis))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVerticalNormalizedPosition(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVerticalNormalizedPosition", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn StopMovement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopMovement", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UI_ICanvasElement_get_transform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform> = __cordl_object
            .invoke("UnityEngine.UI.ICanvasElement.get_transform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateBounds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateBounds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateCachedData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateCachedData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateOneScrollbarVisibility(
        xScrollingNeeded: bool,
        xAxisEnabled: bool,
        scrollbarVisibility: crate::UnityEngine::UI::ScrollRect_ScrollbarVisibility,
        scrollbar: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Scrollbar>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "UpdateOneScrollbarVisibility",
                (xScrollingNeeded, xAxisEnabled, scrollbarVisibility, scrollbar),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdatePrevData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdatePrevData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateScrollbarLayout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateScrollbarLayout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateScrollbarVisibility(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateScrollbarVisibility", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateScrollbars(
        &mut self,
        offset: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateScrollbars", (offset))?;
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
    pub fn get_content(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform> = __cordl_object
            .invoke("get_content", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_decelerationRate(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_decelerationRate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_elasticity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_elasticity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flexibleHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_flexibleHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flexibleWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_flexibleWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hScrollingNeeded(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hScrollingNeeded", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_horizontal(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_horizontal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_horizontalNormalizedPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_horizontalNormalizedPosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_horizontalScrollbar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Scrollbar>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Scrollbar> = __cordl_object
            .invoke("get_horizontalScrollbar", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_horizontalScrollbarSpacing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_horizontalScrollbarSpacing", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_horizontalScrollbarVisibility(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UI::ScrollRect_ScrollbarVisibility,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UI::ScrollRect_ScrollbarVisibility = __cordl_object
            .invoke("get_horizontalScrollbarVisibility", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_inertia(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_inertia", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_layoutPriority(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_layoutPriority", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_minHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_minHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_minWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_minWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_movementType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UI::ScrollRect_MovementType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UI::ScrollRect_MovementType = __cordl_object
            .invoke("get_movementType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_normalizedPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_normalizedPosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_onValueChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ScrollRect_ScrollRectEvent>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UI::ScrollRect_ScrollRectEvent,
        > = __cordl_object.invoke("get_onValueChanged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_preferredHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_preferredHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_preferredWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_preferredWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rectTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform> = __cordl_object
            .invoke("get_rectTransform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scrollSensitivity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_scrollSensitivity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vScrollingNeeded(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_vScrollingNeeded", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_velocity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_velocity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vertical(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_vertical", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_verticalNormalizedPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_verticalNormalizedPosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_verticalScrollbar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Scrollbar>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Scrollbar> = __cordl_object
            .invoke("get_verticalScrollbar", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_verticalScrollbarSpacing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_verticalScrollbarSpacing", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_verticalScrollbarVisibility(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UI::ScrollRect_ScrollbarVisibility,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UI::ScrollRect_ScrollbarVisibility = __cordl_object
            .invoke("get_verticalScrollbarVisibility", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_viewRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform> = __cordl_object
            .invoke("get_viewRect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_viewport(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform> = __cordl_object
            .invoke("get_viewport", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_content(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_content", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_decelerationRate(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_decelerationRate", (value))?;
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
    pub fn set_horizontal(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_horizontal", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_horizontalNormalizedPosition(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_horizontalNormalizedPosition", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_horizontalScrollbar(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Scrollbar>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_horizontalScrollbar", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_horizontalScrollbarSpacing(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_horizontalScrollbarSpacing", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_horizontalScrollbarVisibility(
        &mut self,
        value: crate::UnityEngine::UI::ScrollRect_ScrollbarVisibility,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_horizontalScrollbarVisibility", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_inertia(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_inertia", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_movementType(
        &mut self,
        value: crate::UnityEngine::UI::ScrollRect_MovementType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_movementType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_normalizedPosition(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_normalizedPosition", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_onValueChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UI::ScrollRect_ScrollRectEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onValueChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_scrollSensitivity(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_scrollSensitivity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_velocity(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_velocity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_vertical(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_vertical", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_verticalNormalizedPosition(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_verticalNormalizedPosition", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_verticalScrollbar(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Scrollbar>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_verticalScrollbar", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_verticalScrollbarSpacing(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_verticalScrollbarSpacing", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_verticalScrollbarVisibility(
        &mut self,
        value: crate::UnityEngine::UI::ScrollRect_ScrollbarVisibility,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_verticalScrollbarVisibility", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_viewport(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_viewport", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::ScrollRect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
impl AsRef<crate::UnityEngine::EventSystems::IBeginDragHandler>
for crate::UnityEngine::UI::ScrollRect {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IBeginDragHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
impl AsMut<crate::UnityEngine::EventSystems::IBeginDragHandler>
for crate::UnityEngine::UI::ScrollRect {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IBeginDragHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
impl AsRef<crate::UnityEngine::EventSystems::IDragHandler>
for crate::UnityEngine::UI::ScrollRect {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IDragHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
impl AsMut<crate::UnityEngine::EventSystems::IDragHandler>
for crate::UnityEngine::UI::ScrollRect {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IDragHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
impl AsRef<crate::UnityEngine::EventSystems::IEndDragHandler>
for crate::UnityEngine::UI::ScrollRect {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IEndDragHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
impl AsMut<crate::UnityEngine::EventSystems::IEndDragHandler>
for crate::UnityEngine::UI::ScrollRect {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IEndDragHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
impl AsRef<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::UnityEngine::UI::ScrollRect {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
impl AsMut<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::UnityEngine::UI::ScrollRect {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
impl AsRef<crate::UnityEngine::EventSystems::IInitializePotentialDragHandler>
for crate::UnityEngine::UI::ScrollRect {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::EventSystems::IInitializePotentialDragHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
impl AsMut<crate::UnityEngine::EventSystems::IInitializePotentialDragHandler>
for crate::UnityEngine::UI::ScrollRect {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::EventSystems::IInitializePotentialDragHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
impl AsRef<crate::UnityEngine::EventSystems::IScrollHandler>
for crate::UnityEngine::UI::ScrollRect {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IScrollHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
impl AsMut<crate::UnityEngine::EventSystems::IScrollHandler>
for crate::UnityEngine::UI::ScrollRect {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IScrollHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
impl AsRef<crate::UnityEngine::UI::ICanvasElement>
for crate::UnityEngine::UI::ScrollRect {
    fn as_ref(&self) -> &crate::UnityEngine::UI::ICanvasElement {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
impl AsMut<crate::UnityEngine::UI::ICanvasElement>
for crate::UnityEngine::UI::ScrollRect {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UI::ICanvasElement {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
impl AsRef<crate::UnityEngine::UI::ILayoutController>
for crate::UnityEngine::UI::ScrollRect {
    fn as_ref(&self) -> &crate::UnityEngine::UI::ILayoutController {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
impl AsMut<crate::UnityEngine::UI::ILayoutController>
for crate::UnityEngine::UI::ScrollRect {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UI::ILayoutController {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
impl AsRef<crate::UnityEngine::UI::ILayoutElement>
for crate::UnityEngine::UI::ScrollRect {
    fn as_ref(&self) -> &crate::UnityEngine::UI::ILayoutElement {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
impl AsMut<crate::UnityEngine::UI::ILayoutElement>
for crate::UnityEngine::UI::ScrollRect {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UI::ILayoutElement {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
impl AsRef<crate::UnityEngine::UI::ILayoutGroup> for crate::UnityEngine::UI::ScrollRect {
    fn as_ref(&self) -> &crate::UnityEngine::UI::ILayoutGroup {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect")]
impl AsMut<crate::UnityEngine::UI::ILayoutGroup> for crate::UnityEngine::UI::ScrollRect {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UI::ILayoutGroup {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect+MovementType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollRect_MovementType {
    Clamped = 2i32,
    Elastic = 1i32,
    Unrestricted = 0i32,
}
#[cfg(feature = "UnityEngine+UI+ScrollRect+MovementType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::ScrollRect_MovementType =>
    "UnityEngine.UI"."ScrollRect/MovementType"
);
#[cfg(feature = "UnityEngine+UI+ScrollRect+ScrollRectEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct ScrollRect_ScrollRectEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<
        crate::UnityEngine::Vector2,
    >,
}
#[cfg(feature = "UnityEngine+UI+ScrollRect+ScrollRectEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::ScrollRect_ScrollRectEvent =>
    "UnityEngine.UI"."ScrollRect/ScrollRectEvent"
);
#[cfg(feature = "UnityEngine+UI+ScrollRect+ScrollRectEvent")]
impl std::ops::Deref for crate::UnityEngine::UI::ScrollRect_ScrollRectEvent {
    type Target = crate::UnityEngine::Events::UnityEvent_1<crate::UnityEngine::Vector2>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect+ScrollRectEvent")]
impl std::ops::DerefMut for crate::UnityEngine::UI::ScrollRect_ScrollRectEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect+ScrollRectEvent")]
impl crate::UnityEngine::UI::ScrollRect_ScrollRectEvent {
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
#[cfg(feature = "UnityEngine+UI+ScrollRect+ScrollRectEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::ScrollRect_ScrollRectEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+ScrollRect+ScrollbarVisibility")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollRect_ScrollbarVisibility {
    AutoHide = 1i32,
    AutoHideAndExpandViewport = 2i32,
    Permanent = 0i32,
}
#[cfg(feature = "UnityEngine+UI+ScrollRect+ScrollbarVisibility")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::ScrollRect_ScrollbarVisibility
    => "UnityEngine.UI"."ScrollRect/ScrollbarVisibility"
);
