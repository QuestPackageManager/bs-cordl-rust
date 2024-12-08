#[cfg(feature = "System+Data+DataTableCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct DataTableCollection {
    __cordl_parent: crate::System::Data::InternalDataCollectionBase,
    pub _dataSet: *mut crate::System::Data::DataSet,
    pub _list: *mut crate::System::Collections::ArrayList,
    pub _defaultNameIndex: i32,
    pub _delayedAddRangeTables: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Data::DataTable,
    >,
    pub _onCollectionChangedDelegate: *mut crate::System::ComponentModel::CollectionChangeEventHandler,
    pub _onCollectionChangingDelegate: *mut crate::System::ComponentModel::CollectionChangeEventHandler,
    pub _objectID: i32,
}
#[cfg(feature = "System+Data+DataTableCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::DataTableCollection =>
    "System.Data"."DataTableCollection"
);
#[cfg(feature = "System+Data+DataTableCollection")]
impl std::ops::Deref for crate::System::Data::DataTableCollection {
    type Target = crate::System::Data::InternalDataCollectionBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataTableCollection")]
impl std::ops::DerefMut for crate::System::Data::DataTableCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataTableCollection")]
impl crate::System::Data::DataTableCollection {
    pub fn BaseAdd(
        &mut self,
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BaseAdd", (table))?;
        Ok(__cordl_ret)
    }
    pub fn BaseRemove(
        &mut self,
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BaseRemove", (table))?;
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
    pub fn Contains_String__cordl_bool__cordl_bool1(
        &mut self,
        name: *mut crate::System::String,
        tableNamespace: *mut crate::System::String,
        checkProperty: bool,
        caseSensitive: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Contains", (name, tableNamespace, checkProperty, caseSensitive))?;
        Ok(__cordl_ret)
    }
    pub fn Contains__cordl_bool2(
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
    pub fn ReplaceFromInference(
        &mut self,
        tableList: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Data::DataTable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReplaceFromInference", (tableList))?;
        Ok(__cordl_ret)
    }
    pub fn get_ObjectID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ObjectID", ())?;
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
    pub fn Add(
        &mut self,
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (table))?;
        Ok(__cordl_ret)
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
    pub fn GetTable(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataTable = __cordl_object
            .invoke("GetTable", (name, ns))?;
        Ok(__cordl_ret)
    }
    pub fn OnCollectionChanging(
        &mut self,
        ccevent: *mut crate::System::ComponentModel::CollectionChangeEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCollectionChanging", (ccevent))?;
        Ok(__cordl_ret)
    }
    pub fn InternalIndexOf_String0(
        &mut self,
        tableName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("InternalIndexOf", (tableName))?;
        Ok(__cordl_ret)
    }
    pub fn InternalIndexOf_String1(
        &mut self,
        tableName: *mut crate::System::String,
        tableNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("InternalIndexOf", (tableName, tableNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn Remove(
        &mut self,
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (table))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_i32_0(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataTable = __cordl_object
            .invoke("get_Item", (index))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_String1(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataTable = __cordl_object
            .invoke("get_Item", (name))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_String_String2(
        &mut self,
        name: *mut crate::System::String,
        tableNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataTable = __cordl_object
            .invoke("get_Item", (name, tableNamespace))?;
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
    pub fn GetTableSmart(
        &mut self,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataTable = __cordl_object
            .invoke("GetTableSmart", (name, ns))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        dataSet: *mut crate::System::Data::DataSet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dataSet))?;
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
    pub fn RegisterName(
        &mut self,
        name: *mut crate::System::String,
        tbNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterName", (name, tbNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn IndexOf_DataTable0(
        &mut self,
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("IndexOf", (table))?;
        Ok(__cordl_ret)
    }
    pub fn IndexOf_String1(
        &mut self,
        tableName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("IndexOf", (tableName))?;
        Ok(__cordl_ret)
    }
    pub fn IndexOf_String_String__cordl_bool2(
        &mut self,
        tableName: *mut crate::System::String,
        tableNamespace: *mut crate::System::String,
        chekforNull: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("IndexOf", (tableName, tableNamespace, chekforNull))?;
        Ok(__cordl_ret)
    }
    pub fn BaseGroupSwitch(
        &mut self,
        oldArray: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataTable,
        >,
        oldLength: i32,
        newArray: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataTable,
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
    pub fn CanRemove(
        &mut self,
        table: *mut crate::System::Data::DataTable,
        fThrowException: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CanRemove", (table, fThrowException))?;
        Ok(__cordl_ret)
    }
    pub fn ArrayAdd(
        &mut self,
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ArrayAdd", (table))?;
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
    pub fn New(
        dataSet: *mut crate::System::Data::DataSet,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dataSet))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Data+DataTableCollection")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::DataTableCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
