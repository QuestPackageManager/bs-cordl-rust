#[cfg(feature = "System+Data+DataRelation")]
#[repr(C)]
#[derive(Debug)]
pub struct DataRelation {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _dataSet: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
    pub _extendedProperties: quest_hook::libil2cpp::Gc<
        crate::System::Data::PropertyCollection,
    >,
    pub _relationName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _childKey: crate::System::Data::DataKey,
    pub _parentKey: crate::System::Data::DataKey,
    pub _parentKeyConstraint: quest_hook::libil2cpp::Gc<
        crate::System::Data::UniqueConstraint,
    >,
    pub _childKeyConstraint: quest_hook::libil2cpp::Gc<
        crate::System::Data::ForeignKeyConstraint,
    >,
    pub _parentColumnNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub _childColumnNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub _parentTableName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _childTableName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _parentTableNamespace: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _childTableNamespace: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _nested: bool,
    pub _createConstraints: bool,
    pub _checkMultipleNested: bool,
    pub _objectID: i32,
    pub PropertyChanging: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::PropertyChangedEventHandler,
    >,
}
#[cfg(feature = "System+Data+DataRelation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::DataRelation => "System.Data"
    ."DataRelation"
);
#[cfg(feature = "System+Data+DataRelation")]
impl std::ops::Deref for crate::System::Data::DataRelation {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckNamespaceValidityForNestedRelations", (ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckNestedRelations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckNestedRelations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckStateForProperty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckStateForProperty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Clone(
        &mut self,
        destination: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRelation>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataRelation> = __cordl_object
            .invoke("Clone", (destination))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create(
        &mut self,
        relationName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentColumns: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
        >,
        childColumns: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
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
        Ok(__cordl_ret.into())
    }
    pub fn GetChildRows(
        parentKey: crate::System::Data::DataKey,
        childKey: crate::System::Data::DataKey,
        parentRow: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        version: crate::System::Data::DataRowVersion,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataRow>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataRow>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetChildRows", (parentKey, childKey, parentRow, version))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParentRow(
        parentKey: crate::System::Data::DataKey,
        childKey: crate::System::Data::DataKey,
        childRow: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        version: crate::System::Data::DataRowVersion,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetParentRow", (parentKey, childKey, childRow, version))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParentRows(
        parentKey: crate::System::Data::DataKey,
        childKey: crate::System::Data::DataKey,
        childRow: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        version: crate::System::Data::DataRowVersion,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataRow>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataRow>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetParentRows", (parentKey, childKey, childRow, version))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAutoGenerated(
        &mut self,
        col: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsAutoGenerated", (col))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsKeyNull(
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsKeyNull", (values))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_DataColumn_DataColumn__cordl_bool0(
        relationName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentColumn: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        childColumn: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        createConstraints: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (relationName, parentColumn, childColumn, createConstraints),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppArray_Il2CppArray1(
        relationName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentColumns: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
        >,
        childColumns: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (relationName, parentColumns, childColumns))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppArray_Il2CppArray__cordl_bool2(
        relationName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentColumns: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
        >,
        childColumns: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
        >,
        createConstraints: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (relationName, parentColumns, childColumns, createConstraints),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_Il2CppString_Il2CppArray_Il2CppArray__cordl_bool3(
        relationName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentTableName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        childTableName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentColumnNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        childColumnNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        nested: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_Il2CppString_Il2CppString_Il2CppString_Il2CppArray_Il2CppArray__cordl_bool4(
        relationName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentTableName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentTableNamespace: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        childTableName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        childTableNamespace: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        parentColumnNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        childColumnNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        nested: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn OnPropertyChanging(
        &mut self,
        pcevent: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyChangedEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPropertyChanging", (pcevent))?;
        Ok(__cordl_ret.into())
    }
    pub fn RaisePropertyChanging(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaisePropertyChanging", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetChildKeyConstraint(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Data::ForeignKeyConstraint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetChildKeyConstraint", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDataSet(
        &mut self,
        dataSet: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDataSet", (dataSet))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetParentKeyConstraint(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Data::UniqueConstraint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetParentKeyConstraint", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateMultipleNestedRelations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateMultipleNestedRelations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DataColumn_DataColumn__cordl_bool0(
        &mut self,
        relationName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentColumn: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        childColumn: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray_Il2CppArray1(
        &mut self,
        relationName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentColumns: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
        >,
        childColumns: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (relationName, parentColumns, childColumns))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray_Il2CppArray__cordl_bool2(
        &mut self,
        relationName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentColumns: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
        >,
        childColumns: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Il2CppString_Il2CppArray_Il2CppArray__cordl_bool3(
        &mut self,
        relationName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentTableName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        childTableName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentColumnNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        childColumnNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Il2CppString_Il2CppString_Il2CppString_Il2CppArray_Il2CppArray__cordl_bool4(
        &mut self,
        relationName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentTableName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentTableNamespace: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        childTableName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        childTableNamespace: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        parentColumnNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        childColumnNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
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
        Ok(__cordl_ret.into())
    }
    pub fn get_CheckMultipleNested(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CheckMultipleNested", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ChildColumnNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_ChildColumnNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ChildColumns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
        > = __cordl_object.invoke("get_ChildColumns", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ChildColumnsReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
        > = __cordl_object.invoke("get_ChildColumnsReference", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ChildKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::DataKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::DataKey = __cordl_object
            .invoke("get_ChildKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ChildKeyConstraint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::ForeignKeyConstraint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::ForeignKeyConstraint,
        > = __cordl_object.invoke("get_ChildKeyConstraint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ChildTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable> = __cordl_object
            .invoke("get_ChildTable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DataSet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet> = __cordl_object
            .invoke("get_DataSet", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ExtendedProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::PropertyCollection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::PropertyCollection,
        > = __cordl_object.invoke("get_ExtendedProperties", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Nested(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Nested", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ObjectID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ObjectID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ParentColumnNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_ParentColumnNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ParentColumns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
        > = __cordl_object.invoke("get_ParentColumns", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ParentColumnsReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
        > = __cordl_object.invoke("get_ParentColumnsReference", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ParentKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::DataKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::DataKey = __cordl_object
            .invoke("get_ParentKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ParentKeyConstraint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::UniqueConstraint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::UniqueConstraint,
        > = __cordl_object.invoke("get_ParentKeyConstraint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ParentTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable> = __cordl_object
            .invoke("get_ParentTable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RelationName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_RelationName", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
