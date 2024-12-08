#[cfg(feature = "System+Data+DataView+DataRowReferenceComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct DataView_DataRowReferenceComparer {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Data+DataView+DataRowReferenceComparer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::DataView_DataRowReferenceComparer
    => "System.Data"."DataView/DataRowReferenceComparer"
);
#[cfg(feature = "System+Data+DataView+DataRowReferenceComparer")]
impl std::ops::Deref for crate::System::Data::DataView_DataRowReferenceComparer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataView+DataRowReferenceComparer")]
impl std::ops::DerefMut for crate::System::Data::DataView_DataRowReferenceComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataView+DataRowReferenceComparer")]
impl crate::System::Data::DataView_DataRowReferenceComparer {
    pub fn Equals(
        &mut self,
        x: *mut crate::System::Data::DataRow,
        y: *mut crate::System::Data::DataRow,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (x, y))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(
        &mut self,
        obj: *mut crate::System::Data::DataRow,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (obj))?;
        Ok(__cordl_ret)
    }
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
}
#[cfg(feature = "System+Data+DataView+DataRowReferenceComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Data::DataView_DataRowReferenceComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Data+DataView")]
#[repr(C)]
#[derive(Debug)]
pub struct DataView {
    __cordl_parent: crate::System::ComponentModel::MarshalByValueComponent,
    pub _dataViewManager: *mut crate::System::Data::DataViewManager,
    pub _table: *mut crate::System::Data::DataTable,
    pub _locked: bool,
    pub _index: *mut crate::System::Data::Index,
    pub _findIndexes: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::System::Data::Index,
    >,
    pub _sort: *mut crate::System::String,
    pub _comparison: *mut crate::System::Comparison_1<*mut crate::System::Data::DataRow>,
    pub _rowFilter: *mut crate::System::Data::IFilter,
    pub _recordStates: crate::System::Data::DataViewRowState,
    pub _shouldOpen: bool,
    pub _open: bool,
    pub _allowNew: bool,
    pub _allowEdit: bool,
    pub _allowDelete: bool,
    pub _applyDefaultSort: bool,
    pub _addNewRow: *mut crate::System::Data::DataRow,
    pub _addNewMoved: *mut crate::System::ComponentModel::ListChangedEventArgs,
    pub _onListChanged: *mut crate::System::ComponentModel::ListChangedEventHandler,
    pub _delayedSort: *mut crate::System::String,
    pub _delayedRecordStates: crate::System::Data::DataViewRowState,
    pub _fInitInProgress: bool,
    pub _fEndInitInProgress: bool,
    pub _rowViewCache: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Data::DataRow,
        *mut crate::System::Data::DataRowView,
    >,
    pub _rowViewBuffer: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Data::DataRow,
        *mut crate::System::Data::DataRowView,
    >,
    pub _dvListener: *mut crate::System::Data::DataViewListener,
    pub _objectID: i32,
}
#[cfg(feature = "System+Data+DataView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::DataView => "System.Data"
    ."DataView"
);
#[cfg(feature = "System+Data+DataView")]
impl std::ops::Deref for crate::System::Data::DataView {
    type Target = crate::System::ComponentModel::MarshalByValueComponent;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataView")]
impl std::ops::DerefMut for crate::System::Data::DataView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataView")]
impl crate::System::Data::DataView {
    #[cfg(feature = "System+Data+DataView+DataRowReferenceComparer")]
    pub type DataRowReferenceComparer = crate::System::Data::DataView_DataRowReferenceComparer;
    pub fn AddNew(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataRowView> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRowView = __cordl_object
            .invoke("AddNew", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckOpen(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckOpen", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckSort(
        &mut self,
        sort: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckSort", (sort))?;
        Ok(__cordl_ret)
    }
    pub fn ChildRelationCollectionChanged(
        &mut self,
        sender: *mut crate::System::Object,
        e: *mut crate::System::ComponentModel::CollectionChangeEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChildRelationCollectionChanged", (sender, e))?;
        Ok(__cordl_ret)
    }
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret)
    }
    pub fn ColumnCollectionChanged(
        &mut self,
        sender: *mut crate::System::Object,
        e: *mut crate::System::ComponentModel::CollectionChangeEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ColumnCollectionChanged", (sender, e))?;
        Ok(__cordl_ret)
    }
    pub fn ColumnCollectionChangedInternal(
        &mut self,
        sender: *mut crate::System::Object,
        e: *mut crate::System::ComponentModel::CollectionChangeEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ColumnCollectionChangedInternal", (sender, e))?;
        Ok(__cordl_ret)
    }
    pub fn CopyTo_Array0(
        &mut self,
        array: *mut crate::System::Array,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (array, index))?;
        Ok(__cordl_ret)
    }
    pub fn CopyTo_Il2CppArray1(
        &mut self,
        array: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataRowView,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (array, index))?;
        Ok(__cordl_ret)
    }
    pub fn CreateSortString(
        &mut self,
        property: *mut crate::System::ComponentModel::PropertyDescriptor,
        direction: crate::System::ComponentModel::ListSortDirection,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("CreateSortString", (property, direction))?;
        Ok(__cordl_ret)
    }
    pub fn Delete_DataRow1(
        &mut self,
        row: *mut crate::System::Data::DataRow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Delete", (row))?;
        Ok(__cordl_ret)
    }
    pub fn Delete_i32_0(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Delete", (index))?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret)
    }
    pub fn FinishAddNew(
        &mut self,
        success: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishAddNew", (success))?;
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetFilter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::IFilter> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::IFilter = __cordl_object
            .invoke("GetFilter", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetFindIndex(
        &mut self,
        column: *mut crate::System::String,
        keepIndex: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::Index> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::Index = __cordl_object
            .invoke("GetFindIndex", (column, keepIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetRecord(&mut self, recordIndex: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetRecord", (recordIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetRow(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataRow> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRow = __cordl_object
            .invoke("GetRow", (index))?;
        Ok(__cordl_ret)
    }
    pub fn GetRowView_DataRow1(
        &mut self,
        dr: *mut crate::System::Data::DataRow,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataRowView> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRowView = __cordl_object
            .invoke("GetRowView", (dr))?;
        Ok(__cordl_ret)
    }
    pub fn GetRowView_i32_0(
        &mut self,
        record: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataRowView> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRowView = __cordl_object
            .invoke("GetRowView", (record))?;
        Ok(__cordl_ret)
    }
    pub fn GetSortProperty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptor,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptor = __cordl_object
            .invoke("GetSortProperty", ())?;
        Ok(__cordl_ret)
    }
    pub fn IndexListChanged(
        &mut self,
        sender: *mut crate::System::Object,
        e: *mut crate::System::ComponentModel::ListChangedEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IndexListChanged", (sender, e))?;
        Ok(__cordl_ret)
    }
    pub fn IndexListChangedInternal(
        &mut self,
        e: *mut crate::System::ComponentModel::ListChangedEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IndexListChangedInternal", (e))?;
        Ok(__cordl_ret)
    }
    pub fn IndexOf(
        &mut self,
        rowview: *mut crate::System::Data::DataRowView,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("IndexOf", (rowview))?;
        Ok(__cordl_ret)
    }
    pub fn IndexOfDataRowView(
        &mut self,
        rowview: *mut crate::System::Data::DataRowView,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("IndexOfDataRowView", (rowview))?;
        Ok(__cordl_ret)
    }
    pub fn MaintainDataView(
        &mut self,
        changedType: crate::System::ComponentModel::ListChangedType,
        row: *mut crate::System::Data::DataRow,
        trackAddRemove: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MaintainDataView", (changedType, row, trackAddRemove))?;
        Ok(__cordl_ret)
    }
    pub fn New_DataTable1(
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (table))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool0(
        table: *mut crate::System::Data::DataTable,
        locked: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (table, locked))?;
        Ok(__cordl_object)
    }
    pub fn OnListChanged(
        &mut self,
        e: *mut crate::System::ComponentModel::ListChangedEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnListChanged", (e))?;
        Ok(__cordl_ret)
    }
    pub fn ParentRelationCollectionChanged(
        &mut self,
        sender: *mut crate::System::Object,
        e: *mut crate::System::ComponentModel::CollectionChangeEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParentRelationCollectionChanged", (sender, e))?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResetRowViewCache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetRowViewCache", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetDataViewManager(
        &mut self,
        dataViewManager: *mut crate::System::Data::DataViewManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDataViewManager", (dataViewManager))?;
        Ok(__cordl_ret)
    }
    pub fn SetIndex(
        &mut self,
        newSort: *mut crate::System::String,
        newRowStates: crate::System::Data::DataViewRowState,
        newRowFilter: *mut crate::System::Data::IFilter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIndex", (newSort, newRowStates, newRowFilter))?;
        Ok(__cordl_ret)
    }
    pub fn SetIndex2(
        &mut self,
        newSort: *mut crate::System::String,
        newRowStates: crate::System::Data::DataViewRowState,
        newRowFilter: *mut crate::System::Data::IFilter,
        fireEvent: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIndex2", (newSort, newRowStates, newRowFilter, fireEvent))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_ICollection_get_IsSynchronized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Collections.ICollection.get_IsSynchronized", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_ICollection_get_SyncRoot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("System.Collections.ICollection.get_SyncRoot", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IList_Add(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.Collections.IList.Add", (value))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IList_Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.IList.Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IList_Contains(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Collections.IList.Contains", (value))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IList_IndexOf(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.Collections.IList.IndexOf", (value))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IList_Insert(
        &mut self,
        index: i32,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.IList.Insert", (index, value))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IList_Remove(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.IList.Remove", (value))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IList_RemoveAt(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.IList.RemoveAt", (index))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IList_get_IsFixedSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Collections.IList.get_IsFixedSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IList_get_IsReadOnly(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Collections.IList.get_IsReadOnly", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IList_get_Item(
        &mut self,
        recordIndex: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("System.Collections.IList.get_Item", (recordIndex))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IList_set_Item(
        &mut self,
        recordIndex: i32,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.IList.set_Item", (recordIndex, value))?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_IBindingList_AddIndex(
        &mut self,
        property: *mut crate::System::ComponentModel::PropertyDescriptor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.ComponentModel.IBindingList.AddIndex", (property))?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_IBindingList_AddNew(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("System.ComponentModel.IBindingList.AddNew", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_IBindingList_ApplySort(
        &mut self,
        property: *mut crate::System::ComponentModel::PropertyDescriptor,
        direction: crate::System::ComponentModel::ListSortDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.ComponentModel.IBindingList.ApplySort",
                (property, direction),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_IBindingList_Find(
        &mut self,
        property: *mut crate::System::ComponentModel::PropertyDescriptor,
        key: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.ComponentModel.IBindingList.Find", (property, key))?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_IBindingList_RemoveIndex(
        &mut self,
        property: *mut crate::System::ComponentModel::PropertyDescriptor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.ComponentModel.IBindingList.RemoveIndex", (property))?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_IBindingList_RemoveSort(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.ComponentModel.IBindingList.RemoveSort", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_IBindingList_get_AllowEdit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.ComponentModel.IBindingList.get_AllowEdit", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_IBindingList_get_AllowNew(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.ComponentModel.IBindingList.get_AllowNew", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_IBindingList_get_AllowRemove(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.ComponentModel.IBindingList.get_AllowRemove", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_IBindingList_get_IsSorted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.ComponentModel.IBindingList.get_IsSorted", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_IBindingList_get_SortDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ComponentModel::ListSortDirection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ComponentModel::ListSortDirection = __cordl_object
            .invoke("System.ComponentModel.IBindingList.get_SortDirection", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_IBindingList_get_SortProperty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptor,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptor = __cordl_object
            .invoke("System.ComponentModel.IBindingList.get_SortProperty", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_IBindingList_get_SupportsChangeNotification(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "System.ComponentModel.IBindingList.get_SupportsChangeNotification",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_IBindingList_get_SupportsSearching(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.ComponentModel.IBindingList.get_SupportsSearching", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_IBindingList_get_SupportsSorting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.ComponentModel.IBindingList.get_SupportsSorting", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ITypedList_GetItemProperties(
        &mut self,
        listAccessors: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::ComponentModel::PropertyDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptorCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptorCollection = __cordl_object
            .invoke(
                "System.ComponentModel.ITypedList.GetItemProperties",
                (listAccessors),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ITypedList_GetListName(
        &mut self,
        listAccessors: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::ComponentModel::PropertyDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("System.ComponentModel.ITypedList.GetListName", (listAccessors))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateIndex_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateIndex__cordl_bool1(
        &mut self,
        force: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateIndex", (force))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateIndex__cordl_bool__cordl_bool2(
        &mut self,
        force: bool,
        fireEvent: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateIndex", (force, fireEvent))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DataTable1(
        &mut self,
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (table))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool0(
        &mut self,
        table: *mut crate::System::Data::DataTable,
        locked: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (table, locked))?;
        Ok(__cordl_ret)
    }
    pub fn get_AllowDelete(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_AllowDelete", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AllowEdit(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_AllowEdit", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AllowNew(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_AllowNew", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CountFromIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_CountFromIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DataViewManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataViewManager> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataViewManager = __cordl_object
            .invoke("get_DataViewManager", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsOpen(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsOpen", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Item(
        &mut self,
        recordIndex: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataRowView> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRowView = __cordl_object
            .invoke("get_Item", (recordIndex))?;
        Ok(__cordl_ret)
    }
    pub fn get_ObjectID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ObjectID", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RowStateFilter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::DataViewRowState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::DataViewRowState = __cordl_object
            .invoke("get_RowStateFilter", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Sort(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Sort", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SortComparison(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Comparison_1<*mut crate::System::Data::DataRow>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Comparison_1<
            *mut crate::System::Data::DataRow,
        > = __cordl_object.invoke("get_SortComparison", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Table(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataTable = __cordl_object
            .invoke("get_Table", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Sort(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Sort", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+DataView")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::DataView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
