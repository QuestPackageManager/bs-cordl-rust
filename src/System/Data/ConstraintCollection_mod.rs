#[cfg(feature = "System+Data+ConstraintCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct ConstraintCollection {
    __cordl_parent: crate::System::Data::InternalDataCollectionBase,
    pub _table: *mut crate::System::Data::DataTable,
    pub _list: *mut crate::System::Collections::ArrayList,
    pub _defaultNameIndex: i32,
    pub _onCollectionChanged: *mut crate::System::ComponentModel::CollectionChangeEventHandler,
    pub _delayLoadingConstraints: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Data::Constraint,
    >,
    pub _fLoadForeignKeyConstraintsOnly: bool,
}
#[cfg(feature = "System+Data+ConstraintCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::ConstraintCollection =>
    "System.Data"."ConstraintCollection"
);
#[cfg(feature = "System+Data+ConstraintCollection")]
impl std::ops::Deref for crate::System::Data::ConstraintCollection {
    type Target = crate::System::Data::InternalDataCollectionBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ConstraintCollection")]
impl std::ops::DerefMut for crate::System::Data::ConstraintCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ConstraintCollection")]
impl crate::System::Data::ConstraintCollection {
    pub fn AddForeignKeyConstraint(
        &mut self,
        constraint: *mut crate::System::Data::ForeignKeyConstraint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddForeignKeyConstraint", (constraint))?;
        Ok(__cordl_ret)
    }
    pub fn AddUniqueConstraint(
        &mut self,
        constraint: *mut crate::System::Data::UniqueConstraint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddUniqueConstraint", (constraint))?;
        Ok(__cordl_ret)
    }
    pub fn Add_Constraint0(
        &mut self,
        constraint: *mut crate::System::Data::Constraint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (constraint))?;
        Ok(__cordl_ret)
    }
    pub fn Add_Constraint__cordl_bool1(
        &mut self,
        constraint: *mut crate::System::Data::Constraint,
        addUniqueWhenAddingForeign: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (constraint, addUniqueWhenAddingForeign))?;
        Ok(__cordl_ret)
    }
    pub fn Add_String_Il2CppArray__cordl_bool2(
        &mut self,
        name: *mut crate::System::String,
        columns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
        primaryKey: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::Constraint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::Constraint = __cordl_object
            .invoke("Add", (name, columns, primaryKey))?;
        Ok(__cordl_ret)
    }
    pub fn ArrayAdd(
        &mut self,
        constraint: *mut crate::System::Data::Constraint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ArrayAdd", (constraint))?;
        Ok(__cordl_ret)
    }
    pub fn ArrayRemove(
        &mut self,
        constraint: *mut crate::System::Data::Constraint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ArrayRemove", (constraint))?;
        Ok(__cordl_ret)
    }
    pub fn AssignName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("AssignName", ())?;
        Ok(__cordl_ret)
    }
    pub fn AutoGenerated(
        &mut self,
        constraint: *mut crate::System::Data::Constraint,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AutoGenerated", (constraint))?;
        Ok(__cordl_ret)
    }
    pub fn BaseAdd(
        &mut self,
        constraint: *mut crate::System::Data::Constraint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BaseAdd", (constraint))?;
        Ok(__cordl_ret)
    }
    pub fn BaseGroupSwitch(
        &mut self,
        oldArray: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::Constraint,
        >,
        oldLength: i32,
        newArray: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::Constraint,
        >,
        newLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BaseGroupSwitch", (oldArray, oldLength, newArray, newLength))?;
        Ok(__cordl_ret)
    }
    pub fn BaseRemove(
        &mut self,
        constraint: *mut crate::System::Data::Constraint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BaseRemove", (constraint))?;
        Ok(__cordl_ret)
    }
    pub fn CanRemove(
        &mut self,
        constraint: *mut crate::System::Data::Constraint,
        fThrowException: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CanRemove", (constraint, fThrowException))?;
        Ok(__cordl_ret)
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn Contains_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (name))?;
        Ok(__cordl_ret)
    }
    pub fn Contains__cordl_bool1(
        &mut self,
        name: *mut crate::System::String,
        caseSensitive: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Contains", (name, caseSensitive))?;
        Ok(__cordl_ret)
    }
    pub fn FindConstraint(
        &mut self,
        constraint: *mut crate::System::Data::Constraint,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::Constraint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::Constraint = __cordl_object
            .invoke("FindConstraint", (constraint))?;
        Ok(__cordl_ret)
    }
    pub fn FindForeignKeyConstraint(
        &mut self,
        parentColumns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
        childColumns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::ForeignKeyConstraint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::ForeignKeyConstraint = __cordl_object
            .invoke("FindForeignKeyConstraint", (parentColumns, childColumns))?;
        Ok(__cordl_ret)
    }
    pub fn FindKeyConstraint_DataColumn1(
        &mut self,
        column: *mut crate::System::Data::DataColumn,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::UniqueConstraint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::UniqueConstraint = __cordl_object
            .invoke("FindKeyConstraint", (column))?;
        Ok(__cordl_ret)
    }
    pub fn FindKeyConstraint_Il2CppArray0(
        &mut self,
        columns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataColumn,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::UniqueConstraint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::UniqueConstraint = __cordl_object
            .invoke("FindKeyConstraint", (columns))?;
        Ok(__cordl_ret)
    }
    pub fn InternalIndexOf(
        &mut self,
        constraintName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("InternalIndexOf", (constraintName))?;
        Ok(__cordl_ret)
    }
    pub fn MakeName(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("MakeName", (index))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (table))?;
        Ok(__cordl_object)
    }
    pub fn OnCollectionChanged(
        &mut self,
        ccevent: *mut crate::System::ComponentModel::CollectionChangeEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCollectionChanged", (ccevent))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterName(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterName", (name))?;
        Ok(__cordl_ret)
    }
    pub fn Remove(
        &mut self,
        constraint: *mut crate::System::Data::Constraint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (constraint))?;
        Ok(__cordl_ret)
    }
    pub fn UnregisterName(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterName", (name))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
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
    pub fn get_Item_String1(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::Constraint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::Constraint = __cordl_object
            .invoke("get_Item", (name))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_i32_0(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::Constraint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::Constraint = __cordl_object
            .invoke("get_Item", (index))?;
        Ok(__cordl_ret)
    }
    pub fn get_List(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ArrayList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ArrayList = __cordl_object
            .invoke("get_List", ())?;
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
}
#[cfg(feature = "System+Data+ConstraintCollection")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::ConstraintCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
