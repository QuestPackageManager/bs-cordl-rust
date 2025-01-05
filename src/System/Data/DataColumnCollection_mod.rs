#[cfg(feature = "System+Data+DataColumnCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct DataColumnCollection {
    __cordl_parent: crate::System::Data::InternalDataCollectionBase,
    pub _table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    pub _list: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    pub _defaultNameIndex: i32,
    pub _delayedAddRangeColumns: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
    >,
    pub _columnFromName: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        >,
    >,
    pub _fInClear: bool,
    pub _columnsImplementingIChangeTracking: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
    >,
    pub _nColumnsImplementingIChangeTracking: i32,
    pub _nColumnsImplementingIRevertibleChangeTracking: i32,
    pub CollectionChanged: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::CollectionChangeEventHandler,
    >,
    pub CollectionChanging: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::CollectionChangeEventHandler,
    >,
    pub ColumnPropertyChanged: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::CollectionChangeEventHandler,
    >,
}
#[cfg(feature = "System+Data+DataColumnCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::DataColumnCollection =>
    "System.Data"."DataColumnCollection"
);
#[cfg(feature = "System+Data+DataColumnCollection")]
impl std::ops::Deref for crate::System::Data::DataColumnCollection {
    type Target = crate::System::Data::InternalDataCollectionBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataColumnCollection")]
impl std::ops::DerefMut for crate::System::Data::DataColumnCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataColumnCollection")]
impl crate::System::Data::DataColumnCollection {
    pub fn Add(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (column))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddAt(
        &mut self,
        index: i32,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAt", (index, column))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddColumnsImplementingIChangeTrackingList(
        &mut self,
        dataColumn: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddColumnsImplementingIChangeTrackingList", (dataColumn))?;
        Ok(__cordl_ret.into())
    }
    pub fn ArrayAdd_DataColumn0(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ArrayAdd", (column))?;
        Ok(__cordl_ret.into())
    }
    pub fn ArrayAdd_i32_DataColumn1(
        &mut self,
        index: i32,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ArrayAdd", (index, column))?;
        Ok(__cordl_ret.into())
    }
    pub fn ArrayRemove(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ArrayRemove", (column))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssignName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("AssignName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn BaseAdd(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BaseAdd", (column))?;
        Ok(__cordl_ret.into())
    }
    pub fn BaseGroupSwitch(
        &mut self,
        oldArray: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
        >,
        oldLength: i32,
        newArray: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataColumn>,
        >,
        newLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BaseGroupSwitch", (oldArray, oldLength, newArray, newLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn BaseRemove(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BaseRemove", (column))?;
        Ok(__cordl_ret.into())
    }
    pub fn CanRegisterName(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanRegisterName", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn CanRemove(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        fThrowException: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CanRemove", (column, fThrowException))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckIChangeTracking(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckIChangeTracking", (column))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains_Il2CppString0(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains__cordl_bool1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        caseSensitive: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Contains", (name, caseSensitive))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf(
        &mut self,
        columnName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("IndexOf", (columnName))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfCaseInsensitive(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("IndexOfCaseInsensitive", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeName(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("MakeName", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (table))?;
        Ok(__cordl_object.into())
    }
    pub fn OnCollectionChanged(
        &mut self,
        ccevent: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::CollectionChangeEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCollectionChanged", (ccevent))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnCollectionChanging(
        &mut self,
        ccevent: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::CollectionChangeEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCollectionChanging", (ccevent))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnColumnPropertyChanged(
        &mut self,
        ccevent: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::CollectionChangeEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnPropertyChanged", (ccevent))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterColumnName(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterColumnName", (name, column))?;
        Ok(__cordl_ret.into())
    }
    pub fn Remove(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (column))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveColumnsImplementingIChangeTrackingList(
        &mut self,
        dataColumn: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveColumnsImplementingIChangeTrackingList", (dataColumn))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterName(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterName", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (table))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_CollectionChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::CollectionChangeEventHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_CollectionChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_ColumnPropertyChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::CollectionChangeEventHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_ColumnPropertyChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ColumnsImplementingIChangeTracking(
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
        > = __cordl_object.invoke("get_ColumnsImplementingIChangeTracking", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ColumnsImplementingIChangeTrackingCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_ColumnsImplementingIChangeTrackingCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ColumnsImplementingIRevertibleChangeTrackingCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_ColumnsImplementingIRevertibleChangeTrackingCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item_Il2CppString1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn> = __cordl_object
            .invoke("get_Item", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item_Il2CppString_Il2CppString2(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn> = __cordl_object
            .invoke("get_Item", (name, ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item_i32_0(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn> = __cordl_object
            .invoke("get_Item", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_List(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ArrayList,
        > = __cordl_object.invoke("get_List", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_CollectionChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::CollectionChangeEventHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_CollectionChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_ColumnPropertyChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::CollectionChangeEventHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_ColumnPropertyChanged", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+DataColumnCollection")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::DataColumnCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
