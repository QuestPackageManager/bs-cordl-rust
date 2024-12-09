#[cfg(feature = "HMUI+TableView")]
#[repr(C)]
#[derive(Debug)]
pub struct TableView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _scrollView: *mut crate::HMUI::ScrollView,
    pub _scrollToTopOnEnable: bool,
    pub _alignToCenter: bool,
    pub _tableType: crate::HMUI::TableView_TableType,
    pub _selectionType: crate::HMUI::TableViewSelectionType,
    pub _canSelectSelectedCell: bool,
    pub _preallocatedCells: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::HMUI::TableView_CellsGroup,
    >,
    pub didSelectCellWithIdxEvent: *mut crate::System::Action_2<
        *mut crate::HMUI::TableView,
        i32,
    >,
    pub didReloadDataEvent: *mut crate::System::Action_1<*mut crate::HMUI::TableView>,
    pub didInsertCellsEvent: *mut crate::System::Action_1<*mut crate::HMUI::TableView>,
    pub didDeleteCellsEvent: *mut crate::System::Action_1<*mut crate::HMUI::TableView>,
    pub didChangeRectSizeEvent: *mut crate::System::Action_1<
        *mut crate::HMUI::TableView,
    >,
    pub _contentTransform: *mut crate::UnityEngine::RectTransform,
    pub _viewportTransform: *mut crate::UnityEngine::RectTransform,
    pub _dataSource: *mut crate::HMUI::TableView_IDataSource,
    pub _numberOfCells: i32,
    pub _cellSize: f32,
    pub _visibleCells: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HMUI::TableCell,
    >,
    pub _reusableCells: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::System::Collections::Generic::List_1<*mut crate::HMUI::TableCell>,
    >,
    pub _selectedCellIdxs: *mut crate::System::Collections::Generic::HashSet_1<i32>,
    pub _prevMinIdx: i32,
    pub _prevMaxIdx: i32,
    pub _isInitialized: bool,
    pub _refreshCellsOnEnable: bool,
}
#[cfg(feature = "HMUI+TableView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::TableView => "HMUI"."TableView"
);
#[cfg(feature = "HMUI+TableView")]
impl std::ops::Deref for crate::HMUI::TableView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+TableView")]
impl std::ops::DerefMut for crate::HMUI::TableView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+TableView")]
impl crate::HMUI::TableView {
    #[cfg(feature = "HMUI+TableView+CellsGroup")]
    pub type CellsGroup = crate::HMUI::TableView_CellsGroup;
    #[cfg(feature = "HMUI+TableView+IDataSource")]
    type IDataSource = crate::HMUI::TableView_IDataSource;
    #[cfg(feature = "HMUI+TableView+ScrollPositionType")]
    pub type ScrollPositionType = crate::HMUI::TableView_ScrollPositionType;
    #[cfg(feature = "HMUI+TableView+TableType")]
    pub type TableType = crate::HMUI::TableView_TableType;
    pub fn AddCellToReusableCells(
        &mut self,
        cell: *mut crate::HMUI::TableCell,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCellToReusableCells", (cell))?;
        Ok(__cordl_ret)
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn ChangeRectSize(
        &mut self,
        axis: crate::UnityEngine::RectTransform_Axis,
        _cordl_size: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChangeRectSize", (axis, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn ClearHighlights(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearHighlights", ())?;
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
    pub fn DeleteCells(
        &mut self,
        idx: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeleteCells", (idx, count))?;
        Ok(__cordl_ret)
    }
    pub fn DequeueReusableCellForIdentifier(
        &mut self,
        identifier: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::TableCell> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::TableCell = __cordl_object
            .invoke("DequeueReusableCellForIdentifier", (identifier))?;
        Ok(__cordl_ret)
    }
    pub fn DidSelectCellWithIdx(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DidSelectCellWithIdx", (idx))?;
        Ok(__cordl_ret)
    }
    pub fn GetVisibleCellsIdRange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Tuple_2<i32, i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Tuple_2<i32, i32> = __cordl_object
            .invoke("GetVisibleCellsIdRange", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleCellSelectionDidChange(
        &mut self,
        selectableCell: *mut crate::HMUI::SelectableCell,
        transitionType: crate::HMUI::SelectableCell_TransitionType,
        changeOwner: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleCellSelectionDidChange",
                (selectableCell, transitionType, changeOwner),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleScrollRectValueChanged(
        &mut self,
        f: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleScrollRectValueChanged", (f))?;
        Ok(__cordl_ret)
    }
    pub fn Hide(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Hide", ())?;
        Ok(__cordl_ret)
    }
    pub fn InsertCells(
        &mut self,
        idx: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InsertCells", (idx, count))?;
        Ok(__cordl_ret)
    }
    pub fn LayoutCellForIdx(
        &mut self,
        cell: *mut crate::HMUI::TableCell,
        idx: i32,
        offset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LayoutCellForIdx", (cell, idx, offset))?;
        Ok(__cordl_ret)
    }
    pub fn LazyInit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LazyInit", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn RefreshCells(
        &mut self,
        forcedVisualsRefresh: bool,
        forcedContentRefresh: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshCells", (forcedVisualsRefresh, forcedContentRefresh))?;
        Ok(__cordl_ret)
    }
    pub fn RefreshCellsContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshCellsContent", ())?;
        Ok(__cordl_ret)
    }
    pub fn RefreshContentSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshContentSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReloadData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReloadData", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReloadDataKeepingPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReloadDataKeepingPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn ScrollToCellWithIdx(
        &mut self,
        idx: i32,
        scrollPositionType: crate::HMUI::TableView_ScrollPositionType,
        animated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScrollToCellWithIdx", (idx, scrollPositionType, animated))?;
        Ok(__cordl_ret)
    }
    pub fn SelectCellWithIdx(
        &mut self,
        idx: i32,
        callbackTable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectCellWithIdx", (idx, callbackTable))?;
        Ok(__cordl_ret)
    }
    pub fn SetDataSource(
        &mut self,
        newDataSource: *mut crate::HMUI::TableView_IDataSource,
        reloadData: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDataSource", (newDataSource, reloadData))?;
        Ok(__cordl_ret)
    }
    pub fn Show(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Show", ())?;
        Ok(__cordl_ret)
    }
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
    pub fn add_didChangeRectSizeEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::HMUI::TableView>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didChangeRectSizeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didDeleteCellsEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::HMUI::TableView>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didDeleteCellsEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didInsertCellsEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::HMUI::TableView>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didInsertCellsEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didReloadDataEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::HMUI::TableView>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didReloadDataEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didSelectCellWithIdxEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut crate::HMUI::TableView, i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectCellWithIdxEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_canSelectSelectedCell(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_canSelectSelectedCell", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_cellSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_cellSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_contentTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::RectTransform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::RectTransform = __cordl_object
            .invoke("get_contentTransform", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_dataSource(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::TableView_IDataSource> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::TableView_IDataSource = __cordl_object
            .invoke("get_dataSource", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_numberOfCells(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_numberOfCells", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_scrollView(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::ScrollView> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::ScrollView = __cordl_object
            .invoke("get_scrollView", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectionType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::HMUI::TableViewSelectionType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HMUI::TableViewSelectionType = __cordl_object
            .invoke("get_selectionType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_tableType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::HMUI::TableView_TableType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HMUI::TableView_TableType = __cordl_object
            .invoke("get_tableType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_viewportTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::RectTransform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::RectTransform = __cordl_object
            .invoke("get_viewportTransform", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_visibleCells(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::HMUI::TableCell,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::HMUI::TableCell,
        > = __cordl_object.invoke("get_visibleCells", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didChangeRectSizeEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::HMUI::TableView>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didChangeRectSizeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didDeleteCellsEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::HMUI::TableView>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didDeleteCellsEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didInsertCellsEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::HMUI::TableView>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didInsertCellsEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didReloadDataEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::HMUI::TableView>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didReloadDataEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didSelectCellWithIdxEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut crate::HMUI::TableView, i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectCellWithIdxEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_selectionType(
        &mut self,
        value: crate::HMUI::TableViewSelectionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_selectionType", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HMUI+TableView")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::TableView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+TableView+CellsGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct TableView_CellsGroup {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _reuseIdentifier: *mut quest_hook::libil2cpp::Il2CppString,
    pub _cells: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HMUI::TableCell,
    >,
}
#[cfg(feature = "HMUI+TableView+CellsGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::TableView_CellsGroup => "HMUI"
    ."TableView/CellsGroup"
);
#[cfg(feature = "HMUI+TableView+CellsGroup")]
impl std::ops::Deref for crate::HMUI::TableView_CellsGroup {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+TableView+CellsGroup")]
impl std::ops::DerefMut for crate::HMUI::TableView_CellsGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+TableView+CellsGroup")]
impl crate::HMUI::TableView_CellsGroup {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
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
    pub fn get_cells(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut crate::HMUI::TableCell>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HMUI::TableCell,
        > = __cordl_object.invoke("get_cells", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_reuseIdentifier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_reuseIdentifier", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HMUI+TableView+CellsGroup")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::TableView_CellsGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+TableView+IDataSource")]
#[repr(C)]
#[derive(Debug)]
pub struct TableView_IDataSource {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HMUI+TableView+IDataSource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::TableView_IDataSource => "HMUI"
    ."TableView/IDataSource"
);
#[cfg(feature = "HMUI+TableView+IDataSource")]
impl std::ops::Deref for crate::HMUI::TableView_IDataSource {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+TableView+IDataSource")]
impl std::ops::DerefMut for crate::HMUI::TableView_IDataSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+TableView+IDataSource")]
impl crate::HMUI::TableView_IDataSource {
    pub fn CellForIdx(
        &mut self,
        tableView: *mut crate::HMUI::TableView,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::TableCell> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::TableCell = __cordl_object
            .invoke("CellForIdx", (tableView, idx))?;
        Ok(__cordl_ret)
    }
    pub fn CellSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("CellSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn NumberOfCells(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("NumberOfCells", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "HMUI+TableView+IDataSource")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::TableView_IDataSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+TableView+ScrollPositionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableView_ScrollPositionType {
    Beginning = 0i32,
    Center = 1i32,
    End = 2i32,
}
#[cfg(feature = "HMUI+TableView+ScrollPositionType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::TableView_ScrollPositionType => "HMUI"
    ."TableView/ScrollPositionType"
);
#[cfg(feature = "HMUI+TableView+TableType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableView_TableType {
    Horizontal = 1i32,
    Vertical = 0i32,
}
#[cfg(feature = "HMUI+TableView+TableType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::TableView_TableType => "HMUI"
    ."TableView/TableType"
);
