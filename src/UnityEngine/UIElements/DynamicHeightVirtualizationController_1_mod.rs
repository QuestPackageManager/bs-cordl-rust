#[cfg(feature = "UnityEngine+UIElements+DynamicHeightVirtualizationController_1")]
#[repr(C)]
#[derive(Debug)]
pub struct DynamicHeightVirtualizationController_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::UIElements::VerticalVirtualizationController_1<
        T,
    >,
    pub m_HighestCachedIndex: i32,
    pub m_ItemHeightCache: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<i32, f32>,
    >,
    pub m_ContentHeightCache: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            crate::UnityEngine::UIElements::DynamicHeightVirtualizationController_1_ContentHeightCacheInfo<
                T,
            >,
        >,
    >,
    pub m_WaitingCache: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<i32>,
    >,
    pub m_ForcedFirstVisibleItem: i32,
    pub m_ForcedLastVisibleItem: i32,
    pub m_StickToBottom: bool,
    pub m_LastChange: crate::UnityEngine::UIElements::DynamicHeightVirtualizationController_1_VirtualizationChange<
        T,
    >,
    pub m_ScrollDirection: crate::UnityEngine::UIElements::DynamicHeightVirtualizationController_1_ScrollDirection<
        T,
    >,
    pub m_DelayedScrollOffset: crate::UnityEngine::Vector2,
    pub m_AccumulatedHeight: f32,
    pub m_MinimumItemHeight: f32,
    pub m_FillCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub m_ScrollCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub m_ScrollResetCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub m_GeometryChangedCallback: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::ReusableCollectionItem,
            >,
        >,
    >,
    pub m_ScheduledItem: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::IVisualElementScheduledItem,
    >,
    pub m_ScrollScheduledItem: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::IVisualElementScheduledItem,
    >,
    pub m_ScrollResetScheduledItem: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::IVisualElementScheduledItem,
    >,
    pub m_IndexOutOfBoundsPredicate: quest_hook::libil2cpp::Gc<
        crate::System::Predicate_1<i32>,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+UIElements+DynamicHeightVirtualizationController_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::DynamicHeightVirtualizationController_1 < T > =>
    "UnityEngine.UIElements"."DynamicHeightVirtualizationController`1" < T >
);
#[cfg(feature = "UnityEngine+UIElements+DynamicHeightVirtualizationController_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::UIElements::DynamicHeightVirtualizationController_1<T> {
    type Target = crate::UnityEngine::UIElements::VerticalVirtualizationController_1<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DynamicHeightVirtualizationController_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::UIElements::DynamicHeightVirtualizationController_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DynamicHeightVirtualizationController_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::DynamicHeightVirtualizationController_1<T> {
    #[cfg(
        feature = "UnityEngine+UIElements+DynamicHeightVirtualizationController_1+ContentHeightCacheInfo"
    )]
    pub type ContentHeightCacheInfo = crate::UnityEngine::UIElements::DynamicHeightVirtualizationController_1_ContentHeightCacheInfo<
        T,
    >;
    #[cfg(
        feature = "UnityEngine+UIElements+DynamicHeightVirtualizationController_1+ScrollDirection"
    )]
    pub type ScrollDirection = crate::UnityEngine::UIElements::DynamicHeightVirtualizationController_1_ScrollDirection;
    #[cfg(
        feature = "UnityEngine+UIElements+DynamicHeightVirtualizationController_1+VirtualizationChange"
    )]
    pub type VirtualizationChange = crate::UnityEngine::UIElements::DynamicHeightVirtualizationController_1_VirtualizationChange;
    pub fn ApplyScrollViewUpdate(
        &mut self,
        dimensionsOnly: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyScrollViewUpdate", (dimensionsOnly))?;
        Ok(__cordl_ret.into())
    }
    pub fn CleanItemHeightCache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanItemHeightCache", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CycleItems(
        &mut self,
        firstIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CycleItems", (firstIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndDrag(
        &mut self,
        dropIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndDrag", (dropIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn Fill(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Fill", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCachedContentHeight(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::DynamicHeightVirtualizationController_1_ContentHeightCacheInfo<
            T,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::DynamicHeightVirtualizationController_1_ContentHeightCacheInfo<
            T,
        > = __cordl_object.invoke("GetCachedContentHeight", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetContentHeightForIndex(
        &mut self,
        lastIndex: i32,
    ) -> quest_hook::libil2cpp::Result<f32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetContentHeightForIndex", (lastIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExpectedContentHeight(&mut self) -> quest_hook::libil2cpp::Result<f32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetExpectedContentHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExpectedItemHeight(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<f32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetExpectedItemHeight", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFirstVisibleItem(
        &mut self,
        offset: f32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetFirstVisibleItem", (offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIndexFromPosition(
        &mut self,
        position: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetIndexFromPosition", (position))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrMakeItemAtIndex(
        &mut self,
        activeItemIndex: i32,
        scrollViewIndex: i32,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("GetOrMakeItemAtIndex", (activeItemIndex, scrollViewIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn HideItem(
        &mut self,
        activeItemsIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HideItem", (activeItemsIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsIndexOutOfBounds(&mut self, i: i32) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsIndexOutOfBounds", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkWaitingForLayout(
        &mut self,
        item: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkWaitingForLayout", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn NeedsFill(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("NeedsFill", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        collectionView: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseVerticalCollectionView,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (collectionView))?;
        Ok(__cordl_object.into())
    }
    pub fn OnRecycledItemGeometryChanged(
        &mut self,
        item: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ReusableCollectionItem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRecycledItemGeometryChanged", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnScroll(
        &mut self,
        scrollOffset: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnScroll", (scrollOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnScrollUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnScrollUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Refresh(
        &mut self,
        rebuild: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Refresh", (rebuild))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterItemHeight(
        &mut self,
        index: i32,
        height: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterItemHeight", (index, height))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseItem(
        &mut self,
        activeItemsIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReleaseItem", (activeItemsIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetScroll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetScroll", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Resize(
        &mut self,
        _cordl_size: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Resize", (_cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleFill(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScheduleFill", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleScroll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScheduleScroll", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleScrollDirectionReset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScheduleScrollDirectionReset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScrollToItem(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScrollToItem", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartDragItem(
        &mut self,
        item: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ReusableCollectionItem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartDragItem", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterItemHeight(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterItemHeight", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAnchor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAnchor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateRegisteredHeight(
        &mut self,
        item: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ReusableCollectionItem,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("UpdateRegisteredHeight", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateScrollViewContainer(
        &mut self,
        previousHeight: f32,
        newHeight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateScrollViewContainer", (previousHeight, newHeight))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        collectionView: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseVerticalCollectionView,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (collectionView))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_alwaysRebindOnRefresh(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_alwaysRebindOnRefresh", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_anchorOffset(&mut self) -> quest_hook::libil2cpp::Result<f32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_anchorOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_anchoredIndex(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_anchoredIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_contentHeight(&mut self) -> quest_hook::libil2cpp::Result<f32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_contentHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_contentPadding(&mut self) -> quest_hook::libil2cpp::Result<f32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_contentPadding", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultExpectedHeight(&mut self) -> quest_hook::libil2cpp::Result<f32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_defaultExpectedHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_viewportMaxOffset(&mut self) -> quest_hook::libil2cpp::Result<f32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_viewportMaxOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_anchorOffset(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_anchorOffset", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_anchoredIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_anchoredIndex", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_contentHeight(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_contentHeight", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_contentPadding(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_contentPadding", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+DynamicHeightVirtualizationController_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::DynamicHeightVirtualizationController_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+DynamicHeightVirtualizationController_1+ContentHeightCacheInfo"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DynamicHeightVirtualizationController_1_ContentHeightCacheInfo<
    T: quest_hook::libil2cpp::Type,
> {
    pub sum: f32,
    pub count: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(
    feature = "UnityEngine+UIElements+DynamicHeightVirtualizationController_1+ContentHeightCacheInfo"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::DynamicHeightVirtualizationController_1_ContentHeightCacheInfo
    < T > => "UnityEngine.UIElements"
    ."DynamicHeightVirtualizationController`1/ContentHeightCacheInfo<T>" < T >
);
#[cfg(
    feature = "UnityEngine+UIElements+DynamicHeightVirtualizationController_1+ContentHeightCacheInfo"
)]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::DynamicHeightVirtualizationController_1_ContentHeightCacheInfo<
    T,
> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+DynamicHeightVirtualizationController_1+ContentHeightCacheInfo"
)]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::DynamicHeightVirtualizationController_1_ContentHeightCacheInfo<
    T,
> {
    pub fn _ctor(
        &mut self,
        sum: f32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (sum, count),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+DynamicHeightVirtualizationController_1+ScrollDirection"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DynamicHeightVirtualizationController_1_ScrollDirection {
    #[default]
    Down = 2i32,
    Idle = 0i32,
    Up = 1i32,
}
#[cfg(
    feature = "UnityEngine+UIElements+DynamicHeightVirtualizationController_1+ScrollDirection"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::DynamicHeightVirtualizationController_1_ScrollDirection =>
    "UnityEngine.UIElements"."DynamicHeightVirtualizationController`1/ScrollDirection<T>"
);
#[cfg(
    feature = "UnityEngine+UIElements+DynamicHeightVirtualizationController_1+VirtualizationChange"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DynamicHeightVirtualizationController_1_VirtualizationChange {
    #[default]
    ForcedScroll = 3i32,
    None = 0i32,
    Resize = 1i32,
    Scroll = 2i32,
}
#[cfg(
    feature = "UnityEngine+UIElements+DynamicHeightVirtualizationController_1+VirtualizationChange"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::DynamicHeightVirtualizationController_1_VirtualizationChange
    => "UnityEngine.UIElements"
    ."DynamicHeightVirtualizationController`1/VirtualizationChange<T>"
);
