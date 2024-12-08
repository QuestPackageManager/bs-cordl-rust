#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ColumnData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct MultiColumnCollectionHeader_ColumnData {
    __cordl_parent: crate::System::Object,
    pub _control_k__BackingField: *mut crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumn,
    pub _resizeHandle_k__BackingField: *mut crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnResizeHandle,
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ColumnData"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ColumnData =>
    "UnityEngine.UIElements.Internal"."MultiColumnCollectionHeader/ColumnData"
);
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ColumnData"
)]
impl std::ops::Deref
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ColumnData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ColumnData"
)]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ColumnData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ColumnData"
)]
impl crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ColumnData {
    pub fn set_control(
        &mut self,
        value: *mut crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumn,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_control", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_resizeHandle(
        &mut self,
        value: *mut crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnResizeHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_resizeHandle", (value))?;
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
    pub fn get_control(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumn,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumn = __cordl_object
            .invoke("get_control", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_resizeHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnResizeHandle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::Internal::MultiColumnHeaderColumnResizeHandle = __cordl_object
            .invoke("get_resizeHandle", ())?;
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
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ColumnData"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ColumnData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState+ColumnState"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ViewState_ColumnState {
    pub index: i32,
    pub name: *mut crate::System::String,
    pub actualWidth: f32,
    pub width: crate::UnityEngine::UIElements::Length,
    pub visible: bool,
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState+ColumnState"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Internal::ViewState_ColumnState =>
    "UnityEngine.UIElements.Internal"."MultiColumnCollectionHeader/ViewState/ColumnState"
);
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState+ColumnState"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Internal::ViewState_ColumnState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState+ColumnState"
)]
impl crate::UnityEngine::UIElements::Internal::ViewState_ColumnState {}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiColumnCollectionHeader {
    __cordl_parent: crate::UnityEngine::UIElements::VisualElement,
    pub m_SortingEnabled: bool,
    pub m_SortedColumns: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::SortColumnDescription,
    >,
    pub m_SortDescriptions: *mut crate::UnityEngine::UIElements::SortColumnDescriptions,
    pub m_OldSortedColumnStates: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_SortedColumnState,
    >,
    pub m_SortingUpdatesTemporarilyDisabled: bool,
    pub m_ViewState: *mut crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ViewState,
    pub m_ApplyingViewState: bool,
    pub m_DoLayoutScheduled: bool,
    pub _columnDataMap_k__BackingField: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::UnityEngine::UIElements::Column,
        *mut crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ColumnData,
    >,
    pub _columnLayout_k__BackingField: *mut crate::UnityEngine::UIElements::ColumnLayout,
    pub _columnContainer_k__BackingField: *mut crate::UnityEngine::UIElements::VisualElement,
    pub _resizeHandleContainer_k__BackingField: *mut crate::UnityEngine::UIElements::VisualElement,
    pub _columns_k__BackingField: *mut crate::UnityEngine::UIElements::Columns,
    pub columnResized: *mut crate::System::Action_2<i32, f32>,
    pub columnSortingChanged: *mut crate::System::Action,
    pub contextMenuPopulateEvent: *mut crate::System::Action_2<
        *mut crate::UnityEngine::UIElements::ContextualMenuPopulateEvent,
        *mut crate::UnityEngine::UIElements::Column,
    >,
    pub viewDataRestored: *mut crate::System::Action,
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader =>
    "UnityEngine.UIElements.Internal"."MultiColumnCollectionHeader"
);
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader {
    type Target = crate::UnityEngine::UIElements::VisualElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader")]
impl crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader {
    #[cfg(
        feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+__c__DisplayClass65_0"
    )]
    pub type __c__DisplayClass65_0 = crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader___c__DisplayClass65_0;
    #[cfg(
        feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ColumnData"
    )]
    pub type ColumnData = crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ColumnData;
    #[cfg(
        feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState"
    )]
    pub type ViewState = crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ViewState;
    #[cfg(
        feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+SortedColumnState"
    )]
    pub type SortedColumnState = crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_SortedColumnState;
    #[cfg(
        feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+__c__DisplayClass71_0"
    )]
    pub type __c__DisplayClass71_0 = crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader___c__DisplayClass71_0;
    pub fn OnColumnChanged(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
        _cordl_type: crate::UnityEngine::UIElements::ColumnDataType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnChanged", (column, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn get_sortingEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_sortingEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_columnLayout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::ColumnLayout,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::ColumnLayout = __cordl_object
            .invoke("get_columnLayout", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_columnContainer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::VisualElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::VisualElement = __cordl_object
            .invoke("get_columnContainer", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_viewDataRestored(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_viewDataRestored", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        columns: *mut crate::UnityEngine::UIElements::Columns,
        sortDescriptions: *mut crate::UnityEngine::UIElements::SortColumnDescriptions,
        sortedColumns: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::SortColumnDescription,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (columns, sortDescriptions, sortedColumns))?;
        Ok(__cordl_ret)
    }
    pub fn set_sortingEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sortingEnabled", (value))?;
        Ok(__cordl_ret)
    }
    pub fn OnMoveManipulatorActivated(
        &mut self,
        mover: *mut crate::UnityEngine::UIElements::Internal::ColumnMover,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnMoveManipulatorActivated", (mover))?;
        Ok(__cordl_ret)
    }
    pub fn DoLayout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoLayout", ())?;
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
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_resizeHandleContainer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::VisualElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::VisualElement = __cordl_object
            .invoke("get_resizeHandleContainer", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_columnSortingChanged(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_columnSortingChanged", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _OnContextualMenuManipulator_b__65_0(
        &mut self,
        a: *mut crate::UnityEngine::UIElements::DropdownMenuAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OnContextualMenuManipulator>b__65_0", (a))?;
        Ok(__cordl_ret)
    }
    pub fn get_columns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UIElements::Columns> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::Columns = __cordl_object
            .invoke("get_columns", ())?;
        Ok(__cordl_ret)
    }
    pub fn ScrollHorizontally(
        &mut self,
        horizontalOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScrollHorizontally", (horizontalOffset))?;
        Ok(__cordl_ret)
    }
    pub fn OnColumnClicked(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::EventBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnClicked", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn remove_contextMenuPopulateEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::UnityEngine::UIElements::ContextualMenuPopulateEvent,
            *mut crate::UnityEngine::UIElements::Column,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_contextMenuPopulateEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn RaiseColumnResized(
        &mut self,
        columnIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseColumnResized", (columnIndex))?;
        Ok(__cordl_ret)
    }
    pub fn get_sortDescriptions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::SortColumnDescriptions,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::SortColumnDescriptions = __cordl_object
            .invoke("get_sortDescriptions", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_columnSortingChanged(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_columnSortingChanged", (value))?;
        Ok(__cordl_ret)
    }
    pub fn RaiseColumnSortingChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseColumnSortingChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnColumnReordered(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
        from: i32,
        to: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnReordered", (column, from, to))?;
        Ok(__cordl_ret)
    }
    pub fn OnColumnControlGeometryChanged(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::GeometryChangedEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnControlGeometryChanged", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn ResizeToFit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResizeToFit", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sortedColumns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::UIElements::SortColumnDescription,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::UIElements::SortColumnDescription,
        > = __cordl_object.invoke("get_sortedColumns", ())?;
        Ok(__cordl_ret)
    }
    pub fn SaveViewState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveViewState", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_columnDataMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::UnityEngine::UIElements::Column,
            *mut crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ColumnData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::UnityEngine::UIElements::Column,
            *mut crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ColumnData,
        > = __cordl_object.invoke("get_columnDataMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnColumnResized(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnResized", (column))?;
        Ok(__cordl_ret)
    }
    pub fn add_columnResized(
        &mut self,
        value: *mut crate::System::Action_2<i32, f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_columnResized", (value))?;
        Ok(__cordl_ret)
    }
    pub fn OnColumnRemoved(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnRemoved", (column))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateSortingStatus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateSortingStatus", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateSortColumnDescriptionsOnClick(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
        modifiers: crate::UnityEngine::EventModifiers,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateSortColumnDescriptionsOnClick", (column, modifiers))?;
        Ok(__cordl_ret)
    }
    pub fn remove_columnResized(
        &mut self,
        value: *mut crate::System::Action_2<i32, f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_columnResized", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_sortDescriptions(
        &mut self,
        value: *mut crate::UnityEngine::UIElements::SortColumnDescriptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sortDescriptions", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_viewDataRestored(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_viewDataRestored", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ApplyColumnSorting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyColumnSorting", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isApplyingViewState(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isApplyingViewState", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_contextMenuPopulateEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::UnityEngine::UIElements::ContextualMenuPopulateEvent,
            *mut crate::UnityEngine::UIElements::Column,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_contextMenuPopulateEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn OnContextualMenuManipulator(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::ContextualMenuPopulateEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnContextualMenuManipulator", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateSortedColumns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateSortedColumns", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateColumnControls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateColumnControls", ())?;
        Ok(__cordl_ret)
    }
    pub fn ScheduleDoLayout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScheduleDoLayout", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnColumnAdded_i32_0(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnAdded", (column, index))?;
        Ok(__cordl_ret)
    }
    pub fn OnColumnAdded_Column1(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnAdded", (column))?;
        Ok(__cordl_ret)
    }
    pub fn OnGeometryChanged(
        &mut self,
        e: *mut crate::UnityEngine::UIElements::GeometryChangedEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnGeometryChanged", (e))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        columns: *mut crate::UnityEngine::UIElements::Columns,
        sortDescriptions: *mut crate::UnityEngine::UIElements::SortColumnDescriptions,
        sortedColumns: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::SortColumnDescription,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (columns, sortDescriptions, sortedColumns))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+SortedColumnState"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MultiColumnCollectionHeader_SortedColumnState {
    pub columnDesc: *mut crate::UnityEngine::UIElements::SortColumnDescription,
    pub direction: crate::UnityEngine::UIElements::SortDirection,
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+SortedColumnState"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_SortedColumnState =>
    "UnityEngine.UIElements.Internal"."MultiColumnCollectionHeader/SortedColumnState"
);
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+SortedColumnState"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_SortedColumnState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+SortedColumnState"
)]
impl crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_SortedColumnState {
    pub fn _ctor(
        &mut self,
        desc: *mut crate::UnityEngine::UIElements::SortColumnDescription,
        dir: crate::UnityEngine::UIElements::SortDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (desc, dir),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiColumnCollectionHeader_ViewState {
    __cordl_parent: crate::System::Object,
    pub m_HasPersistedData: bool,
    pub m_SortDescriptions: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::SortColumnDescription,
    >,
    pub m_OrderedColumnStates: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::UIElements::Internal::ViewState_ColumnState,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ViewState =>
    "UnityEngine.UIElements.Internal"."MultiColumnCollectionHeader/ViewState"
);
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ViewState {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ViewState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState")]
impl crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ViewState {
    #[cfg(
        feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState+ColumnState"
    )]
    pub type ColumnState = crate::UnityEngine::UIElements::Internal::ViewState_ColumnState;
    pub fn Apply(
        &mut self,
        header: *mut crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Apply", (header))?;
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
    pub fn Save(
        &mut self,
        header: *mut crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Save", (header))?;
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
#[cfg(feature = "UnityEngine+UIElements+Internal+MultiColumnCollectionHeader+ViewState")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader_ViewState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
