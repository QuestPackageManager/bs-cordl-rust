#[cfg(feature = "System+Data+DataTable")]
#[repr(C)]
#[derive(Debug)]
pub struct DataTable {
    __cordl_parent: crate::System::ComponentModel::MarshalByValueComponent,
    pub _dataSet: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
    pub _defaultView: quest_hook::libil2cpp::Gc<crate::System::Data::DataView>,
    pub _nextRowID: i64,
    pub _rowCollection: quest_hook::libil2cpp::Gc<
        crate::System::Data::DataRowCollection,
    >,
    pub _columnCollection: quest_hook::libil2cpp::Gc<
        crate::System::Data::DataColumnCollection,
    >,
    pub _constraintCollection: quest_hook::libil2cpp::Gc<
        crate::System::Data::ConstraintCollection,
    >,
    pub _elementColumnCount: i32,
    pub _parentRelationsCollection: quest_hook::libil2cpp::Gc<
        crate::System::Data::DataRelationCollection,
    >,
    pub _childRelationsCollection: quest_hook::libil2cpp::Gc<
        crate::System::Data::DataRelationCollection,
    >,
    pub _recordManager: quest_hook::libil2cpp::Gc<crate::System::Data::RecordManager>,
    pub _indexes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::System::Data::Index>,
        >,
    >,
    pub _shadowIndexes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::System::Data::Index>,
        >,
    >,
    pub _shadowCount: i32,
    pub _extendedProperties: quest_hook::libil2cpp::Gc<
        crate::System::Data::PropertyCollection,
    >,
    pub _tableName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _tableNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _tablePrefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _displayExpression: quest_hook::libil2cpp::Gc<
        crate::System::Data::DataExpression,
    >,
    pub _fNestedInDataset: bool,
    pub _culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    pub _cultureUserSet: bool,
    pub _compareInfo: quest_hook::libil2cpp::Gc<
        crate::System::Globalization::CompareInfo,
    >,
    pub _compareFlags: crate::System::Globalization::CompareOptions,
    pub _formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    pub _hashCodeProvider: quest_hook::libil2cpp::Gc<crate::System::StringComparer>,
    pub _caseSensitive: bool,
    pub _caseSensitiveUserSet: bool,
    pub _encodedTableName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _xmlText: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    pub _colUnique: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    pub _minOccurs: crate::System::Decimal,
    pub _maxOccurs: crate::System::Decimal,
    pub _repeatableElement: bool,
    pub _typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _primaryKey: quest_hook::libil2cpp::Gc<crate::System::Data::UniqueConstraint>,
    pub _primaryIndex: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::System::Data::IndexField>,
    >,
    pub _delayedSetPrimaryKey: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        >,
    >,
    pub _loadIndex: quest_hook::libil2cpp::Gc<crate::System::Data::Index>,
    pub _loadIndexwithOriginalAdded: quest_hook::libil2cpp::Gc<
        crate::System::Data::Index,
    >,
    pub _loadIndexwithCurrentDeleted: quest_hook::libil2cpp::Gc<
        crate::System::Data::Index,
    >,
    pub _suspendIndexEvents: i32,
    pub _inDataLoad: bool,
    pub _schemaLoading: bool,
    pub _enforceConstraints: bool,
    pub _suspendEnforceConstraints: bool,
    pub fInitInProgress: bool,
    pub _inLoad: bool,
    pub _fInLoadDiffgram: bool,
    pub _isTypedDataTable: u8,
    pub _emptyDataRowArray: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        >,
    >,
    pub _propertyDescriptorCollectionCache: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::PropertyDescriptorCollection,
    >,
    pub _nestedParentRelations: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Data::DataRelation>,
        >,
    >,
    pub _dependentColumns: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        >,
    >,
    pub _mergingData: bool,
    pub _onRowChangedDelegate: quest_hook::libil2cpp::Gc<
        crate::System::Data::DataRowChangeEventHandler,
    >,
    pub _onRowChangingDelegate: quest_hook::libil2cpp::Gc<
        crate::System::Data::DataRowChangeEventHandler,
    >,
    pub _onRowDeletingDelegate: quest_hook::libil2cpp::Gc<
        crate::System::Data::DataRowChangeEventHandler,
    >,
    pub _onRowDeletedDelegate: quest_hook::libil2cpp::Gc<
        crate::System::Data::DataRowChangeEventHandler,
    >,
    pub _onColumnChangedDelegate: quest_hook::libil2cpp::Gc<
        crate::System::Data::DataColumnChangeEventHandler,
    >,
    pub _onColumnChangingDelegate: quest_hook::libil2cpp::Gc<
        crate::System::Data::DataColumnChangeEventHandler,
    >,
    pub _onTableClearingDelegate: quest_hook::libil2cpp::Gc<
        crate::System::Data::DataTableClearEventHandler,
    >,
    pub _onTableClearedDelegate: quest_hook::libil2cpp::Gc<
        crate::System::Data::DataTableClearEventHandler,
    >,
    pub _onTableNewRowDelegate: quest_hook::libil2cpp::Gc<
        crate::System::Data::DataTableNewRowEventHandler,
    >,
    pub _onPropertyChangingDelegate: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::PropertyChangedEventHandler,
    >,
    pub _rowBuilder: quest_hook::libil2cpp::Gc<crate::System::Data::DataRowBuilder>,
    pub _delayedViews: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::System::Data::DataView>,
        >,
    >,
    pub _dataViewListeners: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::System::Data::DataViewListener>,
        >,
    >,
    pub _rowDiffId: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    pub _indexesLock: quest_hook::libil2cpp::Gc<
        crate::System::Threading::ReaderWriterLockSlim,
    >,
    pub _ukColumnPositionForInference: i32,
    pub _remotingFormat: crate::System::Data::SerializationFormat,
    pub _objectID: i32,
}
#[cfg(feature = "System+Data+DataTable")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Data::DataTable {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "DataTable";
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
        expressionColumn: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AddDependentColumn")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "AddDependentColumn", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expressionColumn))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddForeignKey(
        &mut self,
        parentKey: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
                1usize,
            >("AddForeignKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "AddForeignKey", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn> = unsafe {
            method.invoke_unchecked(self, (parentKey))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddRow(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        proposedID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddRow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "AddRow", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (row, proposedID))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddUniqueKey_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
                0usize,
            >("AddUniqueKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "AddUniqueKey", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddUniqueKey_i32_0(
        &mut self,
        position: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
                1usize,
            >("AddUniqueKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "AddUniqueKey", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn> = unsafe {
            method.invoke_unchecked(self, (position))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CacheNestedParent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("CacheNestedParent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "CacheNestedParent", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CascadeAll(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        action: crate::System::Data::DataRowAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    crate::System::Data::DataRowAction,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CascadeAll")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "CascadeAll", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (row, action))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckCascadingNamespaceConflict(
        &mut self,
        realNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CheckCascadingNamespaceConflict")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "CheckCascadingNamespaceConflict", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (realNamespace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckForClosureOnExpressionTables(
        &mut self,
        tableList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
                    >,
                >),
                bool,
                1usize,
            >("CheckForClosureOnExpressionTables")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "CheckForClosureOnExpressionTables", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (tableList))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckForClosureOnExpressions(
        &mut self,
        dt: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        writeHierarchy: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>, bool),
                bool,
                2usize,
            >("CheckForClosureOnExpressions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "CheckForClosureOnExpressions", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (dt, writeHierarchy))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckNamespaceValidityForNestedParentRelations(
        &mut self,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentTable: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CheckNamespaceValidityForNestedParentRelations")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "CheckNamespaceValidityForNestedParentRelations", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ns, parentTable))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckNamespaceValidityForNestedRelations(
        &mut self,
        realNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CheckNamespaceValidityForNestedRelations")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "CheckNamespaceValidityForNestedRelations", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (realNamespace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckNotModifying(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CheckNotModifying")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "CheckNotModifying", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (row))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Clear_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Clear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "Clear", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Clear__cordl_bool1(
        &mut self,
        clearAll: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("Clear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "Clear", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (clearAll))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CloneHierarchy(
        &mut self,
        sourceTable: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        ds: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
        visitedMap: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
                    quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
                3usize,
            >("CloneHierarchy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "CloneHierarchy", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable> = unsafe {
            method.invoke_unchecked(self, (sourceTable, ds, visitedMap))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CloneTo(
        &mut self,
        clone: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        cloneDS: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
        skipExpressionColumns: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
                3usize,
            >("CloneTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "CloneTo", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable> = unsafe {
            method.invoke_unchecked(self, (clone, cloneDS, skipExpressionColumns))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Clone_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
                0usize,
            >("Clone")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "Clone", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Clone_DataSet1(
        &mut self,
        cloneDS: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
                1usize,
            >("Clone")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "Clone", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable> = unsafe {
            method.invoke_unchecked(self, (cloneDS))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CommitRow(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CommitRow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "CommitRow", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (row))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Compare_CompareInfo1(
        &mut self,
        s1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        s2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        comparer: quest_hook::libil2cpp::Gc<crate::System::Globalization::CompareInfo>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::Globalization::CompareInfo>,
                ),
                i32,
                3usize,
            >("Compare")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "Compare", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (s1, s2, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Compare_Il2CppString_Il2CppString0(
        &mut self,
        s1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        s2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                i32,
                2usize,
            >("Compare")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "Compare", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (s1, s2))? };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToRowError(
        &mut self,
        rowIndex: i32,
        rowErrors: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
        colErrors: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
                    quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ConvertToRowError")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "ConvertToRowError", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (rowIndex, rowErrors, colErrors))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertToRowState(
        &mut self,
        bitStates: quest_hook::libil2cpp::Gc<crate::System::Collections::BitArray>,
        bitIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::DataRowState> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Collections::BitArray>, i32),
                crate::System::Data::DataRowState,
                2usize,
            >("ConvertToRowState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "ConvertToRowState", 2usize
                )
            });
        let __cordl_ret: crate::System::Data::DataRowState = unsafe {
            method.invoke_unchecked(self, (bitStates, bitIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateEmptyRow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                0usize,
            >("CreateEmptyRow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "CreateEmptyRow", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
                0usize,
            >("CreateInstance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "CreateInstance", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateRelationList(
        &mut self,
        tableList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
            >,
        >,
        relationList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRelation>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<crate::System::Data::DataRelation>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CreateRelationList")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "CreateRelationList", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (tableList, relationList))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateTableList(
        &mut self,
        currentTable: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        tableList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CreateTableList")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "CreateTableList", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (currentTable, tableList))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DeleteRow(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DeleteRow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "DeleteRow", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (row))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeConstraints(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
        serIndex: i32,
        allConstraints: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationInfo,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
                    i32,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("DeserializeConstraints")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "DeserializeConstraints", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (info, context, serIndex, allConstraints))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeDataTable(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
        isSingleTable: bool,
        remotingFormat: crate::System::Data::SerializationFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationInfo,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
                    bool,
                    crate::System::Data::SerializationFormat,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("DeserializeDataTable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "DeserializeDataTable", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(self, (info, context, isSingleTable, remotingFormat))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeExpressionColumns(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
        serIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationInfo,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("DeserializeExpressionColumns")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "DeserializeExpressionColumns", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (info, context, serIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeTableData(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
        serIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationInfo,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("DeserializeTableData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "DeserializeTableData", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (info, context, serIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeTableSchema(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
        isSingleTable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationInfo,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("DeserializeTableSchema")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "DeserializeTableSchema", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (info, context, isSingleTable))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoRaiseNamespaceChange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("DoRaiseNamespaceChange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "DoRaiseNamespaceChange", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnableConstraints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("EnableConstraints")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "EnableConstraints", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EvaluateDependentExpressions_DataColumn0(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("EvaluateDependentExpressions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "EvaluateDependentExpressions", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (column))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EvaluateDependentExpressions_List_1_DataRow_DataRowVersion_List_1_1(
        &mut self,
        columns: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
            >,
        >,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        version: crate::System::Data::DataRowVersion,
        cachedRows: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    crate::System::Data::DataRowVersion,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("EvaluateDependentExpressions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "EvaluateDependentExpressions", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (columns, row, version, cachedRows))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EvaluateExpressions_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("EvaluateExpressions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "EvaluateExpressions", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EvaluateExpressions_DataColumn2(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("EvaluateExpressions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "EvaluateExpressions", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (column))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EvaluateExpressions_DataRow_DataRowAction_List_1_1(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        action: crate::System::Data::DataRowAction,
        cachedRows: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    crate::System::Data::DataRowAction,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("EvaluateExpressions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "EvaluateExpressions", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (row, action, cachedRows))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindByIndex(
        &mut self,
        ndx: quest_hook::libil2cpp::Gc<crate::System::Data::Index>,
        key: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::Index>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                2usize,
            >("FindByIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "FindByIndex", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow> = unsafe {
            method.invoke_unchecked(self, (ndx, key))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindMergeTarget(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        key: crate::System::Data::DataKey,
        ndx: quest_hook::libil2cpp::Gc<crate::System::Data::Index>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    crate::System::Data::DataKey,
                    quest_hook::libil2cpp::Gc<crate::System::Data::Index>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                3usize,
            >("FindMergeTarget")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "FindMergeTarget", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow> = unsafe {
            method.invoke_unchecked(self, (row, key, ndx))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindNestedParentRelations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRelation>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::System::Data::DataRelation>,
                    >,
                >,
                0usize,
            >("FindNestedParentRelations")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "FindNestedParentRelations", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRelation>,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn FormatSortString(
        &mut self,
        indexDesc: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::Data::IndexField>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::System::Data::IndexField>,
                >),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("FormatSortString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "FormatSortString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (indexDesc))? };
        Ok(__cordl_ret.into())
    }
    pub fn FreeRecord(
        &mut self,
        record: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<i32>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("FreeRecord")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "FreeRecord", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (record))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDataTableSchema(
        schemaSet: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSet>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaComplexType>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSet>),
                quest_hook::libil2cpp::Gc<
                    crate::System::Xml::Schema::XmlSchemaComplexType,
                >,
                1usize,
            >("GetDataTableSchema")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "GetDataTableSchema", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaComplexType,
        > = unsafe { method.invoke_unchecked((), (schemaSet))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetIndex_Il2CppArray1(
        &mut self,
        indexDesc: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::Data::IndexField>,
        >,
        recordStates: crate::System::Data::DataViewRowState,
        rowFilter: quest_hook::libil2cpp::Gc<crate::System::Data::IFilter>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::Index>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::System::Data::IndexField,
                        >,
                    >,
                    crate::System::Data::DataViewRowState,
                    quest_hook::libil2cpp::Gc<crate::System::Data::IFilter>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Data::Index>,
                3usize,
            >("GetIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "GetIndex", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::Index> = unsafe {
            method.invoke_unchecked(self, (indexDesc, recordStates, rowFilter))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetIndex_Il2CppString0(
        &mut self,
        sort: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        recordStates: crate::System::Data::DataViewRowState,
        rowFilter: quest_hook::libil2cpp::Gc<crate::System::Data::IFilter>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::Index>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::Data::DataViewRowState,
                    quest_hook::libil2cpp::Gc<crate::System::Data::IFilter>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Data::Index>,
                3usize,
            >("GetIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "GetIndex", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::Index> = unsafe {
            method.invoke_unchecked(self, (sort, recordStates, rowFilter))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetInheritedNamespace(
        &mut self,
        visitedTables: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
                    >,
                >),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetInheritedNamespace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "GetInheritedNamespace", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (visitedTables))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetListeners(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataViewListener>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<crate::System::Data::DataViewListener>,
                    >,
                >,
                0usize,
            >("GetListeners")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "GetListeners", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataViewListener>,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectData(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationInfo,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GetObjectData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "GetObjectData", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (info, context))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyDescriptorCollection(
        &mut self,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::System::Attribute>,
                    >,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::PropertyDescriptorCollection,
                >,
                1usize,
            >("GetPropertyDescriptorCollection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "GetPropertyDescriptorCollection", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptorCollection,
        > = unsafe { method.invoke_unchecked(self, (attributes))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRowAndColumnErrors(
        &mut self,
        rowIndex: i32,
        rowErrors: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
        colErrors: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
                    quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("GetRowAndColumnErrors")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "GetRowAndColumnErrors", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (rowIndex, rowErrors, colErrors))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRowType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                0usize,
            >("GetRowType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "GetRowType", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSchema(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
                0usize,
            >("GetSchema")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "GetSchema", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchema,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSpecialHashCode(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                i32,
                1usize,
            >("GetSpecialHashCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "GetSpecialHashCode", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn IncrementalCloneTo(
        &mut self,
        sourceTable: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        targetTable: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
                2usize,
            >("IncrementalCloneTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "IncrementalCloneTo", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable> = unsafe {
            method.invoke_unchecked(self, (sourceTable, targetTable))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf(
        &mut self,
        s1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        s2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                i32,
                2usize,
            >("IndexOf")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "IndexOf", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (s1, s2))? };
        Ok(__cordl_ret.into())
    }
    pub fn InsertRecordToIndexes(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        version: crate::System::Data::DataRowVersion,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    crate::System::Data::DataRowVersion,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                2usize,
            >("InsertRecordToIndexes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "InsertRecordToIndexes", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = unsafe { method.invoke_unchecked(self, (row, version))? };
        Ok(__cordl_ret.into())
    }
    pub fn InsertRow_i32_i32_0(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        proposedID: i32,
        pos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>, i32, i32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("InsertRow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "InsertRow", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (row, proposedID, pos))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InsertRow_i64_2(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        proposedID: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>, i64),
                quest_hook::libil2cpp::Void,
                2usize,
            >("InsertRow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "InsertRow", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (row, proposedID))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InsertRow_i64_i32__cordl_bool1(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        proposedID: i64,
        pos: i32,
        fireEvent: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    i64,
                    i32,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("InsertRow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "InsertRow", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (row, proposedID, pos, fireEvent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsEmptyXml(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>),
                bool,
                1usize,
            >("IsEmptyXml")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "IsEmptyXml", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (reader))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsNamespaceInherited(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("IsNamespaceInherited")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "IsNamespaceInherited", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsSuffix(
        &mut self,
        s1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        s2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                bool,
                2usize,
            >("IsSuffix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "IsSuffix", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (s1, s2))? };
        Ok(__cordl_ret.into())
    }
    pub fn MergeRow(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        targetRow: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        preserveChanges: bool,
        idxSearch: quest_hook::libil2cpp::Gc<crate::System::Data::Index>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    bool,
                    quest_hook::libil2cpp::Gc<crate::System::Data::Index>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                4usize,
            >("MergeRow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "MergeRow", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow> = unsafe {
            method.invoke_unchecked(self, (row, targetRow, preserveChanges, idxSearch))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Merge_DataTable0(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Merge")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "Merge", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (table))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Merge__cordl_bool_MissingSchemaAction1(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        preserveChanges: bool,
        missingSchemaAction: crate::System::Data::MissingSchemaAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
                    bool,
                    crate::System::Data::MissingSchemaAction,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("Merge")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "Merge", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (table, preserveChanges, missingSchemaAction))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MoveToElement(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>, i32),
                bool,
                2usize,
            >("MoveToElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "MoveToElement", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (reader, depth))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewEmptyRow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                0usize,
            >("NewEmptyRow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "NewEmptyRow", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewRecord_0(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("NewRecord")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "NewRecord", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn NewRecord_i32_1(
        &mut self,
        sourceRecord: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), i32, 1usize>("NewRecord")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "NewRecord", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (sourceRecord))? };
        Ok(__cordl_ret.into())
    }
    pub fn NewRowArray(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    >,
                >,
                1usize,
            >("NewRowArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "NewRowArray", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
            >,
        > = unsafe { method.invoke_unchecked(self, (_cordl_size))? };
        Ok(__cordl_ret.into())
    }
    pub fn NewRowCreated(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("NewRowCreated")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "NewRowCreated", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (row))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewRowFromBuilder(
        &mut self,
        builder: quest_hook::libil2cpp::Gc<crate::System::Data::DataRowBuilder>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataRowBuilder>),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                1usize,
            >("NewRowFromBuilder")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "NewRowFromBuilder", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow> = unsafe {
            method.invoke_unchecked(self, (builder))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewRow_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                0usize,
            >("NewRow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "NewRow", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewRow_i32_1(
        &mut self,
        record: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                1usize,
            >("NewRow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "NewRow", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow> = unsafe {
            method.invoke_unchecked(self, (record))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NewUninitializedRecord(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("NewUninitializedRecord")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "NewUninitializedRecord", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn NewUninitializedRow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                0usize,
            >("NewUninitializedRow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "NewUninitializedRow", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString1(
        tableName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tableName))?;
        Ok(__cordl_object.into())
    }
    pub fn New_SerializationInfo_StreamingContext2(
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object.into())
    }
    pub fn OnColumnChanged(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumnChangeEventArgs>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Data::DataColumnChangeEventArgs,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnColumnChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "OnColumnChanged", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (e))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnColumnChanging(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumnChangeEventArgs>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Data::DataColumnChangeEventArgs,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnColumnChanging")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "OnColumnChanging", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (e))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnPropertyChanging(
        &mut self,
        pcevent: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyChangedEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::ComponentModel::PropertyChangedEventArgs,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnPropertyChanging")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "OnPropertyChanging", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pcevent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnRemoveColumn(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnRemoveColumn")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "OnRemoveColumn", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (column))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnRemoveColumnInternal(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnRemoveColumnInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "OnRemoveColumnInternal", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (column))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnRowChanged_DataRowChangeEventArgs1(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Data::DataRowChangeEventArgs>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataRowChangeEventArgs>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnRowChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "OnRowChanged", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (e))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnRowChanged_DataRow_DataRowAction0(
        &mut self,
        args: quest_hook::libil2cpp::Gc<crate::System::Data::DataRowChangeEventArgs>,
        eRow: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        eAction: crate::System::Data::DataRowAction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRowChangeEventArgs>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Data::DataRowChangeEventArgs,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    crate::System::Data::DataRowAction,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRowChangeEventArgs>,
                3usize,
            >("OnRowChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "OnRowChanged", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::DataRowChangeEventArgs,
        > = unsafe { method.invoke_unchecked(self, (args, eRow, eAction))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnRowChanging_DataRowChangeEventArgs1(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Data::DataRowChangeEventArgs>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataRowChangeEventArgs>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnRowChanging")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "OnRowChanging", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (e))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnRowChanging_DataRow_DataRowAction0(
        &mut self,
        args: quest_hook::libil2cpp::Gc<crate::System::Data::DataRowChangeEventArgs>,
        eRow: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        eAction: crate::System::Data::DataRowAction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRowChangeEventArgs>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Data::DataRowChangeEventArgs,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    crate::System::Data::DataRowAction,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRowChangeEventArgs>,
                3usize,
            >("OnRowChanging")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "OnRowChanging", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::DataRowChangeEventArgs,
        > = unsafe { method.invoke_unchecked(self, (args, eRow, eAction))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnRowDeleted(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Data::DataRowChangeEventArgs>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataRowChangeEventArgs>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnRowDeleted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "OnRowDeleted", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (e))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnRowDeleting(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Data::DataRowChangeEventArgs>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataRowChangeEventArgs>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnRowDeleting")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "OnRowDeleting", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (e))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnTableCleared(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Data::DataTableClearEventArgs>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Data::DataTableClearEventArgs,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnTableCleared")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "OnTableCleared", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (e))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnTableClearing(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Data::DataTableClearEventArgs>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Data::DataTableClearEventArgs,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnTableClearing")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "OnTableClearing", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (e))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnTableNewRow(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Data::DataTableNewRowEventArgs>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Data::DataTableNewRowEventArgs,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnTableNewRow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "OnTableNewRow", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (e))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseSortString(
        &mut self,
        sortString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::Data::IndexField>,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::System::Data::IndexField>,
                >,
                1usize,
            >("ParseSortString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "ParseSortString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::Data::IndexField>,
        > = unsafe { method.invoke_unchecked(self, (sortString))? };
        Ok(__cordl_ret.into())
    }
    pub fn RaisePropertyChanging(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RaisePropertyChanging")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "RaisePropertyChanging", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (name))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RaiseRowChanged(
        &mut self,
        args: quest_hook::libil2cpp::Gc<crate::System::Data::DataRowChangeEventArgs>,
        eRow: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        eAction: crate::System::Data::DataRowAction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRowChangeEventArgs>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Data::DataRowChangeEventArgs,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    crate::System::Data::DataRowAction,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRowChangeEventArgs>,
                3usize,
            >("RaiseRowChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "RaiseRowChanged", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::DataRowChangeEventArgs,
        > = unsafe { method.invoke_unchecked(self, (args, eRow, eAction))? };
        Ok(__cordl_ret.into())
    }
    pub fn RaiseRowChanging_DataRowChangeEventArgs_DataRow_DataRowAction0(
        &mut self,
        args: quest_hook::libil2cpp::Gc<crate::System::Data::DataRowChangeEventArgs>,
        eRow: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        eAction: crate::System::Data::DataRowAction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRowChangeEventArgs>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Data::DataRowChangeEventArgs,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    crate::System::Data::DataRowAction,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRowChangeEventArgs>,
                3usize,
            >("RaiseRowChanging")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "RaiseRowChanging", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::DataRowChangeEventArgs,
        > = unsafe { method.invoke_unchecked(self, (args, eRow, eAction))? };
        Ok(__cordl_ret.into())
    }
    pub fn RaiseRowChanging__cordl_bool1(
        &mut self,
        args: quest_hook::libil2cpp::Gc<crate::System::Data::DataRowChangeEventArgs>,
        eRow: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        eAction: crate::System::Data::DataRowAction,
        fireEvent: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRowChangeEventArgs>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Data::DataRowChangeEventArgs,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    crate::System::Data::DataRowAction,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRowChangeEventArgs>,
                4usize,
            >("RaiseRowChanging")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "RaiseRowChanging", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::DataRowChangeEventArgs,
        > = unsafe { method.invoke_unchecked(self, (args, eRow, eAction, fireEvent))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadEndElement(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ReadEndElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "ReadEndElement", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (reader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadXDRSchema(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ReadXDRSchema")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "ReadXDRSchema", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (reader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadXSDSchema(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
        denyResolving: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReadXSDSchema")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "ReadXSDSchema", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (reader, denyResolving))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadXml(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
        mode: crate::System::Data::XmlReadMode,
        denyResolving: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::XmlReadMode> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
                    crate::System::Data::XmlReadMode,
                    bool,
                ),
                crate::System::Data::XmlReadMode,
                3usize,
            >("ReadXml")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "ReadXml", 3usize
                )
            });
        let __cordl_ret: crate::System::Data::XmlReadMode = unsafe {
            method.invoke_unchecked(self, (reader, mode, denyResolving))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadXmlDiffgram(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ReadXmlDiffgram")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "ReadXmlDiffgram", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (reader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadXmlSchema(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
        denyResolving: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReadXmlSchema")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "ReadXmlSchema", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (reader, denyResolving))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadXmlSerializable(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ReadXmlSerializable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "ReadXmlSerializable", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (reader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RecordChanged_Il2CppArray_Il2CppArray1(
        &mut self,
        oldIndex: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        newIndex: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("RecordChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "RecordChanged", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (oldIndex, newIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RecordChanged_i32_0(
        &mut self,
        record: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("RecordChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "RecordChanged", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (record))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RecordStateChanged_i32_DataViewRowState_DataViewRowState0(
        &mut self,
        record: i32,
        oldState: crate::System::Data::DataViewRowState,
        newState: crate::System::Data::DataViewRowState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    crate::System::Data::DataViewRowState,
                    crate::System::Data::DataViewRowState,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("RecordStateChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "RecordStateChanged", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (record, oldState, newState))?
        };
        Ok(__cordl_ret.into())
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    crate::System::Data::DataViewRowState,
                    crate::System::Data::DataViewRowState,
                    i32,
                    crate::System::Data::DataViewRowState,
                    crate::System::Data::DataViewRowState,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("RecordStateChanged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "RecordStateChanged", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (record1, oldState1, newState1, record2, oldState2, newState2),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveDependentColumn(
        &mut self,
        expressionColumn: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RemoveDependentColumn")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "RemoveDependentColumn", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (expressionColumn))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveRecordFromIndexes(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        version: crate::System::Data::DataRowVersion,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    crate::System::Data::DataRowVersion,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                2usize,
            >("RemoveRecordFromIndexes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "RemoveRecordFromIndexes", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = unsafe { method.invoke_unchecked(self, (row, version))? };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveRow(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        check: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("RemoveRow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "RemoveRow", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (row, check))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "Reset", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetConstraints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ResetConstraints")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "ResetConstraints", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetIndexes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ResetIndexes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "ResetIndexes", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetInternalIndexes(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ResetInternalIndexes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "ResetInternalIndexes", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (column))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RestoreConstraint(
        &mut self,
        originalEnforceConstraint: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RestoreConstraint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "RestoreConstraint", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (originalEnforceConstraint))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RestoreIndexEvents(
        &mut self,
        forceReset: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RestoreIndexEvents")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "RestoreIndexEvents", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (forceReset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RestoreShadowIndexes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("RestoreShadowIndexes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "RestoreShadowIndexes", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RollbackRow(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RollbackRow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "RollbackRow", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (row))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeConstraints(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
        serIndex: i32,
        allConstraints: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationInfo,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
                    i32,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SerializeConstraints")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "SerializeConstraints", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (info, context, serIndex, allConstraints))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeDataTable(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
        isSingleTable: bool,
        remotingFormat: crate::System::Data::SerializationFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationInfo,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
                    bool,
                    crate::System::Data::SerializationFormat,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SerializeDataTable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "SerializeDataTable", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(self, (info, context, isSingleTable, remotingFormat))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeExpressionColumns(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
        serIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationInfo,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SerializeExpressionColumns")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "SerializeExpressionColumns", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (info, context, serIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeTableData(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
        serIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationInfo,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SerializeTableData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "SerializeTableData", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (info, context, serIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeTableSchema(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
        isSingleTable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationInfo,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SerializeTableSchema")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "SerializeTableSchema", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (info, context, isSingleTable))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetCaseSensitiveValue(
        &mut self,
        isCaseSensitive: bool,
        userSet: bool,
        resetIndexes: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool, bool, bool), bool, 3usize>("SetCaseSensitiveValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "SetCaseSensitiveValue", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (isCaseSensitive, userSet, resetIndexes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetDataSet(
        &mut self,
        dataSet: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetDataSet")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "SetDataSet", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (dataSet))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyValues(
        &mut self,
        key: crate::System::Data::DataKey,
        keyValues: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        record: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::System::Data::DataKey,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        >,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetKeyValues")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "SetKeyValues", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (key, keyValues, record))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetLocaleValue(
        &mut self,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        userSet: bool,
        resetIndexes: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
                    bool,
                    bool,
                ),
                bool,
                3usize,
            >("SetLocaleValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "SetLocaleValue", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (culture, userSet, resetIndexes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetMergeRecords(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        newRecord: i32,
        oldRecord: i32,
        action: crate::System::Data::DataRowAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    i32,
                    i32,
                    crate::System::Data::DataRowAction,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SetMergeRecords")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "SetMergeRecords", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (row, newRecord, oldRecord, action))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetNewRecord(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        proposedRecord: i32,
        action: crate::System::Data::DataRowAction,
        isInMerge: bool,
        fireEvent: bool,
        suppressEnsurePropertyChanged: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    i32,
                    crate::System::Data::DataRowAction,
                    bool,
                    bool,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("SetNewRecord")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "SetNewRecord", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        row,
                        proposedRecord,
                        action,
                        isInMerge,
                        fireEvent,
                        suppressEnsurePropertyChanged,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetNewRecordWorker(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        proposedRecord: i32,
        action: crate::System::Data::DataRowAction,
        isInMerge: bool,
        suppressEnsurePropertyChanged: bool,
        position: i32,
        fireEvent: bool,
        deferredException: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Exception>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    i32,
                    crate::System::Data::DataRowAction,
                    bool,
                    bool,
                    i32,
                    bool,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<crate::System::Exception>,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                8usize,
            >("SetNewRecordWorker")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "SetNewRecordWorker", 8usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
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
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOldRecord(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        proposedRecord: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetOldRecord")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "SetOldRecord", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (row, proposedRecord))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetShadowIndexes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("SetShadowIndexes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "SetShadowIndexes", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShadowIndexCopy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ShadowIndexCopy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "ShadowIndexCopy", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldSerializeCaseSensitive(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("ShouldSerializeCaseSensitive")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "ShouldSerializeCaseSensitive", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldSerializeLocale(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("ShouldSerializeLocale")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "ShouldSerializeLocale", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SilentlySetValue(
        &mut self,
        dr: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        dc: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        version: crate::System::Data::DataRowVersion,
        newValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
                    crate::System::Data::DataRowVersion,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SilentlySetValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "SilentlySetValue", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (dr, dc, version, newValue))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SuspendIndexEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("SuspendIndexEvents")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "SuspendIndexEvents", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_Serialization_IXmlSerializable_GetSchema(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
                0usize,
            >("System.Xml.Serialization.IXmlSerializable.GetSchema")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "System.Xml.Serialization.IXmlSerializable.GetSchema",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchema,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_Serialization_IXmlSerializable_ReadXml(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("System.Xml.Serialization.IXmlSerializable.ReadXml")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "System.Xml.Serialization.IXmlSerializable.ReadXml",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (reader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_Xml_Serialization_IXmlSerializable_WriteXml(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("System.Xml.Serialization.IXmlSerializable.WriteXml")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "System.Xml.Serialization.IXmlSerializable.WriteXml",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (writer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "ToString", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdatePropertyDescriptorCollectionCache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("UpdatePropertyDescriptorCollectionCache")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "UpdatePropertyDescriptorCollectionCache", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdatingCurrent(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        action: crate::System::Data::DataRowAction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
                    crate::System::Data::DataRowAction,
                ),
                bool,
                2usize,
            >("UpdatingCurrent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "UpdatingCurrent", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (row, action))? };
        Ok(__cordl_ret.into())
    }
    pub fn WriteXml(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
        mode: crate::System::Data::XmlWriteMode,
        writeHierarchy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
                    crate::System::Data::XmlWriteMode,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("WriteXml")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "WriteXml", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (writer, mode, writeHierarchy))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteXmlSchema(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
        writeHierarchy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteXmlSchema")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "WriteXmlSchema", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (writer, writeHierarchy))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString1(
        &mut self,
        tableName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (tableName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SerializationInfo_StreamingContext2(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationInfo,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (info, context))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_AreIndexEventsSuspended(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_AreIndexEventsSuspended")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_AreIndexEventsSuspended", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_CaseSensitive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_CaseSensitive")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_CaseSensitive", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ChildRelations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRelationCollection>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRelationCollection>,
                0usize,
            >("get_ChildRelations")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_ChildRelations", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::DataRelationCollection,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Columns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataColumnCollection>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataColumnCollection>,
                0usize,
            >("get_Columns")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_Columns", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::DataColumnCollection,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_CompareInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CompareInfo>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Globalization::CompareInfo>,
                0usize,
            >("get_CompareInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_CompareInfo", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CompareInfo,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Constraints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::ConstraintCollection>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Data::ConstraintCollection>,
                0usize,
            >("get_Constraints")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_Constraints", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::ConstraintCollection,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_DataSet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
                0usize,
            >("get_DataSet")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_DataSet", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_DisplayExpressionInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_DisplayExpressionInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_DisplayExpressionInternal", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ElementColumnCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_ElementColumnCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_ElementColumnCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_EncodedTableName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_EncodedTableName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_EncodedTableName", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_EnforceConstraints(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_EnforceConstraints")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_EnforceConstraints", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ExtendedProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::PropertyCollection>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Data::PropertyCollection>,
                0usize,
            >("get_ExtendedProperties")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_ExtendedProperties", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::PropertyCollection,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_FormatProvider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                0usize,
            >("get_FormatProvider")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_FormatProvider", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsTypedDataTable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsTypedDataTable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_IsTypedDataTable", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_LiveIndexes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::System::Data::Index>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<crate::System::Data::Index>,
                    >,
                >,
                0usize,
            >("get_LiveIndexes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_LiveIndexes", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::System::Data::Index>,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Locale(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
                0usize,
            >("get_Locale")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_Locale", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxOccurs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::System::Decimal, 0usize>("get_MaxOccurs")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_MaxOccurs", 0usize
                )
            });
        let __cordl_ret: crate::System::Decimal = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_MergingData(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_MergingData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_MergingData", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_MinOccurs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::System::Decimal, 0usize>("get_MinOccurs")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_MinOccurs", 0usize
                )
            });
        let __cordl_ret: crate::System::Decimal = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_MinimumCapacity(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_MinimumCapacity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_MinimumCapacity", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Namespace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_Namespace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_Namespace", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_NeedColumnChangeEvents(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_NeedColumnChangeEvents")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_NeedColumnChangeEvents", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_NestedParentRelations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRelation>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::System::Data::DataRelation>,
                    >,
                >,
                0usize,
            >("get_NestedParentRelations")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_NestedParentRelations", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRelation>,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_NestedParentsCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_NestedParentsCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_NestedParentsCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ObjectID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_ObjectID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_ObjectID", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ParentRelations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRelationCollection>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRelationCollection>,
                0usize,
            >("get_ParentRelations")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_ParentRelations", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::DataRelationCollection,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Prefix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_Prefix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_Prefix", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_PrimaryKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
                    >,
                >,
                0usize,
            >("get_PrimaryKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_PrimaryKey", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_RecordCapacity(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_RecordCapacity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_RecordCapacity", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_RemotingFormat(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::SerializationFormat> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Data::SerializationFormat,
                0usize,
            >("get_RemotingFormat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_RemotingFormat", 0usize
                )
            });
        let __cordl_ret: crate::System::Data::SerializationFormat = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_RowDiffId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
                0usize,
            >("get_RowDiffId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_RowDiffId", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Hashtable,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Rows(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRowCollection>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataRowCollection>,
                0usize,
            >("get_Rows")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_Rows", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::DataRowCollection,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_SchemaLoading(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_SchemaLoading")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_SchemaLoading", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_SelfNested(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_SelfNested")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_SelfNested", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Site(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::ISite>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::ComponentModel::ISite>,
                0usize,
            >("get_Site")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_Site", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ISite,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_SuspendEnforceConstraints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_SuspendEnforceConstraints")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_SuspendEnforceConstraints", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_TableName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_TableName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_TableName", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_TypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
                0usize,
            >("get_TypeName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_TypeName", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlQualifiedName,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_UKColumnPositionForInference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_UKColumnPositionForInference")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_UKColumnPositionForInference", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
                0usize,
            >("get_XmlText")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "get_XmlText", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_CaseSensitive(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_CaseSensitive")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "set_CaseSensitive", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_ElementColumnCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_ElementColumnCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "set_ElementColumnCount", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_EnforceConstraints(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_EnforceConstraints")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "set_EnforceConstraints", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Locale(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Locale")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "set_Locale", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_MaxOccurs(
        &mut self,
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Decimal),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_MaxOccurs")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "set_MaxOccurs", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_MergingData(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_MergingData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "set_MergingData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_MinOccurs(
        &mut self,
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Decimal),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_MinOccurs")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "set_MinOccurs", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_MinimumCapacity(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_MinimumCapacity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "set_MinimumCapacity", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Namespace(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Namespace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "set_Namespace", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Prefix(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Prefix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "set_Prefix", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_PrimaryKey(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_PrimaryKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "set_PrimaryKey", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_RemotingFormat(
        &mut self,
        value: crate::System::Data::SerializationFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Data::SerializationFormat),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_RemotingFormat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "set_RemotingFormat", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_SuspendEnforceConstraints(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_SuspendEnforceConstraints")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "set_SuspendEnforceConstraints", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_TableName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_TableName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "set_TableName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_TypeName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_TypeName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "set_TypeName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_UKColumnPositionForInference(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_UKColumnPositionForInference")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "set_UKColumnPositionForInference", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_XmlText(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_XmlText")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable as quest_hook::libil2cpp::Type >
                    ::class(), "set_XmlText", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Data+DataTable")]
impl AsRef<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Data::DataTable {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Data+DataTable")]
impl AsMut<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Data::DataTable {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Data+DataTable")]
impl AsRef<crate::System::Xml::Serialization::IXmlSerializable>
for crate::System::Data::DataTable {
    fn as_ref(&self) -> &crate::System::Xml::Serialization::IXmlSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Data+DataTable")]
impl AsMut<crate::System::Xml::Serialization::IXmlSerializable>
for crate::System::Data::DataTable {
    fn as_mut(&mut self) -> &mut crate::System::Xml::Serialization::IXmlSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Data+DataTable+DSRowDiffIdUsageSection")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DataTable_DSRowDiffIdUsageSection {
    pub _targetDS: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
}
#[cfg(feature = "System+Data+DataTable+DSRowDiffIdUsageSection")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Data::DataTable_DSRowDiffIdUsageSection {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "DataTable/DSRowDiffIdUsageSection";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Data+DataTable+DSRowDiffIdUsageSection")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Data::DataTable_DSRowDiffIdUsageSection {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Data+DataTable+DSRowDiffIdUsageSection")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Data::DataTable_DSRowDiffIdUsageSection {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+Data+DataTable+DSRowDiffIdUsageSection")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Data::DataTable_DSRowDiffIdUsageSection {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Data+DataTable+DSRowDiffIdUsageSection")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Data::DataTable_DSRowDiffIdUsageSection {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
        ds: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable_DSRowDiffIdUsageSection as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Prepare")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable_DSRowDiffIdUsageSection as
                    quest_hook::libil2cpp::Type > ::class(), "Prepare", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ds))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+DataTable+RowDiffIdUsageSection")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DataTable_RowDiffIdUsageSection {
    pub _targetTable: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
}
#[cfg(feature = "System+Data+DataTable+RowDiffIdUsageSection")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Data::DataTable_RowDiffIdUsageSection {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "DataTable/RowDiffIdUsageSection";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Data+DataTable+RowDiffIdUsageSection")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Data::DataTable_RowDiffIdUsageSection {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Data+DataTable+RowDiffIdUsageSection")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Data::DataTable_RowDiffIdUsageSection {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+Data+DataTable+RowDiffIdUsageSection")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Data::DataTable_RowDiffIdUsageSection {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Data+DataTable+RowDiffIdUsageSection")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Data::DataTable_RowDiffIdUsageSection {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::DataTable_RowDiffIdUsageSection as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Prepare")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::DataTable_RowDiffIdUsageSection as
                    quest_hook::libil2cpp::Type > ::class(), "Prepare", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (table))?
        };
        Ok(__cordl_ret.into())
    }
}
