#[cfg(feature = "System+Data+DataRelation")]
#[repr(C)]
#[derive(Debug)]
pub struct DataRelation {
    __cordl_parent: crate::System::Object,
    pub _dataSet: *mut crate::System::Data::DataSet,
    pub _extendedProperties: *mut crate::System::Data::PropertyCollection,
    pub _relationName: *mut crate::System::String,
    pub _childKey: crate::System::Data::DataKey,
    pub _parentKey: crate::System::Data::DataKey,
    pub _parentKeyConstraint: *mut crate::System::Data::UniqueConstraint,
    pub _childKeyConstraint: *mut crate::System::Data::ForeignKeyConstraint,
    pub _parentColumnNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub _childColumnNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub _parentTableName: *mut crate::System::String,
    pub _childTableName: *mut crate::System::String,
    pub _parentTableNamespace: *mut crate::System::String,
    pub _childTableNamespace: *mut crate::System::String,
    pub _nested: bool,
    pub _createConstraints: bool,
    pub _checkMultipleNested: bool,
    pub _objectID: i32,
    pub PropertyChanging: *mut crate::System::ComponentModel::PropertyChangedEventHandler,
}
#[cfg(feature = "System+Data+DataRelation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::DataRelation => "System.Data"
    ."DataRelation"
);
#[cfg(feature = "System+Data+DataRelation")]
impl std::ops::Deref for crate::System::Data::DataRelation {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataRelation")]
impl std::ops::DerefMut for crate::System::Data::DataRelation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataRelation")]
impl crate::System::Data::DataRelation {
    pub fn CheckNamespaceValidityForNestedRelations(
        &mut self,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckNamespaceValidityForNestedRelations", (ns))?;
        Ok(__cordl_ret)
    }
    pub fn CheckNestedRelations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckNestedRelations", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckState", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckStateForProperty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckStateForProperty", ())?;
        Ok(__cordl_ret)
    }
    pub fn Clone(
        &mut self,
        destination: *mut crate::System::Data::DataSet,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataRelation> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRelation = __cordl_object
            .invoke("Clone", (destination))?;
        Ok(__cordl_ret)
    }
    pub fn Create(
        &mut self,
        relationName: *mut crate::System::String,
        parentColumns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
        childColumns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
        createConstraints: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Create",
                (relationName, parentColumns, childColumns, createConstraints),
            )?;
        Ok(__cordl_ret)
    }
    pub fn IsAutoGenerated(
        &mut self,
        col: *mut crate::System::Data::DataColumn,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsAutoGenerated", (col))?;
        Ok(__cordl_ret)
    }
    pub fn New_DataColumn_DataColumn__cordl_bool0(
        relationName: *mut crate::System::String,
        parentColumn: *mut crate::System::Data::DataColumn,
        childColumn: *mut crate::System::Data::DataColumn,
        createConstraints: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (relationName, parentColumn, childColumn, createConstraints),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray_Il2CppArray1(
        relationName: *mut crate::System::String,
        parentColumns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
        childColumns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (relationName, parentColumns, childColumns))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray_Il2CppArray__cordl_bool2(
        relationName: *mut crate::System::String,
        parentColumns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
        childColumns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
        createConstraints: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (relationName, parentColumns, childColumns, createConstraints),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_String_String_Il2CppArray_Il2CppArray__cordl_bool3(
        relationName: *mut crate::System::String,
        parentTableName: *mut crate::System::String,
        childTableName: *mut crate::System::String,
        parentColumnNames: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
        childColumnNames: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
        nested: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    relationName,
                    parentTableName,
                    childTableName,
                    parentColumnNames,
                    childColumnNames,
                    nested,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_String_String_String_String_Il2CppArray_Il2CppArray__cordl_bool4(
        relationName: *mut crate::System::String,
        parentTableName: *mut crate::System::String,
        parentTableNamespace: *mut crate::System::String,
        childTableName: *mut crate::System::String,
        childTableNamespace: *mut crate::System::String,
        parentColumnNames: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
        childColumnNames: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
        nested: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    relationName,
                    parentTableName,
                    parentTableNamespace,
                    childTableName,
                    childTableNamespace,
                    parentColumnNames,
                    childColumnNames,
                    nested,
                ),
            )?;
        Ok(__cordl_object)
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
    pub fn SetChildKeyConstraint(
        &mut self,
        value: *mut crate::System::Data::ForeignKeyConstraint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetChildKeyConstraint", (value))?;
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
    pub fn SetParentKeyConstraint(
        &mut self,
        value: *mut crate::System::Data::UniqueConstraint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetParentKeyConstraint", (value))?;
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
    pub fn ValidateMultipleNestedRelations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateMultipleNestedRelations", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DataColumn_DataColumn__cordl_bool0(
        &mut self,
        relationName: *mut crate::System::String,
        parentColumn: *mut crate::System::Data::DataColumn,
        childColumn: *mut crate::System::Data::DataColumn,
        createConstraints: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (relationName, parentColumn, childColumn, createConstraints),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray_Il2CppArray1(
        &mut self,
        relationName: *mut crate::System::String,
        parentColumns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
        childColumns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (relationName, parentColumns, childColumns))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray_Il2CppArray__cordl_bool2(
        &mut self,
        relationName: *mut crate::System::String,
        parentColumns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
        childColumns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
        createConstraints: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (relationName, parentColumns, childColumns, createConstraints),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_String_Il2CppArray_Il2CppArray__cordl_bool3(
        &mut self,
        relationName: *mut crate::System::String,
        parentTableName: *mut crate::System::String,
        childTableName: *mut crate::System::String,
        parentColumnNames: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
        childColumnNames: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
        nested: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    relationName,
                    parentTableName,
                    childTableName,
                    parentColumnNames,
                    childColumnNames,
                    nested,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_String_String_String_Il2CppArray_Il2CppArray__cordl_bool4(
        &mut self,
        relationName: *mut crate::System::String,
        parentTableName: *mut crate::System::String,
        parentTableNamespace: *mut crate::System::String,
        childTableName: *mut crate::System::String,
        childTableNamespace: *mut crate::System::String,
        parentColumnNames: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
        childColumnNames: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
        nested: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    relationName,
                    parentTableName,
                    parentTableNamespace,
                    childTableName,
                    childTableNamespace,
                    parentColumnNames,
                    childColumnNames,
                    nested,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_CheckMultipleNested(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CheckMultipleNested", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ChildColumnNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_ChildColumnNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ChildColumns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        > = __cordl_object.invoke("get_ChildColumns", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ChildColumnsReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        > = __cordl_object.invoke("get_ChildColumnsReference", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ChildKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::DataKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::DataKey = __cordl_object
            .invoke("get_ChildKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ChildKeyConstraint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::ForeignKeyConstraint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::ForeignKeyConstraint = __cordl_object
            .invoke("get_ChildKeyConstraint", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ChildTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataTable = __cordl_object
            .invoke("get_ChildTable", ())?;
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
    pub fn get_Nested(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Nested", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ObjectID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ObjectID", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ParentColumnNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_ParentColumnNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ParentColumns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        > = __cordl_object.invoke("get_ParentColumns", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ParentColumnsReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        > = __cordl_object.invoke("get_ParentColumnsReference", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ParentKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::DataKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::DataKey = __cordl_object
            .invoke("get_ParentKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ParentKeyConstraint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::UniqueConstraint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::UniqueConstraint = __cordl_object
            .invoke("get_ParentKeyConstraint", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ParentTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataTable = __cordl_object
            .invoke("get_ParentTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RelationName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_RelationName", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_CheckMultipleNested(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CheckMultipleNested", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Nested(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Nested", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+DataRelation")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::DataRelation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
