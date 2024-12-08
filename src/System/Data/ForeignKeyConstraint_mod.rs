#[cfg(feature = "System+Data+ForeignKeyConstraint")]
#[repr(C)]
#[derive(Debug)]
pub struct ForeignKeyConstraint {
    __cordl_parent: crate::System::Data::Constraint,
    pub _deleteRule: crate::System::Data::Rule,
    pub _updateRule: crate::System::Data::Rule,
    pub _acceptRejectRule: crate::System::Data::AcceptRejectRule,
    pub _childKey: crate::System::Data::DataKey,
    pub _parentKey: crate::System::Data::DataKey,
    pub _constraintName: *mut crate::System::String,
    pub _parentColumnNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub _childColumnNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub _parentTableName: *mut crate::System::String,
}
#[cfg(feature = "System+Data+ForeignKeyConstraint")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::ForeignKeyConstraint =>
    "System.Data"."ForeignKeyConstraint"
);
#[cfg(feature = "System+Data+ForeignKeyConstraint")]
impl std::ops::Deref for crate::System::Data::ForeignKeyConstraint {
    type Target = crate::System::Data::Constraint;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ForeignKeyConstraint")]
impl std::ops::DerefMut for crate::System::Data::ForeignKeyConstraint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ForeignKeyConstraint")]
impl crate::System::Data::ForeignKeyConstraint {
    pub fn NonVirtualCheckState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NonVirtualCheckState", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckCanAddToCollection(
        &mut self,
        constraints: *mut crate::System::Data::ConstraintCollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckCanAddToCollection", (constraints))?;
        Ok(__cordl_ret)
    }
    pub fn CascadeUpdate(
        &mut self,
        row: *mut crate::System::Data::DataRow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CascadeUpdate", (row))?;
        Ok(__cordl_ret)
    }
    pub fn get_DeleteRule(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::Rule> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::Rule = __cordl_object
            .invoke("get_DeleteRule", ())?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
        &mut self,
        key: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (key))?;
        Ok(__cordl_ret)
    }
    pub fn get_Columns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        > = __cordl_object.invoke("get_Columns", ())?;
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
    pub fn CheckCanRemoveParentRow(
        &mut self,
        row: *mut crate::System::Data::DataRow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckCanRemoveParentRow", (row))?;
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
    pub fn CascadeRollback(
        &mut self,
        row: *mut crate::System::Data::DataRow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CascadeRollback", (row))?;
        Ok(__cordl_ret)
    }
    pub fn get_UpdateRule(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::Rule> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::Rule = __cordl_object
            .invoke("get_UpdateRule", ())?;
        Ok(__cordl_ret)
    }
    pub fn CascadeDelete(
        &mut self,
        row: *mut crate::System::Data::DataRow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CascadeDelete", (row))?;
        Ok(__cordl_ret)
    }
    pub fn set_AcceptRejectRule(
        &mut self,
        value: crate::System::Data::AcceptRejectRule,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AcceptRejectRule", (value))?;
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
    pub fn IsKeyNull(
        &mut self,
        values: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsKeyNull", (values))?;
        Ok(__cordl_ret)
    }
    pub fn IsConstraintViolated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsConstraintViolated", ())?;
        Ok(__cordl_ret)
    }
    pub fn CanEnableConstraint(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanEnableConstraint", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckConstraint(
        &mut self,
        childRow: *mut crate::System::Data::DataRow,
        action: crate::System::Data::DataRowAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckConstraint", (childRow, action))?;
        Ok(__cordl_ret)
    }
    pub fn get_AcceptRejectRule(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::AcceptRejectRule> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::AcceptRejectRule = __cordl_object
            .invoke("get_AcceptRejectRule", ())?;
        Ok(__cordl_ret)
    }
    pub fn ContainsColumn(
        &mut self,
        column: *mut crate::System::Data::DataColumn,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ContainsColumn", (column))?;
        Ok(__cordl_ret)
    }
    pub fn Clone_DataSet0(
        &mut self,
        destination: *mut crate::System::Data::DataSet,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::Constraint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::Constraint = __cordl_object
            .invoke("Clone", (destination))?;
        Ok(__cordl_ret)
    }
    pub fn Clone_DataSet__cordl_bool1(
        &mut self,
        destination: *mut crate::System::Data::DataSet,
        ignorNSforTableLookup: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::Constraint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::Constraint = __cordl_object
            .invoke("Clone", (destination, ignorNSforTableLookup))?;
        Ok(__cordl_ret)
    }
    pub fn Clone_DataTable2(
        &mut self,
        destination: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::ForeignKeyConstraint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::ForeignKeyConstraint = __cordl_object
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Create", (relationName, parentColumns, childColumns))?;
        Ok(__cordl_ret)
    }
    pub fn FindParentRelation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataRelation> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRelation = __cordl_object
            .invoke("FindParentRelation", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_UpdateRule(
        &mut self,
        value: crate::System::Data::Rule,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UpdateRule", (value))?;
        Ok(__cordl_ret)
    }
    pub fn CanBeRemovedFromCollection(
        &mut self,
        constraints: *mut crate::System::Data::ConstraintCollection,
        fThrowException: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CanBeRemovedFromCollection", (constraints, fThrowException))?;
        Ok(__cordl_ret)
    }
    pub fn set_DeleteRule(
        &mut self,
        value: crate::System::Data::Rule,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DeleteRule", (value))?;
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
    pub fn get_RelatedTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataTable = __cordl_object
            .invoke("get_RelatedTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn CascadeCommit(
        &mut self,
        row: *mut crate::System::Data::DataRow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CascadeCommit", (row))?;
        Ok(__cordl_ret)
    }
    pub fn CheckCanClearParentTable(
        &mut self,
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckCanClearParentTable", (table))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
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
    pub fn CheckCascade(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        action: crate::System::Data::DataRowAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckCascade", (row, action))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray_Il2CppArray0(
        &mut self,
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
            .invoke(".ctor", (parentColumns, childColumns))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray_Il2CppArray1(
        &mut self,
        constraintName: *mut crate::System::String,
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
            .invoke(".ctor", (constraintName, parentColumns, childColumns))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_String_Il2CppArray_Il2CppArray_AcceptRejectRule_Rule_Rule2(
        &mut self,
        constraintName: *mut crate::System::String,
        parentTableName: *mut crate::System::String,
        parentColumnNames: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
        childColumnNames: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
        acceptRejectRule: crate::System::Data::AcceptRejectRule,
        deleteRule: crate::System::Data::Rule,
        updateRule: crate::System::Data::Rule,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    constraintName,
                    parentTableName,
                    parentColumnNames,
                    childColumnNames,
                    acceptRejectRule,
                    deleteRule,
                    updateRule,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_RelatedColumns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        > = __cordl_object.invoke("get_RelatedColumns", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RelatedColumnsReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        > = __cordl_object.invoke("get_RelatedColumnsReference", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Il2CppArray_Il2CppArray0(
        parentColumns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
        childColumns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parentColumns, childColumns))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Il2CppArray_Il2CppArray1(
        constraintName: *mut crate::System::String,
        parentColumns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
        childColumns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (constraintName, parentColumns, childColumns))?;
        Ok(__cordl_object)
    }
    pub fn New_String_String_Il2CppArray_Il2CppArray_AcceptRejectRule_Rule_Rule2(
        constraintName: *mut crate::System::String,
        parentTableName: *mut crate::System::String,
        parentColumnNames: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
        childColumnNames: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
        acceptRejectRule: crate::System::Data::AcceptRejectRule,
        deleteRule: crate::System::Data::Rule,
        updateRule: crate::System::Data::Rule,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    constraintName,
                    parentTableName,
                    parentColumnNames,
                    childColumnNames,
                    acceptRejectRule,
                    deleteRule,
                    updateRule,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Data+ForeignKeyConstraint")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::ForeignKeyConstraint {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
