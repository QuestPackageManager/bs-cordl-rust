#[cfg(feature = "System+Data+UniqueConstraint")]
#[repr(C)]
#[derive(Debug)]
pub struct UniqueConstraint {
    __cordl_parent: crate::System::Data::Constraint,
    pub _key: crate::System::Data::DataKey,
    pub _constraintIndex: *mut crate::System::Data::Index,
    pub _bPrimaryKey: bool,
    pub _constraintName: *mut crate::System::String,
    pub _columnNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "System+Data+UniqueConstraint")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::UniqueConstraint => "System.Data"
    ."UniqueConstraint"
);
#[cfg(feature = "System+Data+UniqueConstraint")]
impl std::ops::Deref for crate::System::Data::UniqueConstraint {
    type Target = crate::System::Data::Constraint;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+UniqueConstraint")]
impl std::ops::DerefMut for crate::System::Data::UniqueConstraint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+UniqueConstraint")]
impl crate::System::Data::UniqueConstraint {
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
    pub fn CanEnableConstraint(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanEnableConstraint", ())?;
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
    pub fn CheckConstraint(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        action: crate::System::Data::DataRowAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckConstraint", (row, action))?;
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
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::UniqueConstraint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::UniqueConstraint = __cordl_object
            .invoke("Clone", (table))?;
        Ok(__cordl_ret)
    }
    pub fn ConstraintIndexClear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConstraintIndexClear", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConstraintIndexInitialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConstraintIndexInitialize", ())?;
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
    pub fn Create(
        &mut self,
        constraintName: *mut crate::System::String,
        columns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Create", (constraintName, columns))?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
        &mut self,
        key2: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (key2))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsConstraintViolated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsConstraintViolated", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_DataColumn0(
        column: *mut crate::System::Data::DataColumn,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (column))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray2(
        columns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (columns))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Il2CppArray1(
        name: *mut crate::System::String,
        columns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, columns))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Il2CppArray__cordl_bool3(
        name: *mut crate::System::String,
        columnNames: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        isPrimaryKey: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, columnNames, isPrimaryKey))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Il2CppArray__cordl_bool4(
        name: *mut crate::System::String,
        columns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
        isPrimaryKey: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, columns, isPrimaryKey))?;
        Ok(__cordl_object)
    }
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
    pub fn _ctor_DataColumn0(
        &mut self,
        column: *mut crate::System::Data::DataColumn,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (column))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray2(
        &mut self,
        columns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (columns))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray1(
        &mut self,
        name: *mut crate::System::String,
        columns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, columns))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray__cordl_bool3(
        &mut self,
        name: *mut crate::System::String,
        columnNames: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        isPrimaryKey: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, columnNames, isPrimaryKey))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray__cordl_bool4(
        &mut self,
        name: *mut crate::System::String,
        columns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
        isPrimaryKey: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, columns, isPrimaryKey))?;
        Ok(__cordl_ret)
    }
    pub fn get_ColumnNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_ColumnNames", ())?;
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
    pub fn get_ColumnsReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        > = __cordl_object.invoke("get_ColumnsReference", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ConstraintIndex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::Index> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::Index = __cordl_object
            .invoke("get_ConstraintIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsPrimaryKey(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsPrimaryKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Key(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::DataKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::DataKey = __cordl_object
            .invoke("get_Key", ())?;
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
    pub fn set_InCollection(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_InCollection", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+UniqueConstraint")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::UniqueConstraint {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}