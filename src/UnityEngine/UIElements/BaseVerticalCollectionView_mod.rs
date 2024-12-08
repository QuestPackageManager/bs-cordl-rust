#[cfg(feature = "UnityEngine+UIElements+BaseVerticalCollectionView")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseVerticalCollectionView {
    __cordl_parent: crate::UnityEngine::UIElements::BindableElement,
    pub itemsChosen: *mut crate::System::Action_1<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    >,
    pub selectionChanged: *mut crate::System::Action_1<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Object,
        >,
    >,
    pub selectedIndicesChanged: *mut crate::System::Action_1<
        *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
    >,
    pub itemIndexChanged: *mut crate::System::Action_2<i32, i32>,
    pub itemsSourceChanged: *mut crate::System::Action,
    pub selectionNotChanged: *mut crate::System::Action,
    pub canStartDrag: *mut crate::System::Func_2<
        crate::UnityEngine::UIElements::CanStartDragArgs,
        bool,
    >,
    pub setupDragAndDrop: *mut crate::System::Func_2<
        crate::UnityEngine::UIElements::SetupDragAndDropArgs,
        crate::UnityEngine::UIElements::StartDragArgs,
    >,
    pub dragAndDropUpdate: *mut crate::System::Func_2<
        crate::UnityEngine::UIElements::HandleDragAndDropArgs,
        crate::UnityEngine::UIElements::DragVisualMode,
    >,
    pub handleDrop: *mut crate::System::Func_2<
        crate::UnityEngine::UIElements::HandleDragAndDropArgs,
        crate::UnityEngine::UIElements::DragVisualMode,
    >,
    pub m_SelectionType: crate::UnityEngine::UIElements::SelectionType,
    pub m_HorizontalScrollingEnabled: bool,
    pub m_ShowAlternatingRowBackgrounds: crate::UnityEngine::UIElements::AlternatingRowBackground,
    pub m_FixedItemHeight: f32,
    pub m_ItemHeightIsInline: bool,
    pub m_VirtualizationMethod: crate::UnityEngine::UIElements::CollectionVirtualizationMethod,
    pub m_ScrollView: *mut crate::UnityEngine::UIElements::ScrollView,
    pub m_ViewController: *mut crate::UnityEngine::UIElements::CollectionViewController,
    pub m_VirtualizationController: *mut crate::UnityEngine::UIElements::CollectionVirtualizationController,
    pub m_NavigationManipulator: *mut crate::UnityEngine::UIElements::KeyboardNavigationManipulator,
    pub serializedVirtualizationData: *mut crate::UnityEngine::UIElements::SerializedVirtualizationData,
    pub m_SelectedIds: *mut crate::System::Collections::Generic::List_1<i32>,
    pub m_SelectedIndices: *mut crate::System::Collections::Generic::List_1<i32>,
    pub m_SelectedItems: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Object,
    >,
    pub m_LastHeight: f32,
    pub m_IsRangeSelectionDirectionUp: bool,
    pub m_Dragger: *mut crate::UnityEngine::UIElements::ListViewDragger,
    pub m_ItemIndexChangedCallback: *mut crate::System::Action_2<i32, i32>,
    pub m_ItemsSourceChangedCallback: *mut crate::System::Action,
    pub m_TouchDownPosition: crate::UnityEngine::Vector3,
}
#[cfg(feature = "UnityEngine+UIElements+BaseVerticalCollectionView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::BaseVerticalCollectionView => "UnityEngine.UIElements"
    ."BaseVerticalCollectionView"
);
#[cfg(feature = "UnityEngine+UIElements+BaseVerticalCollectionView")]
impl std::ops::Deref for crate::UnityEngine::UIElements::BaseVerticalCollectionView {
    type Target = crate::UnityEngine::UIElements::BindableElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseVerticalCollectionView")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::BaseVerticalCollectionView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseVerticalCollectionView")]
