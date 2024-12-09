#[cfg(feature = "System+Data+DataTable")]
#[repr(C)]
#[derive(Debug)]
pub struct DataTable {
    __cordl_parent: crate::System::ComponentModel::MarshalByValueComponent,
    pub _dataSet: *mut crate::System::Data::DataSet,
    pub _defaultView: *mut crate::System::Data::DataView,
    pub _nextRowID: i64,
    pub _rowCollection: *mut crate::System::Data::DataRowCollection,
    pub _columnCollection: *mut crate::System::Data::DataColumnCollection,
    pub _constraintCollection: *mut crate::System::Data::ConstraintCollection,
    pub _elementColumnCount: i32,
    pub _parentRelationsCollection: *mut crate::System::Data::DataRelationCollection,
    pub _childRelationsCollection: *mut crate::System::Data::DataRelationCollection,
    pub _recordManager: *mut crate::System::Data::RecordManager,
    pub _indexes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Data::Index,
    >,
    pub _shadowIndexes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Data::Index,
    >,
    pub _shadowCount: i32,
    pub _extendedProperties: *mut crate::System::Data::PropertyCollection,
    pub _tableName: *mut crate::System::String,
    pub _tableNamespace: *mut crate::System::String,
    pub _tablePrefix: *mut crate::System::String,
    pub _displayExpression: *mut crate::System::Data::DataExpression,
    pub _fNestedInDataset: bool,
    pub _culture: *mut crate::System::Globalization::CultureInfo,
    pub _cultureUserSet: bool,
    pub _compareInfo: *mut crate::System::Globalization::CompareInfo,
    pub _compareFlags: crate::System::Globalization::CompareOptions,
    pub _formatProvider: *mut crate::System::IFormatProvider,
    pub _hashCodeProvider: *mut crate::System::StringComparer,
    pub _caseSensitive: bool,
    pub _caseSensitiveUserSet: bool,
    pub _encodedTableName: *mut crate::System::String,
    pub _xmlText: *mut crate::System::Data::DataColumn,
    pub _colUnique: *mut crate::System::Data::DataColumn,
    pub _minOccurs: crate::System::Decimal,
    pub _maxOccurs: crate::System::Decimal,
    pub _repeatableElement: bool,
    pub _typeName: *mut crate::System::Object,
    pub _primaryKey: *mut crate::System::Data::UniqueConstraint,
    pub _primaryIndex: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Data::IndexField,
    >,
    pub _delayedSetPrimaryKey: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Data::DataColumn,
    >,
    pub _loadIndex: *mut crate::System::Data::Index,
    pub _loadIndexwithOriginalAdded: *mut crate::System::Data::Index,
    pub _loadIndexwithCurrentDeleted: *mut crate::System::Data::Index,
    pub _suspendIndexEvents: i32,
    pub _inDataLoad: bool,
    pub _schemaLoading: bool,
    pub _enforceConstraints: bool,
    pub _suspendEnforceConstraints: bool,
    pub fInitInProgress: bool,
    pub _inLoad: bool,
    pub _fInLoadDiffgram: bool,
    pub _isTypedDataTable: u8,
    pub _emptyDataRowArray: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Data::DataRow,
    >,
    pub _propertyDescriptorCollectionCache: *mut crate::System::ComponentModel::PropertyDescriptorCollection,
    pub _nestedParentRelations: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Data::DataRelation,
    >,
    pub _dependentColumns: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Data::DataColumn,
    >,
    pub _mergingData: bool,
    pub _onRowChangedDelegate: *mut crate::System::Data::DataRowChangeEventHandler,
    pub _onRowChangingDelegate: *mut crate::System::Data::DataRowChangeEventHandler,
    pub _onRowDeletingDelegate: *mut crate::System::Data::DataRowChangeEventHandler,
    pub _onRowDeletedDelegate: *mut crate::System::Data::DataRowChangeEventHandler,
    pub _onColumnChangedDelegate: *mut crate::System::Data::DataColumnChangeEventHandler,
    pub _onColumnChangingDelegate: *mut crate::System::Data::DataColumnChangeEventHandler,
    pub _onTableClearingDelegate: *mut crate::System::Data::DataTableClearEventHandler,
    pub _onTableClearedDelegate: *mut crate::System::Data::DataTableClearEventHandler,
    pub _onTableNewRowDelegate: *mut crate::System::Data::DataTableNewRowEventHandler,
    pub _onPropertyChangingDelegate: *mut crate::System::ComponentModel::PropertyChangedEventHandler,
    pub _rowBuilder: *mut crate::System::Data::DataRowBuilder,
    pub _delayedViews: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Data::DataView,
    >,
    pub _dataViewListeners: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Data::DataViewListener,
    >,
    pub _rowDiffId: *mut crate::System::Collections::Hashtable,
    pub _indexesLock: *mut crate::System::Threading::ReaderWriterLockSlim,
    pub _ukColumnPositionForInference: i32,
    pub _remotingFormat: crate::System::Data::SerializationFormat,
    pub _objectID: i32,
}
#[cfg(feature = "System+Data+DataTable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::DataTable => "System.Data"
    ."DataTable"
);
#[cfg(feature = "System+Data+DataTable")]
impl std::ops::Deref for crate::System::Data::DataTable {
    type Target = crate::System::ComponentModel::MarshalByValueComponent;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataTable")]
impl std::ops::DerefMut for crate::System::Data::DataTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataTable")]
impl crate::System::Data::DataTable {
    #[cfg(feature = "System+Data+DataTable+DSRowDiffIdUsageSection")]
    pub type DSRowDiffIdUsageSection = crate::System::Data::DataTable_DSRowDiffIdUsageSection;
    #[cfg(feature = "System+Data+DataTable+RowDiffIdUsageSection")]
    pub type RowDiffIdUsageSection = crate::System::Data::DataTable_RowDiffIdUsageSection;
    pub fn AddDependentColumn(
        &mut self,
        expressionColumn: *mut crate::System::Data::DataColumn,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddDependentColumn", (expressionColumn))?;
        Ok(__cordl_ret)
    }
    pub fn AddForeignKey(
        &mut self,
        parentKey: *mut crate::System::Data::DataColumn,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataColumn> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataColumn = __cordl_object
            .invoke("AddForeignKey", (parentKey))?;
        Ok(__cordl_ret)
    }
    pub fn AddRow(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        proposedID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddRow", (row, proposedID))?;
        Ok(__cordl_ret)
    }
    pub fn AddUniqueKey_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataColumn> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataColumn = __cordl_object
            .invoke("AddUniqueKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddUniqueKey_i32_0(
        &mut self,
        position: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataColumn> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataColumn = __cordl_object
            .invoke("AddUniqueKey", (position))?;
        Ok(__cordl_ret)
    }
    pub fn CacheNestedParent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CacheNestedParent", ())?;
        Ok(__cordl_ret)
    }
    pub fn CascadeAll(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        action: crate::System::Data::DataRowAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CascadeAll", (row, action))?;
        Ok(__cordl_ret)
    }
    pub fn CheckCascadingNamespaceConflict(
        &mut self,
        realNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckCascadingNamespaceConflict", (realNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn CheckForClosureOnExpressionTables(
        &mut self,
        tableList: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Data::DataTable,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckForClosureOnExpressionTables", (tableList))?;
        Ok(__cordl_ret)
    }
    pub fn CheckForClosureOnExpressions(
        &mut self,
        dt: *mut crate::System::Data::DataTable,
        writeHierarchy: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckForClosureOnExpressions", (dt, writeHierarchy))?;
        Ok(__cordl_ret)
    }
    pub fn CheckNamespaceValidityForNestedParentRelations(
        &mut self,
        ns: *mut crate::System::String,
        parentTable: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CheckNamespaceValidityForNestedParentRelations",
                (ns, parentTable),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CheckNamespaceValidityForNestedRelations(
        &mut self,
        realNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckNamespaceValidityForNestedRelations", (realNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn CheckNotModifying(
        &mut self,
        row: *mut crate::System::Data::DataRow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckNotModifying", (row))?;
        Ok(__cordl_ret)
    }
    pub fn Clear_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn Clear__cordl_bool1(
        &mut self,
        clearAll: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", (clearAll))?;
        Ok(__cordl_ret)
    }
    pub fn CloneHierarchy(
        &mut self,
        sourceTable: *mut crate::System::Data::DataTable,
        ds: *mut crate::System::Data::DataSet,
        visitedMap: *mut crate::System::Collections::Hashtable,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataTable = __cordl_object
            .invoke("CloneHierarchy", (sourceTable, ds, visitedMap))?;
        Ok(__cordl_ret)
    }
    pub fn CloneTo(
        &mut self,
        clone: *mut crate::System::Data::DataTable,
        cloneDS: *mut crate::System::Data::DataSet,
        skipExpressionColumns: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataTable = __cordl_object
            .invoke("CloneTo", (clone, cloneDS, skipExpressionColumns))?;
        Ok(__cordl_ret)
    }
    pub fn Clone_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataTable = __cordl_object
            .invoke("Clone", ())?;
        Ok(__cordl_ret)
    }
    pub fn Clone_DataSet1(
        &mut self,
        cloneDS: *mut crate::System::Data::DataSet,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataTable = __cordl_object
            .invoke("Clone", (cloneDS))?;
        Ok(__cordl_ret)
    }
    pub fn CommitRow(
        &mut self,
        row: *mut crate::System::Data::DataRow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CommitRow", (row))?;
        Ok(__cordl_ret)
    }
    pub fn Compare_CompareInfo1(
        &mut self,
        s1: *mut crate::System::String,
        s2: *mut crate::System::String,
        comparer: *mut crate::System::Globalization::CompareInfo,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (s1, s2, comparer))?;
        Ok(__cordl_ret)
    }
    pub fn Compare_String_String0(
        &mut self,
        s1: *mut crate::System::String,
        s2: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (s1, s2))?;
        Ok(__cordl_ret)
    }
    pub fn ConvertToRowError(
        &mut self,
        rowIndex: i32,
        rowErrors: *mut crate::System::Collections::Hashtable,
        colErrors: *mut crate::System::Collections::Hashtable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConvertToRowError", (rowIndex, rowErrors, colErrors))?;
        Ok(__cordl_ret)
    }
    pub fn ConvertToRowState(
        &mut self,
        bitStates: *mut crate::System::Collections::BitArray,
        bitIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::DataRowState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::DataRowState = __cordl_object
            .invoke("ConvertToRowState", (bitStates, bitIndex))?;
        Ok(__cordl_ret)
    }
    pub fn CreateEmptyRow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataRow> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRow = __cordl_object
            .invoke("CreateEmptyRow", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateInstance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataTable = __cordl_object
            .invoke("CreateInstance", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateRelationList(
        &mut self,
        tableList: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Data::DataTable,
        >,
        relationList: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Data::DataRelation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateRelationList", (tableList, relationList))?;
        Ok(__cordl_ret)
    }
    pub fn CreateTableList(
        &mut self,
        currentTable: *mut crate::System::Data::DataTable,
        tableList: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Data::DataTable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateTableList", (currentTable, tableList))?;
        Ok(__cordl_ret)
    }
    pub fn DeleteRow(
        &mut self,
        row: *mut crate::System::Data::DataRow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeleteRow", (row))?;
        Ok(__cordl_ret)
    }
    pub fn DeserializeConstraints(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
        serIndex: i32,
        allConstraints: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DeserializeConstraints",
                (info, context, serIndex, allConstraints),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DeserializeDataTable(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
        isSingleTable: bool,
        remotingFormat: crate::System::Data::SerializationFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DeserializeDataTable",
                (info, context, isSingleTable, remotingFormat),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DeserializeExpressionColumns(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
        serIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeserializeExpressionColumns", (info, context, serIndex))?;
        Ok(__cordl_ret)
    }
    pub fn DeserializeTableData(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
        serIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeserializeTableData", (info, context, serIndex))?;
        Ok(__cordl_ret)
    }
    pub fn DeserializeTableSchema(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
        isSingleTable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeserializeTableSchema", (info, context, isSingleTable))?;
        Ok(__cordl_ret)
    }
    pub fn DoRaiseNamespaceChange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoRaiseNamespaceChange", ())?;
        Ok(__cordl_ret)
    }
    pub fn EnableConstraints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnableConstraints", ())?;
        Ok(__cordl_ret)
    }
    pub fn EvaluateDependentExpressions_DataColumn0(
        &mut self,
        column: *mut crate::System::Data::DataColumn,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EvaluateDependentExpressions", (column))?;
        Ok(__cordl_ret)
    }
    pub fn EvaluateDependentExpressions_List_1_DataRow_DataRowVersion_List_1_1(
        &mut self,
        columns: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Data::DataColumn,
        >,
        row: *mut crate::System::Data::DataRow,
        version: crate::System::Data::DataRowVersion,
        cachedRows: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Data::DataRow,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "EvaluateDependentExpressions",
                (columns, row, version, cachedRows),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EvaluateExpressions_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EvaluateExpressions", ())?;
        Ok(__cordl_ret)
    }
    pub fn EvaluateExpressions_DataColumn2(
        &mut self,
        column: *mut crate::System::Data::DataColumn,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EvaluateExpressions", (column))?;
        Ok(__cordl_ret)
    }
    pub fn EvaluateExpressions_DataRow_DataRowAction_List_1_1(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        action: crate::System::Data::DataRowAction,
        cachedRows: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Data::DataRow,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EvaluateExpressions", (row, action, cachedRows))?;
        Ok(__cordl_ret)
    }
    pub fn FindByIndex(
        &mut self,
        ndx: *mut crate::System::Data::Index,
        key: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataRow> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRow = __cordl_object
            .invoke("FindByIndex", (ndx, key))?;
        Ok(__cordl_ret)
    }
    pub fn FindMergeTarget(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        key: crate::System::Data::DataKey,
        ndx: *mut crate::System::Data::Index,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataRow> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRow = __cordl_object
            .invoke("FindMergeTarget", (row, key, ndx))?;
        Ok(__cordl_ret)
    }
    pub fn FindNestedParentRelations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataRelation>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataRelation,
        > = __cordl_object.invoke("FindNestedParentRelations", ())?;
        Ok(__cordl_ret)
    }
    pub fn FormatSortString(
        &mut self,
        indexDesc: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Data::IndexField,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("FormatSortString", (indexDesc))?;
        Ok(__cordl_ret)
    }
    pub fn FreeRecord(
        &mut self,
        record: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FreeRecord", (record))?;
        Ok(__cordl_ret)
    }
    pub fn GetIndex_Il2CppArray1(
        &mut self,
        indexDesc: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Data::IndexField,
        >,
        recordStates: crate::System::Data::DataViewRowState,
        rowFilter: *mut crate::System::Data::IFilter,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::Index> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::Index = __cordl_object
            .invoke("GetIndex", (indexDesc, recordStates, rowFilter))?;
        Ok(__cordl_ret)
    }
    pub fn GetIndex_String0(
        &mut self,
        sort: *mut crate::System::String,
        recordStates: crate::System::Data::DataViewRowState,
        rowFilter: *mut crate::System::Data::IFilter,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::Index> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::Index = __cordl_object
            .invoke("GetIndex", (sort, recordStates, rowFilter))?;
        Ok(__cordl_ret)
    }
    pub fn GetInheritedNamespace(
        &mut self,
        visitedTables: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Data::DataTable,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetInheritedNamespace", (visitedTables))?;
        Ok(__cordl_ret)
    }
    pub fn GetListeners(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Data::DataViewListener,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Data::DataViewListener,
        > = __cordl_object.invoke("GetListeners", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetObjectData(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetObjectData", (info, context))?;
        Ok(__cordl_ret)
    }
    pub fn GetPropertyDescriptorCollection(
        &mut self,
        attributes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Attribute,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptorCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptorCollection = __cordl_object
            .invoke("GetPropertyDescriptorCollection", (attributes))?;
        Ok(__cordl_ret)
    }
    pub fn GetRowAndColumnErrors(
        &mut self,
        rowIndex: i32,
        rowErrors: *mut crate::System::Collections::Hashtable,
        colErrors: *mut crate::System::Collections::Hashtable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetRowAndColumnErrors", (rowIndex, rowErrors, colErrors))?;
        Ok(__cordl_ret)
    }
    pub fn GetRowType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("GetRowType", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSchema(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::XmlSchema> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchema = __cordl_object
            .invoke("GetSchema", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSpecialHashCode(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetSpecialHashCode", (name))?;
        Ok(__cordl_ret)
    }
    pub fn IncrementalCloneTo(
        &mut self,
        sourceTable: *mut crate::System::Data::DataTable,
        targetTable: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataTable = __cordl_object
            .invoke("IncrementalCloneTo", (sourceTable, targetTable))?;
        Ok(__cordl_ret)
    }
    pub fn IndexOf(
        &mut self,
        s1: *mut crate::System::String,
        s2: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("IndexOf", (s1, s2))?;
        Ok(__cordl_ret)
    }
    pub fn InsertRecordToIndexes(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        version: crate::System::Data::DataRowVersion,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<i32> = __cordl_object
            .invoke("InsertRecordToIndexes", (row, version))?;
        Ok(__cordl_ret)
    }
    pub fn InsertRow_i32_i32_0(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        proposedID: i32,
        pos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InsertRow", (row, proposedID, pos))?;
        Ok(__cordl_ret)
    }
    pub fn InsertRow_i64_2(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        proposedID: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InsertRow", (row, proposedID))?;
        Ok(__cordl_ret)
    }
    pub fn InsertRow_i64_i32__cordl_bool1(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        proposedID: i64,
        pos: i32,
        fireEvent: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InsertRow", (row, proposedID, pos, fireEvent))?;
        Ok(__cordl_ret)
    }
    pub fn IsEmptyXml(
        &mut self,
        reader: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEmptyXml", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn IsNamespaceInherited(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsNamespaceInherited", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsSuffix(
        &mut self,
        s1: *mut crate::System::String,
        s2: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSuffix", (s1, s2))?;
        Ok(__cordl_ret)
    }
    pub fn MergeRow(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        targetRow: *mut crate::System::Data::DataRow,
        preserveChanges: bool,
        idxSearch: *mut crate::System::Data::Index,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataRow> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRow = __cordl_object
            .invoke("MergeRow", (row, targetRow, preserveChanges, idxSearch))?;
        Ok(__cordl_ret)
    }
    pub fn Merge_DataTable0(
        &mut self,
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Merge", (table))?;
        Ok(__cordl_ret)
    }
    pub fn Merge__cordl_bool_MissingSchemaAction1(
        &mut self,
        table: *mut crate::System::Data::DataTable,
        preserveChanges: bool,
        missingSchemaAction: crate::System::Data::MissingSchemaAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Merge", (table, preserveChanges, missingSchemaAction))?;
        Ok(__cordl_ret)
    }
    pub fn MoveToElement(
        &mut self,
        reader: *mut crate::System::Xml::XmlReader,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveToElement", (reader, depth))?;
        Ok(__cordl_ret)
    }
    pub fn NewEmptyRow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataRow> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRow = __cordl_object
            .invoke("NewEmptyRow", ())?;
        Ok(__cordl_ret)
    }
    pub fn NewRecord_0(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("NewRecord", ())?;
        Ok(__cordl_ret)
    }
    pub fn NewRecord_i32_1(
        &mut self,
        sourceRecord: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("NewRecord", (sourceRecord))?;
        Ok(__cordl_ret)
    }
    pub fn NewRowArray(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataRow>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataRow,
        > = __cordl_object.invoke("NewRowArray", (_cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn NewRowCreated(
        &mut self,
        row: *mut crate::System::Data::DataRow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NewRowCreated", (row))?;
        Ok(__cordl_ret)
    }
    pub fn NewRowFromBuilder(
        &mut self,
        builder: *mut crate::System::Data::DataRowBuilder,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataRow> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRow = __cordl_object
            .invoke("NewRowFromBuilder", (builder))?;
        Ok(__cordl_ret)
    }
    pub fn NewRow_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataRow> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRow = __cordl_object
            .invoke("NewRow", ())?;
        Ok(__cordl_ret)
    }
    pub fn NewRow_i32_1(
        &mut self,
        record: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataRow> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRow = __cordl_object
            .invoke("NewRow", (record))?;
        Ok(__cordl_ret)
    }
    pub fn NewUninitializedRecord(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("NewUninitializedRecord", ())?;
        Ok(__cordl_ret)
    }
    pub fn NewUninitializedRow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataRow> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRow = __cordl_object
            .invoke("NewUninitializedRow", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_SerializationInfo_StreamingContext2(
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        tableName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tableName))?;
        Ok(__cordl_object)
    }
    pub fn OnColumnChanged(
        &mut self,
        e: *mut crate::System::Data::DataColumnChangeEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnChanged", (e))?;
        Ok(__cordl_ret)
    }
    pub fn OnColumnChanging(
        &mut self,
        e: *mut crate::System::Data::DataColumnChangeEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnChanging", (e))?;
        Ok(__cordl_ret)
    }
    pub fn OnPropertyChanging(
        &mut self,
        pcevent: *mut crate::System::ComponentModel::PropertyChangedEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPropertyChanging", (pcevent))?;
        Ok(__cordl_ret)
    }
    pub fn OnRemoveColumn(
        &mut self,
        column: *mut crate::System::Data::DataColumn,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRemoveColumn", (column))?;
        Ok(__cordl_ret)
    }
    pub fn OnRemoveColumnInternal(
        &mut self,
        column: *mut crate::System::Data::DataColumn,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRemoveColumnInternal", (column))?;
        Ok(__cordl_ret)
    }
    pub fn OnRowChanged_DataRowChangeEventArgs1(
        &mut self,
        e: *mut crate::System::Data::DataRowChangeEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRowChanged", (e))?;
        Ok(__cordl_ret)
    }
    pub fn OnRowChanged_DataRow_DataRowAction0(
        &mut self,
        args: *mut crate::System::Data::DataRowChangeEventArgs,
        eRow: *mut crate::System::Data::DataRow,
        eAction: crate::System::Data::DataRowAction,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Data::DataRowChangeEventArgs,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRowChangeEventArgs = __cordl_object
            .invoke("OnRowChanged", (args, eRow, eAction))?;
        Ok(__cordl_ret)
    }
    pub fn OnRowChanging_DataRowChangeEventArgs1(
        &mut self,
        e: *mut crate::System::Data::DataRowChangeEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRowChanging", (e))?;
        Ok(__cordl_ret)
    }
    pub fn OnRowChanging_DataRow_DataRowAction0(
        &mut self,
        args: *mut crate::System::Data::DataRowChangeEventArgs,
        eRow: *mut crate::System::Data::DataRow,
        eAction: crate::System::Data::DataRowAction,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Data::DataRowChangeEventArgs,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRowChangeEventArgs = __cordl_object
            .invoke("OnRowChanging", (args, eRow, eAction))?;
        Ok(__cordl_ret)
    }
    pub fn OnRowDeleted(
        &mut self,
        e: *mut crate::System::Data::DataRowChangeEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRowDeleted", (e))?;
        Ok(__cordl_ret)
    }
    pub fn OnRowDeleting(
        &mut self,
        e: *mut crate::System::Data::DataRowChangeEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRowDeleting", (e))?;
        Ok(__cordl_ret)
    }
    pub fn OnTableCleared(
        &mut self,
        e: *mut crate::System::Data::DataTableClearEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnTableCleared", (e))?;
        Ok(__cordl_ret)
    }
    pub fn OnTableClearing(
        &mut self,
        e: *mut crate::System::Data::DataTableClearEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnTableClearing", (e))?;
        Ok(__cordl_ret)
    }
    pub fn OnTableNewRow(
        &mut self,
        e: *mut crate::System::Data::DataTableNewRowEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnTableNewRow", (e))?;
        Ok(__cordl_ret)
    }
    pub fn ParseSortString(
        &mut self,
        sortString: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::System::Data::IndexField>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Data::IndexField,
        > = __cordl_object.invoke("ParseSortString", (sortString))?;
        Ok(__cordl_ret)
    }
    pub fn RaisePropertyChanging(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaisePropertyChanging", (name))?;
        Ok(__cordl_ret)
    }
    pub fn RaiseRowChanged(
        &mut self,
        args: *mut crate::System::Data::DataRowChangeEventArgs,
        eRow: *mut crate::System::Data::DataRow,
        eAction: crate::System::Data::DataRowAction,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Data::DataRowChangeEventArgs,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRowChangeEventArgs = __cordl_object
            .invoke("RaiseRowChanged", (args, eRow, eAction))?;
        Ok(__cordl_ret)
    }
    pub fn RaiseRowChanging_DataRowChangeEventArgs_DataRow_DataRowAction0(
        &mut self,
        args: *mut crate::System::Data::DataRowChangeEventArgs,
        eRow: *mut crate::System::Data::DataRow,
        eAction: crate::System::Data::DataRowAction,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Data::DataRowChangeEventArgs,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRowChangeEventArgs = __cordl_object
            .invoke("RaiseRowChanging", (args, eRow, eAction))?;
        Ok(__cordl_ret)
    }
    pub fn RaiseRowChanging__cordl_bool1(
        &mut self,
        args: *mut crate::System::Data::DataRowChangeEventArgs,
        eRow: *mut crate::System::Data::DataRow,
        eAction: crate::System::Data::DataRowAction,
        fireEvent: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Data::DataRowChangeEventArgs,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRowChangeEventArgs = __cordl_object
            .invoke("RaiseRowChanging", (args, eRow, eAction, fireEvent))?;
        Ok(__cordl_ret)
    }
    pub fn ReadEndElement(
        &mut self,
        reader: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadEndElement", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn ReadXDRSchema(
        &mut self,
        reader: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadXDRSchema", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn ReadXSDSchema(
        &mut self,
        reader: *mut crate::System::Xml::XmlReader,
        denyResolving: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadXSDSchema", (reader, denyResolving))?;
        Ok(__cordl_ret)
    }
    pub fn ReadXml(
        &mut self,
        reader: *mut crate::System::Xml::XmlReader,
        mode: crate::System::Data::XmlReadMode,
        denyResolving: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::XmlReadMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::XmlReadMode = __cordl_object
            .invoke("ReadXml", (reader, mode, denyResolving))?;
        Ok(__cordl_ret)
    }
    pub fn ReadXmlDiffgram(
        &mut self,
        reader: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadXmlDiffgram", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn ReadXmlSchema(
        &mut self,
        reader: *mut crate::System::Xml::XmlReader,
        denyResolving: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadXmlSchema", (reader, denyResolving))?;
        Ok(__cordl_ret)
    }
    pub fn ReadXmlSerializable(
        &mut self,
        reader: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadXmlSerializable", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn RecordChanged_Il2CppArray_Il2CppArray1(
        &mut self,
        oldIndex: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        newIndex: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecordChanged", (oldIndex, newIndex))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn RecordStateChanged_i32_DataViewRowState_DataViewRowState1(
        &mut self,
        record1: i32,
        oldState1: crate::System::Data::DataViewRowState,
        newState1: crate::System::Data::DataViewRowState,
        record2: i32,
        oldState2: crate::System::Data::DataViewRowState,
        newState2: crate::System::Data::DataViewRowState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RecordStateChanged",
                (record1, oldState1, newState1, record2, oldState2, newState2),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RemoveDependentColumn(
        &mut self,
        expressionColumn: *mut crate::System::Data::DataColumn,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveDependentColumn", (expressionColumn))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveRecordFromIndexes(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        version: crate::System::Data::DataRowVersion,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<i32> = __cordl_object
            .invoke("RemoveRecordFromIndexes", (row, version))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveRow(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        check: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveRow", (row, check))?;
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
    pub fn ResetConstraints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetConstraints", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResetIndexes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetIndexes", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResetInternalIndexes(
        &mut self,
        column: *mut crate::System::Data::DataColumn,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetInternalIndexes", (column))?;
        Ok(__cordl_ret)
    }
    pub fn RestoreConstraint(
        &mut self,
        originalEnforceConstraint: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RestoreConstraint", (originalEnforceConstraint))?;
        Ok(__cordl_ret)
    }
    pub fn RestoreIndexEvents(
        &mut self,
        forceReset: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RestoreIndexEvents", (forceReset))?;
        Ok(__cordl_ret)
    }
    pub fn RestoreShadowIndexes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RestoreShadowIndexes", ())?;
        Ok(__cordl_ret)
    }
    pub fn RollbackRow(
        &mut self,
        row: *mut crate::System::Data::DataRow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RollbackRow", (row))?;
        Ok(__cordl_ret)
    }
    pub fn SerializeConstraints(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
        serIndex: i32,
        allConstraints: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SerializeConstraints", (info, context, serIndex, allConstraints))?;
        Ok(__cordl_ret)
    }
    pub fn SerializeDataTable(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
        isSingleTable: bool,
        remotingFormat: crate::System::Data::SerializationFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SerializeDataTable",
                (info, context, isSingleTable, remotingFormat),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SerializeExpressionColumns(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
        serIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SerializeExpressionColumns", (info, context, serIndex))?;
        Ok(__cordl_ret)
    }
    pub fn SerializeTableData(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
        serIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SerializeTableData", (info, context, serIndex))?;
        Ok(__cordl_ret)
    }
    pub fn SerializeTableSchema(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
        isSingleTable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SerializeTableSchema", (info, context, isSingleTable))?;
        Ok(__cordl_ret)
    }
    pub fn SetCaseSensitiveValue(
        &mut self,
        isCaseSensitive: bool,
        userSet: bool,
        resetIndexes: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetCaseSensitiveValue", (isCaseSensitive, userSet, resetIndexes))?;
        Ok(__cordl_ret)
    }
    pub fn SetDataSet(
        &mut self,
        dataSet: *mut crate::System::Data::DataSet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDataSet", (dataSet))?;
        Ok(__cordl_ret)
    }
    pub fn SetKeyValues(
        &mut self,
        key: crate::System::Data::DataKey,
        keyValues: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
        record: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetKeyValues", (key, keyValues, record))?;
        Ok(__cordl_ret)
    }
    pub fn SetLocaleValue(
        &mut self,
        culture: *mut crate::System::Globalization::CultureInfo,
        userSet: bool,
        resetIndexes: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetLocaleValue", (culture, userSet, resetIndexes))?;
        Ok(__cordl_ret)
    }
    pub fn SetMergeRecords(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        newRecord: i32,
        oldRecord: i32,
        action: crate::System::Data::DataRowAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMergeRecords", (row, newRecord, oldRecord, action))?;
        Ok(__cordl_ret)
    }
    pub fn SetNewRecord(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        proposedRecord: i32,
        action: crate::System::Data::DataRowAction,
        isInMerge: bool,
        fireEvent: bool,
        suppressEnsurePropertyChanged: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetNewRecord",
                (
                    row,
                    proposedRecord,
                    action,
                    isInMerge,
                    fireEvent,
                    suppressEnsurePropertyChanged,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetNewRecordWorker(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        proposedRecord: i32,
        action: crate::System::Data::DataRowAction,
        isInMerge: bool,
        suppressEnsurePropertyChanged: bool,
        position: i32,
        fireEvent: bool,
        deferredException: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetNewRecordWorker",
                (
                    row,
                    proposedRecord,
                    action,
                    isInMerge,
                    suppressEnsurePropertyChanged,
                    position,
                    fireEvent,
                    deferredException,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetOldRecord(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        proposedRecord: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetOldRecord", (row, proposedRecord))?;
        Ok(__cordl_ret)
    }
    pub fn SetShadowIndexes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetShadowIndexes", ())?;
        Ok(__cordl_ret)
    }
    pub fn ShadowIndexCopy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShadowIndexCopy", ())?;
        Ok(__cordl_ret)
    }
    pub fn ShouldSerializeCaseSensitive(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldSerializeCaseSensitive", ())?;
        Ok(__cordl_ret)
    }
    pub fn ShouldSerializeLocale(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ShouldSerializeLocale", ())?;
        Ok(__cordl_ret)
    }
    pub fn SilentlySetValue(
        &mut self,
        dr: *mut crate::System::Data::DataRow,
        dc: *mut crate::System::Data::DataColumn,
        version: crate::System::Data::DataRowVersion,
        newValue: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SilentlySetValue", (dr, dc, version, newValue))?;
        Ok(__cordl_ret)
    }
    pub fn SuspendIndexEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SuspendIndexEvents", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_Serialization_IXmlSerializable_GetSchema(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::XmlSchema> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchema = __cordl_object
            .invoke("System.Xml.Serialization.IXmlSerializable.GetSchema", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_Serialization_IXmlSerializable_ReadXml(
        &mut self,
        reader: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Xml.Serialization.IXmlSerializable.ReadXml", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn System_Xml_Serialization_IXmlSerializable_WriteXml(
        &mut self,
        writer: *mut crate::System::Xml::XmlWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Xml.Serialization.IXmlSerializable.WriteXml", (writer))?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdatePropertyDescriptorCollectionCache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdatePropertyDescriptorCollectionCache", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdatingCurrent(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        action: crate::System::Data::DataRowAction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("UpdatingCurrent", (row, action))?;
        Ok(__cordl_ret)
    }
    pub fn WriteXml(
        &mut self,
        writer: *mut crate::System::Xml::XmlWriter,
        mode: crate::System::Data::XmlWriteMode,
        writeHierarchy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteXml", (writer, mode, writeHierarchy))?;
        Ok(__cordl_ret)
    }
    pub fn WriteXmlSchema(
        &mut self,
        writer: *mut crate::System::Xml::XmlWriter,
        writeHierarchy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteXmlSchema", (writer, writeHierarchy))?;
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
    pub fn _ctor_SerializationInfo_StreamingContext2(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, context))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String1(
        &mut self,
        tableName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tableName))?;
        Ok(__cordl_ret)
    }
    pub fn get_AreIndexEventsSuspended(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_AreIndexEventsSuspended", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CaseSensitive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CaseSensitive", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ChildRelations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Data::DataRelationCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRelationCollection = __cordl_object
            .invoke("get_ChildRelations", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Columns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataColumnCollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataColumnCollection = __cordl_object
            .invoke("get_Columns", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CompareInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Globalization::CompareInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Globalization::CompareInfo = __cordl_object
            .invoke("get_CompareInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Constraints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::ConstraintCollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::ConstraintCollection = __cordl_object
            .invoke("get_Constraints", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DataSet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataSet> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataSet = __cordl_object
            .invoke("get_DataSet", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DisplayExpressionInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_DisplayExpressionInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ElementColumnCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ElementColumnCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EncodedTableName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_EncodedTableName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EnforceConstraints(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_EnforceConstraints", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ExtendedProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::PropertyCollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::PropertyCollection = __cordl_object
            .invoke("get_ExtendedProperties", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_FormatProvider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IFormatProvider> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IFormatProvider = __cordl_object
            .invoke("get_FormatProvider", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsTypedDataTable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsTypedDataTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LiveIndexes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut crate::System::Data::Index>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Data::Index,
        > = __cordl_object.invoke("get_LiveIndexes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Locale(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Globalization::CultureInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Globalization::CultureInfo = __cordl_object
            .invoke("get_Locale", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MaxOccurs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Decimal = __cordl_object
            .invoke("get_MaxOccurs", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MergingData(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_MergingData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MinOccurs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Decimal = __cordl_object
            .invoke("get_MinOccurs", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MinimumCapacity(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_MinimumCapacity", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Namespace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Namespace", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NeedColumnChangeEvents(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_NeedColumnChangeEvents", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NestedParentRelations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataRelation>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataRelation,
        > = __cordl_object.invoke("get_NestedParentRelations", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NestedParentsCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_NestedParentsCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ObjectID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ObjectID", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ParentRelations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Data::DataRelationCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRelationCollection = __cordl_object
            .invoke("get_ParentRelations", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Prefix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Prefix", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PrimaryKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        > = __cordl_object.invoke("get_PrimaryKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RecordCapacity(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_RecordCapacity", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RemotingFormat(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SerializationFormat> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::SerializationFormat = __cordl_object
            .invoke("get_RemotingFormat", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RowDiffId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::Hashtable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Hashtable = __cordl_object
            .invoke("get_RowDiffId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Rows(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataRowCollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRowCollection = __cordl_object
            .invoke("get_Rows", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SchemaLoading(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_SchemaLoading", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SelfNested(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_SelfNested", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Site(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::ComponentModel::ISite> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::ISite = __cordl_object
            .invoke("get_Site", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SuspendEnforceConstraints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_SuspendEnforceConstraints", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TableName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_TableName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XmlQualifiedName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XmlQualifiedName = __cordl_object
            .invoke("get_TypeName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UKColumnPositionForInference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_UKColumnPositionForInference", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_XmlText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataColumn> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataColumn = __cordl_object
            .invoke("get_XmlText", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_CaseSensitive(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CaseSensitive", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ElementColumnCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ElementColumnCount", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_EnforceConstraints(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_EnforceConstraints", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Locale(
        &mut self,
        value: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Locale", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_MaxOccurs(
        &mut self,
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MaxOccurs", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_MergingData(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MergingData", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_MinOccurs(
        &mut self,
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MinOccurs", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_MinimumCapacity(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MinimumCapacity", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Namespace(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Namespace", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Prefix(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Prefix", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_PrimaryKey(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PrimaryKey", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_RemotingFormat(
        &mut self,
        value: crate::System::Data::SerializationFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RemotingFormat", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_SuspendEnforceConstraints(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SuspendEnforceConstraints", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_TableName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TableName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_TypeName(
        &mut self,
        value: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TypeName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_UKColumnPositionForInference(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UKColumnPositionForInference", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_XmlText(
        &mut self,
        value: *mut crate::System::Data::DataColumn,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_XmlText", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+DataTable")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::DataTable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Data+DataTable+DSRowDiffIdUsageSection")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DataTable_DSRowDiffIdUsageSection {
    pub _targetDS: *mut crate::System::Data::DataSet,
}
#[cfg(feature = "System+Data+DataTable+DSRowDiffIdUsageSection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::DataTable_DSRowDiffIdUsageSection
    => "System.Data"."DataTable/DSRowDiffIdUsageSection"
);
#[cfg(feature = "System+Data+DataTable+DSRowDiffIdUsageSection")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Data::DataTable_DSRowDiffIdUsageSection {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Data+DataTable+DSRowDiffIdUsageSection")]
impl crate::System::Data::DataTable_DSRowDiffIdUsageSection {
    pub fn Prepare(
        &mut self,
        ds: *mut crate::System::Data::DataSet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Prepare",
            (ds),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+DataTable+RowDiffIdUsageSection")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DataTable_RowDiffIdUsageSection {
    pub _targetTable: *mut crate::System::Data::DataTable,
}
#[cfg(feature = "System+Data+DataTable+RowDiffIdUsageSection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::DataTable_RowDiffIdUsageSection =>
    "System.Data"."DataTable/RowDiffIdUsageSection"
);
#[cfg(feature = "System+Data+DataTable+RowDiffIdUsageSection")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Data::DataTable_RowDiffIdUsageSection {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Data+DataTable+RowDiffIdUsageSection")]
impl crate::System::Data::DataTable_RowDiffIdUsageSection {
    pub fn Prepare(
        &mut self,
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Prepare",
            (table),
        )?;
        Ok(__cordl_ret)
    }
}
