#[cfg(feature = "System+Data+XmlToDatasetMap+TableSchemaInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlToDatasetMap_TableSchemaInfo {
    __cordl_parent: crate::System::Object,
    pub TableSchema: *mut crate::System::Data::DataTable,
    pub ColumnsSchemaMap: *mut crate::System::Data::XmlToDatasetMap_XmlNodeIdHashtable,
}
#[cfg(feature = "System+Data+XmlToDatasetMap+TableSchemaInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::XmlToDatasetMap_TableSchemaInfo =>
    "System.Data"."XmlToDatasetMap/TableSchemaInfo"
);
#[cfg(feature = "System+Data+XmlToDatasetMap+TableSchemaInfo")]
impl std::ops::Deref for crate::System::Data::XmlToDatasetMap_TableSchemaInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XmlToDatasetMap+TableSchemaInfo")]
impl std::ops::DerefMut for crate::System::Data::XmlToDatasetMap_TableSchemaInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XmlToDatasetMap+TableSchemaInfo")]
impl crate::System::Data::XmlToDatasetMap_TableSchemaInfo {
    pub fn New(
        tableSchema: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tableSchema))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        tableSchema: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tableSchema))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+XmlToDatasetMap+TableSchemaInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Data::XmlToDatasetMap_TableSchemaInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Data+XmlToDatasetMap+XmlNodeIdHashtable")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlToDatasetMap_XmlNodeIdHashtable {
    __cordl_parent: crate::System::Collections::Hashtable,
    pub _id: *mut crate::System::Data::XmlToDatasetMap_XmlNodeIdentety,
}
#[cfg(feature = "System+Data+XmlToDatasetMap+XmlNodeIdHashtable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::XmlToDatasetMap_XmlNodeIdHashtable
    => "System.Data"."XmlToDatasetMap/XmlNodeIdHashtable"
);
#[cfg(feature = "System+Data+XmlToDatasetMap+XmlNodeIdHashtable")]
impl std::ops::Deref for crate::System::Data::XmlToDatasetMap_XmlNodeIdHashtable {
    type Target = crate::System::Collections::Hashtable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XmlToDatasetMap+XmlNodeIdHashtable")]
impl std::ops::DerefMut for crate::System::Data::XmlToDatasetMap_XmlNodeIdHashtable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XmlToDatasetMap+XmlNodeIdHashtable")]
impl crate::System::Data::XmlToDatasetMap_XmlNodeIdHashtable {
    pub fn New(capacity: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (capacity))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (capacity))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_DataTable2(
        &mut self,
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_Item", (table))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_String3(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_Item", (name))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_XmlNode0(
        &mut self,
        node: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_Item", (node))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_XmlReader1(
        &mut self,
        dataReader: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_Item", (dataReader))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+XmlToDatasetMap+XmlNodeIdHashtable")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Data::XmlToDatasetMap_XmlNodeIdHashtable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Data+XmlToDatasetMap+XmlNodeIdentety")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlToDatasetMap_XmlNodeIdentety {
    __cordl_parent: crate::System::Object,
    pub LocalName: *mut crate::System::String,
    pub NamespaceURI: *mut crate::System::String,
}
#[cfg(feature = "System+Data+XmlToDatasetMap+XmlNodeIdentety")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::XmlToDatasetMap_XmlNodeIdentety =>
    "System.Data"."XmlToDatasetMap/XmlNodeIdentety"
);
#[cfg(feature = "System+Data+XmlToDatasetMap+XmlNodeIdentety")]
impl std::ops::Deref for crate::System::Data::XmlToDatasetMap_XmlNodeIdentety {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XmlToDatasetMap+XmlNodeIdentety")]
impl std::ops::DerefMut for crate::System::Data::XmlToDatasetMap_XmlNodeIdentety {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XmlToDatasetMap+XmlNodeIdentety")]
impl crate::System::Data::XmlToDatasetMap_XmlNodeIdentety {
    pub fn Equals(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        localName: *mut crate::System::String,
        namespaceURI: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (localName, namespaceURI))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        localName: *mut crate::System::String,
        namespaceURI: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (localName, namespaceURI))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+XmlToDatasetMap+XmlNodeIdentety")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Data::XmlToDatasetMap_XmlNodeIdentety {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Data+XmlToDatasetMap")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlToDatasetMap {
    __cordl_parent: crate::System::Object,
    pub _tableSchemaMap: *mut crate::System::Data::XmlToDatasetMap_XmlNodeIdHashtable,
    pub _lastTableSchemaInfo: *mut crate::System::Data::XmlToDatasetMap_TableSchemaInfo,
}
#[cfg(feature = "System+Data+XmlToDatasetMap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::XmlToDatasetMap => "System.Data"
    ."XmlToDatasetMap"
);
#[cfg(feature = "System+Data+XmlToDatasetMap")]
impl std::ops::Deref for crate::System::Data::XmlToDatasetMap {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XmlToDatasetMap")]
impl std::ops::DerefMut for crate::System::Data::XmlToDatasetMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XmlToDatasetMap")]
impl crate::System::Data::XmlToDatasetMap {
    #[cfg(feature = "System+Data+XmlToDatasetMap+TableSchemaInfo")]
    pub type TableSchemaInfo = crate::System::Data::XmlToDatasetMap_TableSchemaInfo;
    #[cfg(feature = "System+Data+XmlToDatasetMap+XmlNodeIdentety")]
    pub type XmlNodeIdentety = crate::System::Data::XmlToDatasetMap_XmlNodeIdentety;
    #[cfg(feature = "System+Data+XmlToDatasetMap+XmlNodeIdHashtable")]
    pub type XmlNodeIdHashtable = crate::System::Data::XmlToDatasetMap_XmlNodeIdHashtable;
    pub fn AddColumnSchema_DataColumn_XmlNameTable0(
        &mut self,
        col: *mut crate::System::Data::DataColumn,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        columns: *mut crate::System::Data::XmlToDatasetMap_XmlNodeIdHashtable,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AddColumnSchema", (col, nameTable, columns))?;
        Ok(__cordl_ret)
    }
    pub fn AddColumnSchema_XmlNameTable_DataColumn1(
        &mut self,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        col: *mut crate::System::Data::DataColumn,
        columns: *mut crate::System::Data::XmlToDatasetMap_XmlNodeIdHashtable,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AddColumnSchema", (nameTable, col, columns))?;
        Ok(__cordl_ret)
    }
    pub fn AddTableSchema_DataTable_XmlNameTable0(
        &mut self,
        table: *mut crate::System::Data::DataTable,
        nameTable: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Data::XmlToDatasetMap_TableSchemaInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::XmlToDatasetMap_TableSchemaInfo = __cordl_object
            .invoke("AddTableSchema", (table, nameTable))?;
        Ok(__cordl_ret)
    }
    pub fn AddTableSchema_XmlNameTable_DataTable1(
        &mut self,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Data::XmlToDatasetMap_TableSchemaInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::XmlToDatasetMap_TableSchemaInfo = __cordl_object
            .invoke("AddTableSchema", (nameTable, table))?;
        Ok(__cordl_ret)
    }
    pub fn BuildIdentityMap_DataSet_XmlNameTable0(
        &mut self,
        dataSet: *mut crate::System::Data::DataSet,
        nameTable: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuildIdentityMap", (dataSet, nameTable))?;
        Ok(__cordl_ret)
    }
    pub fn BuildIdentityMap_DataTable_XmlNameTable2(
        &mut self,
        dataTable: *mut crate::System::Data::DataTable,
        nameTable: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuildIdentityMap", (dataTable, nameTable))?;
        Ok(__cordl_ret)
    }
    pub fn BuildIdentityMap_XmlNameTable_DataSet1(
        &mut self,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        dataSet: *mut crate::System::Data::DataSet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuildIdentityMap", (nameTable, dataSet))?;
        Ok(__cordl_ret)
    }
    pub fn BuildIdentityMap_XmlNameTable_DataTable3(
        &mut self,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        dataTable: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuildIdentityMap", (nameTable, dataTable))?;
        Ok(__cordl_ret)
    }
    pub fn GetColumnSchema_DataTable_XmlReader__cordl_bool1(
        &mut self,
        table: *mut crate::System::Data::DataTable,
        dataReader: *mut crate::System::Xml::XmlReader,
        fIgnoreNamespace: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetColumnSchema", (table, dataReader, fIgnoreNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn GetColumnSchema_XmlNode__cordl_bool0(
        &mut self,
        node: *mut crate::System::Xml::XmlNode,
        fIgnoreNamespace: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetColumnSchema", (node, fIgnoreNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn GetSchemaForNode(
        &mut self,
        node: *mut crate::System::Xml::XmlNode,
        fIgnoreNamespace: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetSchemaForNode", (node, fIgnoreNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn GetSelfAndDescendants(
        &mut self,
        dt: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ArrayList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ArrayList = __cordl_object
            .invoke("GetSelfAndDescendants", (dt))?;
        Ok(__cordl_ret)
    }
    pub fn GetTableForNode(
        &mut self,
        node: *mut crate::System::Xml::XmlReader,
        fIgnoreNamespace: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataTable = __cordl_object
            .invoke("GetTableForNode", (node, fIgnoreNamespace))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSpecialColumn(
        &mut self,
        col: *mut crate::System::Data::DataColumn,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        columns: *mut crate::System::Data::XmlToDatasetMap_XmlNodeIdHashtable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSpecialColumn", (col, nameTable, columns))?;
        Ok(__cordl_ret)
    }
    pub fn New_DataSet_XmlNameTable0(
        dataSet: *mut crate::System::Data::DataSet,
        nameTable: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dataSet, nameTable))?;
        Ok(__cordl_object)
    }
    pub fn New_DataTable_XmlNameTable2(
        dataTable: *mut crate::System::Data::DataTable,
        nameTable: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dataTable, nameTable))?;
        Ok(__cordl_object)
    }
    pub fn New_XmlNameTable_DataSet1(
        nameTable: *mut crate::System::Xml::XmlNameTable,
        dataSet: *mut crate::System::Data::DataSet,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nameTable, dataSet))?;
        Ok(__cordl_object)
    }
    pub fn New_XmlNameTable_DataTable3(
        nameTable: *mut crate::System::Xml::XmlNameTable,
        dataTable: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nameTable, dataTable))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_DataSet_XmlNameTable0(
        &mut self,
        dataSet: *mut crate::System::Data::DataSet,
        nameTable: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dataSet, nameTable))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DataTable_XmlNameTable2(
        &mut self,
        dataTable: *mut crate::System::Data::DataTable,
        nameTable: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dataTable, nameTable))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_XmlNameTable_DataSet1(
        &mut self,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        dataSet: *mut crate::System::Data::DataSet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nameTable, dataSet))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_XmlNameTable_DataTable3(
        &mut self,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        dataTable: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nameTable, dataTable))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+XmlToDatasetMap")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::XmlToDatasetMap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
