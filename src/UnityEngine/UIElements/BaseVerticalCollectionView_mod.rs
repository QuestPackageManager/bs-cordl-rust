#[cfg(feature = "UnityEngine+UIElements+BaseVerticalCollectionView")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseVerticalCollectionView {
    __cordl_parent: crate::UnityEngine::UIElements::BindableElement,
    pub itemsChosen: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IEnumerable_1<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                >,
            >,
        >,
    >,
    pub selectionChanged: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IEnumerable_1<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                >,
            >,
        >,
    >,
    pub selectedIndicesChanged: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IEnumerable_1<i32>,
            >,
        >,
    >,
    pub itemIndexChanged: quest_hook::libil2cpp::Gc<crate::System::Action_2<i32, i32>>,
    pub itemsSourceChanged: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub selectionNotChanged: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub canStartDrag: quest_hook::libil2cpp::Gc<
        crate::System::Func_2<crate::UnityEngine::UIElements::CanStartDragArgs, bool>,
    >,
    pub setupDragAndDrop: quest_hook::libil2cpp::Gc<
        crate::System::Func_2<
            crate::UnityEngine::UIElements::SetupDragAndDropArgs,
            crate::UnityEngine::UIElements::StartDragArgs,
        >,
    >,
    pub dragAndDropUpdate: quest_hook::libil2cpp::Gc<
        crate::System::Func_2<
            crate::UnityEngine::UIElements::HandleDragAndDropArgs,
            crate::UnityEngine::UIElements::DragVisualMode,
        >,
    >,
    pub handleDrop: quest_hook::libil2cpp::Gc<
        crate::System::Func_2<
            crate::UnityEngine::UIElements::HandleDragAndDropArgs,
            crate::UnityEngine::UIElements::DragVisualMode,
        >,
    >,
    pub m_SelectionType: crate::UnityEngine::UIElements::SelectionType,
    pub m_HorizontalScrollingEnabled: bool,
    pub m_ShowAlternatingRowBackgrounds: crate::UnityEngine::UIElements::AlternatingRowBackground,
    pub m_FixedItemHeight: f32,
    pub m_ItemHeightIsInline: bool,
    pub m_VirtualizationMethod: crate::UnityEngine::UIElements::CollectionVirtualizationMethod,
    pub m_ScrollView: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::ScrollView,
    >,
    pub m_ViewController: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::CollectionViewController,
    >,
    pub m_VirtualizationController: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::CollectionVirtualizationController,
    >,
    pub m_NavigationManipulator: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::KeyboardNavigationManipulator,
    >,
    pub serializedVirtualizationData: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::SerializedVirtualizationData,
    >,
    pub m_SelectedIds: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<i32>,
    >,
    pub m_SelectedIndices: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<i32>,
    >,
    pub m_SelectedItems: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    >,
    pub m_LastHeight: f32,
    pub m_IsRangeSelectionDirectionUp: bool,
    pub m_Dragger: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::ListViewDragger,
    >,
    pub m_ItemIndexChangedCallback: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<i32, i32>,
    >,
    pub m_ItemsSourceChangedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub m_TouchDownPosition: crate::UnityEngine::Vector3,
}
#[cfg(feature = "UnityEngine+UIElements+BaseVerticalCollectionView")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::BaseVerticalCollectionView {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "BaseVerticalCollectionView";
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
    #[cfg(feature = "UnityEngine+UIElements+BaseVerticalCollectionView+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::BaseVerticalCollectionView_UxmlTraits;
    pub fn AddToSelectionWithoutValidation(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddToSelectionWithoutValidation", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddToSelection_IList_1_1(
        &mut self,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddToSelection", (indexes))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn Apply_EventBase1(
        &mut self,
        op: crate::UnityEngine::UIElements::KeyboardNavigationOperation,
        sourceEvent: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Apply", (op, sourceEvent))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn ClearSelection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearSelection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearSelectionWithoutValidation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearSelectionWithoutValidation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDragAndDropController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ICollectionDragAndDropController,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ICollectionDragAndDropController,
        > = __cordl_object.invoke("CreateDragAndDropController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDragger(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ListViewDragger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ListViewDragger,
        > = __cordl_object.invoke("CreateDragger", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::CollectionViewController,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::CollectionViewController,
        > = __cordl_object.invoke("CreateViewController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateVirtualizationController_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateVirtualizationController", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
    pub fn GetOrCreateViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::CollectionViewController,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::CollectionViewController,
        > = __cordl_object.invoke("GetOrCreateViewController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrCreateVirtualizationController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::CollectionVirtualizationController,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::CollectionVirtualizationController,
        > = __cordl_object.invoke("GetOrCreateVirtualizationController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRootElementForId(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("GetRootElementForId", (id))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HasCanStartDrag(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasCanStartDrag", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasValidDataAndBindings(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasValidDataAndBindings", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn MatchesExistingSelection(
        &mut self,
        indices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MatchesExistingSelection", (indices))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_IList_f32_1(
        itemsSource: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        itemHeight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (itemsSource, itemHeight))?;
        Ok(__cordl_object.into())
    }
    pub fn NotifyOfSelectionChange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyOfSelectionChange", ())?;
        Ok(__cordl_ret.into())
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
    pub fn OnCustomStyleResolved(
        &mut self,
        e: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::CustomStyleResolvedEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCustomStyleResolved", (e))?;
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
        Ok(__cordl_ret.into())
    }
    pub fn OnItemsSourceChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnItemsSourceChanged", ())?;
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
    pub fn OnScroll(
        &mut self,
        offset: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnScroll", (offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSizeChanged(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::GeometryChangedEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSizeChanged", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnViewDataReady(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnViewDataReady", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PostRefresh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PostRefresh", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessPointerDown(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPointerEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessPointerDown", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessPointerUp(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPointerEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessPointerUp", (evt))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn RaiseCanStartDrag(
        &mut self,
        item: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ReusableCollectionItem,
        >,
        ids: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RaiseCanStartDrag", (item, ids))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn RaiseSetupDragAndDrop(
        &mut self,
        item: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ReusableCollectionItem,
        >,
        ids: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
        args: crate::UnityEngine::UIElements::StartDragArgs,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StartDragArgs> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StartDragArgs = __cordl_object
            .invoke("RaiseSetupDragAndDrop", (item, ids, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn Rebuild(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Rebuild", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshItems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshItems", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshSelection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshSelection", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn ResolveItemHeight(
        &mut self,
        height: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("ResolveItemHeight", (height))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn SelectAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectAll", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSelectionInternal(
        &mut self,
        indices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
        sendNotification: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSelectionInternal", (indices, sendNotification))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSelectionWithoutNotify(
        &mut self,
        indices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSelectionWithoutNotify", (indices))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSelection_IEnumerable_1_1(
        &mut self,
        indices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSelection", (indices))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn SetViewController(
        &mut self,
        controller: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::CollectionViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetViewController", (controller))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ISerializationCallbackReceiver_OnBeforeSerialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.ISerializationCallbackReceiver.OnBeforeSerialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _Apply_g__HandleSelectionAndScroll_181_0(
        &mut self,
        index: i32,
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
        Ok(__cordl_ret.into())
    }
    pub fn _RefreshSelection_g__NotifyIfChanged_170_0(
        &mut self,
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
        Ok(__cordl_ret.into())
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
    pub fn _ctor_IList_f32_1(
        &mut self,
        itemsSource: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        itemHeight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (itemsSource, itemHeight))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_activeItems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::ReusableCollectionItem,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::ReusableCollectionItem,
                >,
            >,
        > = __cordl_object.invoke("get_activeItems", ())?;
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
    pub fn get_dragger(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ListViewDragger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ListViewDragger,
        > = __cordl_object.invoke("get_dragger", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fixedItemHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_fixedItemHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_itemsSource(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IList> = __cordl_object
            .invoke("get_itemsSource", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lastHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_lastHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_reorderable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_reorderable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scrollView(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ScrollView>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ScrollView,
        > = __cordl_object.invoke("get_scrollView", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        > = __cordl_object.invoke("get_selectedIds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_selectedIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedIndices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        > = __cordl_object.invoke("get_selectedIndices", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectionType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::SelectionType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::SelectionType = __cordl_object
            .invoke("get_selectionType", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_viewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::CollectionViewController,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::CollectionViewController,
        > = __cordl_object.invoke("get_viewController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_virtualizationController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::CollectionVirtualizationController,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::CollectionVirtualizationController,
        > = __cordl_object.invoke("get_virtualizationController", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_itemsSource(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_itemsSource", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
#[cfg(feature = "UnityEngine+UIElements+BaseVerticalCollectionView")]
impl AsRef<crate::UnityEngine::ISerializationCallbackReceiver>
for crate::UnityEngine::UIElements::BaseVerticalCollectionView {
    fn as_ref(&self) -> &crate::UnityEngine::ISerializationCallbackReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseVerticalCollectionView")]
impl AsMut<crate::UnityEngine::ISerializationCallbackReceiver>
for crate::UnityEngine::UIElements::BaseVerticalCollectionView {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::ISerializationCallbackReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseVerticalCollectionView+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseVerticalCollectionView_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::BindableElement_UxmlTraits,
    pub m_FixedItemHeight: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
    >,
    pub m_VirtualizationMethod: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlEnumAttributeDescription_1<
            crate::UnityEngine::UIElements::CollectionVirtualizationMethod,
        >,
    >,
    pub m_ShowBorder: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    >,
    pub m_SelectionType: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlEnumAttributeDescription_1<
            crate::UnityEngine::UIElements::SelectionType,
        >,
    >,
    pub m_ShowAlternatingRowBackgrounds: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlEnumAttributeDescription_1<
            crate::UnityEngine::UIElements::AlternatingRowBackground,
        >,
    >,
    pub m_Reorderable: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    >,
    pub m_HorizontalScrollingEnabled: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+BaseVerticalCollectionView+UxmlTraits")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::BaseVerticalCollectionView_UxmlTraits {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "UxmlTraits";
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