impl crate::UnityEngine::UIElements::BaseVerticalCollectionView {
    #[cfg(
        feature = "UnityEngine+UIElements+BaseVerticalCollectionView+__c__DisplayClass181_0"
    )]
    pub type __c__DisplayClass181_0 = crate::UnityEngine::UIElements::BaseVerticalCollectionView___c__DisplayClass181_0;
    #[cfg(
        feature = "UnityEngine+UIElements+BaseVerticalCollectionView+__c__DisplayClass161_0"
    )]
    pub type __c__DisplayClass161_0 = crate::UnityEngine::UIElements::BaseVerticalCollectionView___c__DisplayClass161_0;
    #[cfg(feature = "UnityEngine+UIElements+BaseVerticalCollectionView+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::BaseVerticalCollectionView_UxmlTraits;
    #[cfg(
        feature = "UnityEngine+UIElements+BaseVerticalCollectionView+__c__DisplayClass170_0"
    )]
    pub type __c__DisplayClass170_0 = crate::UnityEngine::UIElements::BaseVerticalCollectionView___c__DisplayClass170_0;
    pub fn GetOrCreateViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::CollectionViewController,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::CollectionViewController = __cordl_object
            .invoke("GetOrCreateViewController", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectedIndices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<i32> = __cordl_object
            .invoke("get_selectedIndices", ())?;
        Ok(__cordl_ret)
    }
    pub fn Rebuild(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Rebuild", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_showAlternatingRowBackgrounds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::AlternatingRowBackground,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::AlternatingRowBackground = __cordl_object
            .invoke("get_showAlternatingRowBackgrounds", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_virtualizationMethod(
        &mut self,
        value: crate::UnityEngine::UIElements::CollectionVirtualizationMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_virtualizationMethod", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_fixedItemHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_fixedItemHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn RefreshItems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshItems", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDetachFromPanel(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::DetachFromPanelEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDetachFromPanel", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn get_virtualizationController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::CollectionVirtualizationController,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::CollectionVirtualizationController = __cordl_object
            .invoke("get_virtualizationController", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_reorderable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_reorderable", ())?;
        Ok(__cordl_ret)
    }
    pub fn RaiseSetupDragAndDrop(
        &mut self,
        item: *mut crate::UnityEngine::UIElements::ReusableCollectionItem,
        ids: *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
        args: crate::UnityEngine::UIElements::StartDragArgs,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StartDragArgs> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StartDragArgs = __cordl_object
            .invoke("RaiseSetupDragAndDrop", (item, ids, args))?;
        Ok(__cordl_ret)
    }
    pub fn get_activeItems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::UIElements::ReusableCollectionItem,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::UIElements::ReusableCollectionItem,
        > = __cordl_object.invoke("get_activeItems", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasCanStartDrag(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasCanStartDrag", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lastHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_lastHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn SelectAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectAll", ())?;
        Ok(__cordl_ret)
    }
    pub fn MatchesExistingSelection(
        &mut self,
        indices: *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MatchesExistingSelection", (indices))?;
        Ok(__cordl_ret)
    }
    pub fn ClearSelection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearSelection", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_ISerializationCallbackReceiver_OnAfterDeserialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.ISerializationCallbackReceiver.OnAfterDeserialize",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreateViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::CollectionViewController,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::CollectionViewController = __cordl_object
            .invoke("CreateViewController", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessSingleClick(
        &mut self,
        clickedIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessSingleClick", (clickedIndex))?;
        Ok(__cordl_ret)
    }
    pub fn get_selectedIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_selectedIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_selectionType(
        &mut self,
        value: crate::UnityEngine::UIElements::SelectionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_selectionType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_selectedIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_selectedIndex", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Apply__cordl_bool__cordl_bool0(
        &mut self,
        op: crate::UnityEngine::UIElements::KeyboardNavigationOperation,
        shiftKey: bool,
        altKey: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Apply", (op, shiftKey, altKey))?;
        Ok(__cordl_ret)
    }
    pub fn Apply_EventBase1(
        &mut self,
        op: crate::UnityEngine::UIElements::KeyboardNavigationOperation,
        sourceEvent: *mut crate::UnityEngine::UIElements::EventBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Apply", (op, sourceEvent))?;
        Ok(__cordl_ret)
    }
    pub fn _RefreshSelection_g__NotifyIfChanged_170_0(
        &mut self,
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::BaseVerticalCollectionView___c__DisplayClass170_0,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "<RefreshSelection>g__NotifyIfChanged|170_0",
                (_cordl_fixed_empty_name_whitespace),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RaiseHandleDragAndDrop(
        &mut self,
        pointerPosition: crate::UnityEngine::Vector2,
        dragAndDropArgs: crate::UnityEngine::UIElements::DragAndDropArgs,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::DragVisualMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::DragVisualMode = __cordl_object
            .invoke("RaiseHandleDragAndDrop", (pointerPosition, dragAndDropArgs))?;
        Ok(__cordl_ret)
    }
    pub fn OnScroll(
        &mut self,
        offset: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnScroll", (offset))?;
        Ok(__cordl_ret)
    }
    pub fn Resize(
        &mut self,
        _cordl_size: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Resize", (_cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn set_showAlternatingRowBackgrounds(
        &mut self,
        value: crate::UnityEngine::UIElements::AlternatingRowBackground,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_showAlternatingRowBackgrounds", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_dragger(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::ListViewDragger,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::ListViewDragger = __cordl_object
            .invoke("get_dragger", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectedIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<i32> = __cordl_object
            .invoke("get_selectedIds", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IList_f32_1(
        &mut self,
        itemsSource: *mut crate::System::Collections::IList,
        itemHeight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (itemsSource, itemHeight))?;
        Ok(__cordl_ret)
    }
    pub fn PostRefresh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PostRefresh", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleItemNavigation(
        &mut self,
        moveIn: bool,
        altKey: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HandleItemNavigation", (moveIn, altKey))?;
        Ok(__cordl_ret)
    }
    pub fn SetSelection_i32_0(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSelection", (index))?;
        Ok(__cordl_ret)
    }
    pub fn SetSelection_IEnumerable_1_1(
        &mut self,
        indices: *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSelection", (indices))?;
        Ok(__cordl_ret)
    }
    pub fn GetRootElementForId(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::VisualElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::VisualElement = __cordl_object
            .invoke("GetRootElementForId", (id))?;
        Ok(__cordl_ret)
    }
    pub fn OnSizeChanged(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::GeometryChangedEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSizeChanged", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnCustomStyleResolved(
        &mut self,
        e: *mut crate::UnityEngine::UIElements::CustomStyleResolvedEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCustomStyleResolved", (e))?;
        Ok(__cordl_ret)
    }
    pub fn __ctor_b__158_0(
        &mut self,
        v: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<.ctor>b__158_0", (v))?;
        Ok(__cordl_ret)
    }
    pub fn set_fixedItemHeight(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fixedItemHeight", (value))?;
        Ok(__cordl_ret)
    }
    pub fn NotifyOfSelectionChange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyOfSelectionChange", ())?;
        Ok(__cordl_ret)
    }
    pub fn RaiseDrop(
        &mut self,
        pointerPosition: crate::UnityEngine::Vector2,
        dragAndDropArgs: crate::UnityEngine::UIElements::DragAndDropArgs,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::DragVisualMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::DragVisualMode = __cordl_object
            .invoke("RaiseDrop", (pointerPosition, dragAndDropArgs))?;
        Ok(__cordl_ret)
    }
    pub fn SetSelectionWithoutNotify(
        &mut self,
        indices: *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSelectionWithoutNotify", (indices))?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_ISerializationCallbackReceiver_OnBeforeSerialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.ISerializationCallbackReceiver.OnBeforeSerialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn ExecuteDefaultAction(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::EventBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteDefaultAction", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnItemsSourceChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnItemsSourceChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnAttachToPanel(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::AttachToPanelEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAttachToPanel", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessPointerUp(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::IPointerEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessPointerUp", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn ClearSelectionWithoutValidation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearSelectionWithoutValidation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_viewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::CollectionViewController,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::CollectionViewController = __cordl_object
            .invoke("get_viewController", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateVirtualizationController_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateVirtualizationController", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateVirtualizationController_1<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateVirtualizationController", ())?;
        Ok(__cordl_ret)
    }
    pub fn DoRangeSelection(
        &mut self,
        rangeSelectionFinalIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoRangeSelection", (rangeSelectionFinalIndex))?;
        Ok(__cordl_ret)
    }
    pub fn set_showBorder(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_showBorder", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetOrCreateVirtualizationController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::CollectionVirtualizationController,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::CollectionVirtualizationController = __cordl_object
            .invoke("GetOrCreateVirtualizationController", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitializeDragAndDropController(
        &mut self,
        enableReordering: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeDragAndDropController", (enableReordering))?;
        Ok(__cordl_ret)
    }
    pub fn CreateDragAndDropController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::ICollectionDragAndDropController,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::ICollectionDragAndDropController = __cordl_object
            .invoke("CreateDragAndDropController", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddToSelectionWithoutValidation(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddToSelectionWithoutValidation", (index))?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerCancel(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::PointerCancelEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerCancel", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessPointerDown(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::IPointerEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessPointerDown", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn AddToSelection_i32_0(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddToSelection", (index))?;
        Ok(__cordl_ret)
    }
    pub fn AddToSelection_IList_1_1(
        &mut self,
        indexes: *mut crate::System::Collections::Generic::IList_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddToSelection", (indexes))?;
        Ok(__cordl_ret)
    }
    pub fn set_reorderable(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_reorderable", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_selectionType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::SelectionType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::SelectionType = __cordl_object
            .invoke("get_selectionType", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResolveItemHeight(
        &mut self,
        height: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("ResolveItemHeight", (height))?;
        Ok(__cordl_ret)
    }
    pub fn CreateDragger(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::ListViewDragger,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::ListViewDragger = __cordl_object
            .invoke("CreateDragger", ())?;
        Ok(__cordl_ret)
    }
    pub fn DoSelect(
        &mut self,
        localPosition: crate::UnityEngine::Vector2,
        clickCount: i32,
        actionKey: bool,
        shiftKey: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoSelect", (localPosition, clickCount, actionKey, shiftKey))?;
        Ok(__cordl_ret)
    }
    pub fn get_scrollView(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UIElements::ScrollView> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::ScrollView = __cordl_object
            .invoke("get_scrollView", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_contentContainer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::VisualElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::VisualElement = __cordl_object
            .invoke("get_contentContainer", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerDown(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::PointerDownEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerDown", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn set_itemsSource(
        &mut self,
        value: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_itemsSource", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HasValidDataAndBindings(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasValidDataAndBindings", ())?;
        Ok(__cordl_ret)
    }
    pub fn ScrollToItem(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScrollToItem", (index))?;
        Ok(__cordl_ret)
    }
    pub fn OnItemIndexChanged(
        &mut self,
        srcIndex: i32,
        dstIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnItemIndexChanged", (srcIndex, dstIndex))?;
        Ok(__cordl_ret)
    }
    pub fn SetSelectionInternal(
        &mut self,
        indices: *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
        sendNotification: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSelectionInternal", (indices, sendNotification))?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerUp(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::PointerUpEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerUp", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn get_itemsSource(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IList = __cordl_object
            .invoke("get_itemsSource", ())?;
        Ok(__cordl_ret)
    }
    pub fn ScrollToItemById(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScrollToItemById", (id))?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerMove(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::PointerMoveEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerMove", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveFromSelection(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveFromSelection", (index))?;
        Ok(__cordl_ret)
    }
    pub fn _Apply_g__HandleSelectionAndScroll_181_0(
        &mut self,
        index: i32,
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::BaseVerticalCollectionView___c__DisplayClass181_0,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "<Apply>g__HandleSelectionAndScroll|181_0",
                (index, _cordl_fixed_empty_name_whitespace),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_virtualizationMethod(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::CollectionVirtualizationMethod,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::CollectionVirtualizationMethod = __cordl_object
            .invoke("get_virtualizationMethod", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetViewController(
        &mut self,
        controller: *mut crate::UnityEngine::UIElements::CollectionViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetViewController", (controller))?;
        Ok(__cordl_ret)
    }
    pub fn RefreshSelection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshSelection", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnViewDataReady(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnViewDataReady", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_horizontalScrollingEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_horizontalScrollingEnabled", (value))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveFromSelectionWithoutValidation(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveFromSelectionWithoutValidation", (index))?;
        Ok(__cordl_ret)
    }
    pub fn RaiseCanStartDrag(
        &mut self,
        item: *mut crate::UnityEngine::UIElements::ReusableCollectionItem,
        ids: *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RaiseCanStartDrag", (item, ids))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_IList_f32_1(
        itemsSource: *mut crate::System::Collections::IList,
        itemHeight: f32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (itemsSource, itemHeight))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseVerticalCollectionView")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::BaseVerticalCollectionView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseVerticalCollectionView+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseVerticalCollectionView_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::BindableElement_UxmlTraits,
    pub m_FixedItemHeight: *mut crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
    pub m_VirtualizationMethod: *mut crate::UnityEngine::UIElements::UxmlEnumAttributeDescription_1<
        crate::UnityEngine::UIElements::CollectionVirtualizationMethod,
    >,
    pub m_ShowBorder: *mut crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    pub m_SelectionType: *mut crate::UnityEngine::UIElements::UxmlEnumAttributeDescription_1<
        crate::UnityEngine::UIElements::SelectionType,
    >,
    pub m_ShowAlternatingRowBackgrounds: *mut crate::UnityEngine::UIElements::UxmlEnumAttributeDescription_1<
        crate::UnityEngine::UIElements::AlternatingRowBackground,
    >,
    pub m_Reorderable: *mut crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    pub m_HorizontalScrollingEnabled: *mut crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
}
#[cfg(feature = "UnityEngine+UIElements+BaseVerticalCollectionView+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::BaseVerticalCollectionView_UxmlTraits =>
    "UnityEngine.UIElements"."BaseVerticalCollectionView/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+BaseVerticalCollectionView+UxmlTraits")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::BaseVerticalCollectionView_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::BindableElement_UxmlTraits;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseVerticalCollectionView+UxmlTraits")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::BaseVerticalCollectionView_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseVerticalCollectionView+UxmlTraits")]
impl crate::UnityEngine::UIElements::BaseVerticalCollectionView_UxmlTraits {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        ve: *mut crate::UnityEngine::UIElements::VisualElement,
        bag: *mut crate::UnityEngine::UIElements::IUxmlAttributes,
        cc: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (ve, bag, cc))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseVerticalCollectionView+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::BaseVerticalCollectionView_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
