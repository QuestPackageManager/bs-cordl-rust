#[cfg(feature = "System+Data+Index")]
#[repr(C)]
#[derive(Debug)]
pub struct Index {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    pub _indexFields: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::System::Data::IndexField>,
    >,
    pub _comparison: quest_hook::libil2cpp::Gc<
        crate::System::Comparison_1<
            quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        >,
    >,
    pub _recordStates: crate::System::Data::DataViewRowState,
    pub _rowFilter: quest_hook::libil2cpp::Gc<crate::System::WeakReference>,
    pub _records: quest_hook::libil2cpp::Gc<crate::System::Data::Index_IndexTree>,
    pub _recordCount: i32,
    pub _refCount: i32,
    pub _listeners: quest_hook::libil2cpp::Gc<
        crate::System::Data::Listeners_1<
            quest_hook::libil2cpp::Gc<crate::System::Data::DataViewListener>,
        >,
    >,
    pub _suspendEvents: bool,
    pub _isSharable: bool,
    pub _hasRemoteAggregate: bool,
    pub _objectID: i32,
}
#[cfg(feature = "System+Data+Index")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Data::Index {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "Index";
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
#[cfg(feature = "System+Data+Index")]
impl std::ops::Deref for crate::System::Data::Index {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Index")]
impl std::ops::DerefMut for crate::System::Data::Index {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Index")]
impl crate::System::Data::Index {
    #[cfg(feature = "System+Data+Index+IndexTree")]
    pub type IndexTree = crate::System::Data::Index_IndexTree;
    pub fn AcceptRecord_IFilter1(
        &mut self,
        record: i32,
        filter: quest_hook::libil2cpp::Gc<crate::System::Data::IFilter>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AcceptRecord", (record, filter))?;
        Ok(__cordl_ret.into())
    }
    pub fn AcceptRecord_i32_0(
        &mut self,
        record: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AcceptRecord", (record))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddRef(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddRef", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyChangeAction(
        &mut self,
        record: i32,
        action: i32,
        changeRecord: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyChangeAction", (record, action, changeRecord))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckUnique(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CheckUnique", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareDataRows(
        &mut self,
        record1: i32,
        record2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CompareDataRows", (record1, record2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareDuplicateRecords(
        &mut self,
        record1: i32,
        record2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CompareDuplicateRecords", (record1, record2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareRecordToKey(
        &mut self,
        record1: i32,
        vals: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CompareRecordToKey", (record1, vals))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareRecords(
        &mut self,
        record1: i32,
        record2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CompareRecords", (record1, record2))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteRecordFromIndex(
        &mut self,
        recordIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeleteRecordFromIndex", (recordIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteRecord__cordl_bool1(
        &mut self,
        recordIndex: i32,
        fireEvent: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeleteRecord", (recordIndex, fireEvent))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteRecord_i32_0(
        &mut self,
        recordIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeleteRecord", (recordIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equal(
        &mut self,
        indexDesc: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::Data::IndexField>,
        >,
        recordStates: crate::System::Data::DataViewRowState,
        rowFilter: quest_hook::libil2cpp::Gc<crate::System::Data::IFilter>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Equal", (indexDesc, recordStates, rowFilter))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindNodeByKey(
        &mut self,
        originalKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FindNodeByKey", (originalKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindNodeByKeyRecord(
        &mut self,
        record: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FindNodeByKeyRecord", (record))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindNodeByKeys(
        &mut self,
        originalKey: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FindNodeByKeys", (originalKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindRecords_Il2CppArray1(
        &mut self,
        key: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::Range> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::Range = __cordl_object
            .invoke("FindRecords", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindRecords_Il2CppObject0(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::Range> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::Range = __cordl_object
            .invoke("FindRecords", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn FireResetEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FireResetEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllFields(
        columns: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumnCollection>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::Data::IndexField>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::Data::IndexField>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAllFields", (columns))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetChangeAction(
        &mut self,
        oldState: crate::System::Data::DataViewRowState,
        newState: crate::System::Data::DataViewRowState,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetChangeAction", (oldState, newState))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Data::RBTree_1_RBTreeEnumerator<i32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::RBTree_1_RBTreeEnumerator<i32> = __cordl_object
            .invoke("GetEnumerator", (startIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIndex_i32_0(&mut self, record: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetIndex", (record))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIndex_i32_1(
        &mut self,
        record: i32,
        changeRecord: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetIndex", (record, changeRecord))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRangeFromNode(
        &mut self,
        nodeId: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::Range> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::Range = __cordl_object
            .invoke("GetRangeFromNode", (nodeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRecord(&mut self, recordIndex: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetRecord", (recordIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetReplaceAction(
        oldState: crate::System::Data::DataViewRowState,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetReplaceAction", (oldState))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRow(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow> = __cordl_object
            .invoke("GetRow", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRows_Il2CppArray0(
        &mut self,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
            >,
        > = __cordl_object.invoke("GetRows", (values))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRows_Range1(
        &mut self,
        range: crate::System::Data::Range,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
            >,
        > = __cordl_object.invoke("GetRows", (range))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUniqueKeyValues_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = __cordl_object.invoke("GetUniqueKeyValues", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUniqueKeyValues_List_1_i32_1(
        &mut self,
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    >,
                >,
            >,
        >,
        curNodeId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetUniqueKeyValues", (list, curNodeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfReference<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        item: T,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOfReference", (list, item))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitRecords(
        &mut self,
        filter: quest_hook::libil2cpp::Gc<crate::System::Data::IFilter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitRecords", (filter))?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertRecord(
        &mut self,
        record: i32,
        fireEvent: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("InsertRecord", (record, fireEvent))?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertRecordToIndex(
        &mut self,
        record: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("InsertRecordToIndex", (record))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsKeyInIndex_Il2CppArray1(
        &mut self,
        key: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsKeyInIndex", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsKeyInIndex_Il2CppObject0(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsKeyInIndex", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsKeyRecordInIndex(
        &mut self,
        record: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsKeyRecordInIndex", (record))?;
        Ok(__cordl_ret.into())
    }
    pub fn ListChangedAdd(
        &mut self,
        listener: quest_hook::libil2cpp::Gc<crate::System::Data::DataViewListener>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ListChangedAdd", (listener))?;
        Ok(__cordl_ret.into())
    }
    pub fn ListChangedRemove(
        &mut self,
        listener: quest_hook::libil2cpp::Gc<crate::System::Data::DataViewListener>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ListChangedRemove", (listener))?;
        Ok(__cordl_ret.into())
    }
    pub fn MaintainDataView(
        &mut self,
        changedType: crate::System::ComponentModel::ListChangedType,
        record: i32,
        trackAddRemove: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MaintainDataView", (changedType, record, trackAddRemove))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Comparison_1_DataViewRowState_IFilter1(
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        comparison: quest_hook::libil2cpp::Gc<
            crate::System::Comparison_1<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
            >,
        >,
        recordStates: crate::System::Data::DataViewRowState,
        rowFilter: quest_hook::libil2cpp::Gc<crate::System::Data::IFilter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (table, comparison, recordStates, rowFilter))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppArray_Comparison_1_DataViewRowState_IFilter2(
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        indexFields: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::Data::IndexField>,
        >,
        comparison: quest_hook::libil2cpp::Gc<
            crate::System::Comparison_1<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
            >,
        >,
        recordStates: crate::System::Data::DataViewRowState,
        rowFilter: quest_hook::libil2cpp::Gc<crate::System::Data::IFilter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (table, indexFields, comparison, recordStates, rowFilter),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppArray_DataViewRowState_IFilter0(
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        indexFields: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::Data::IndexField>,
        >,
        recordStates: crate::System::Data::DataViewRowState,
        rowFilter: quest_hook::libil2cpp::Gc<crate::System::Data::IFilter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (table, indexFields, recordStates, rowFilter))?;
        Ok(__cordl_object.into())
    }
    pub fn OnListChanged_ListChangedEventArgs2(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::ComponentModel::ListChangedEventArgs>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnListChanged", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnListChanged_ListChangedType_i32_1(
        &mut self,
        changedType: crate::System::ComponentModel::ListChangedType,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnListChanged", (changedType, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnListChanged_ListChangedType_i32_i32_0(
        &mut self,
        changedType: crate::System::ComponentModel::ListChangedType,
        newIndex: i32,
        oldIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnListChanged", (changedType, newIndex, oldIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecordChanged_i32_0(
        &mut self,
        record: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecordChanged", (record))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecordChanged_i32_1(
        &mut self,
        oldIndex: i32,
        newIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecordChanged", (oldIndex, newIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecordStateChanged_i32_DataViewRowState_DataViewRowState0(
        &mut self,
        record: i32,
        oldState: crate::System::Data::DataViewRowState,
        newState: crate::System::Data::DataViewRowState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecordStateChanged", (record, oldState, newState))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecordStateChanged_i32_DataViewRowState_DataViewRowState1(
        &mut self,
        oldRecord: i32,
        oldOldState: crate::System::Data::DataViewRowState,
        oldNewState: crate::System::Data::DataViewRowState,
        newRecord: i32,
        newOldState: crate::System::Data::DataViewRowState,
        newNewState: crate::System::Data::DataViewRowState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RecordStateChanged",
                (
                    oldRecord,
                    oldOldState,
                    oldNewState,
                    newRecord,
                    newOldState,
                    newNewState,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveRef(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("RemoveRef", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Comparison_1_DataViewRowState_IFilter1(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        comparison: quest_hook::libil2cpp::Gc<
            crate::System::Comparison_1<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
            >,
        >,
        recordStates: crate::System::Data::DataViewRowState,
        rowFilter: quest_hook::libil2cpp::Gc<crate::System::Data::IFilter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (table, comparison, recordStates, rowFilter))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray_Comparison_1_DataViewRowState_IFilter2(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        indexFields: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::Data::IndexField>,
        >,
        comparison: quest_hook::libil2cpp::Gc<
            crate::System::Comparison_1<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
            >,
        >,
        recordStates: crate::System::Data::DataViewRowState,
        rowFilter: quest_hook::libil2cpp::Gc<crate::System::Data::IFilter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (table, indexFields, comparison, recordStates, rowFilter))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray_DataViewRowState_IFilter0(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        indexFields: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::Data::IndexField>,
        >,
        recordStates: crate::System::Data::DataViewRowState,
        rowFilter: quest_hook::libil2cpp::Gc<crate::System::Data::IFilter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (table, indexFields, recordStates, rowFilter))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DoListChanged(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_DoListChanged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasDuplicates(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasDuplicates", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasRemoteAggregate(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasRemoteAggregate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ObjectID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ObjectID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RecordCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_RecordCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RecordStates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::DataViewRowState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::DataViewRowState = __cordl_object
            .invoke("get_RecordStates", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RefCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_RefCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RowFilter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::IFilter>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::IFilter> = __cordl_object
            .invoke("get_RowFilter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Table(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable> = __cordl_object
            .invoke("get_Table", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+Index")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::Index {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Data+Index+IndexTree")]
#[repr(C)]
#[derive(Debug)]
pub struct Index_IndexTree {
    __cordl_parent: crate::System::Data::RBTree_1<i32>,
    pub _index: quest_hook::libil2cpp::Gc<crate::System::Data::Index>,
}
#[cfg(feature = "System+Data+Index+IndexTree")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Data::Index_IndexTree {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "Index/IndexTree";
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
#[cfg(feature = "System+Data+Index+IndexTree")]
impl std::ops::Deref for crate::System::Data::Index_IndexTree {
    type Target = crate::System::Data::RBTree_1<i32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Index+IndexTree")]
impl std::ops::DerefMut for crate::System::Data::Index_IndexTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Index+IndexTree")]
impl crate::System::Data::Index_IndexTree {
    pub fn CompareNode(
        &mut self,
        record1: i32,
        record2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareNode", (record1, record2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareSateliteTreeNode(
        &mut self,
        record1: i32,
        record2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CompareSateliteTreeNode", (record1, record2))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        index: quest_hook::libil2cpp::Gc<crate::System::Data::Index>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (index))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        index: quest_hook::libil2cpp::Gc<crate::System::Data::Index>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (index))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+Index+IndexTree")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::Index_IndexTree {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
