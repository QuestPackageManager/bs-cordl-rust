#[cfg(feature = "System+Data+XmlDataLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlDataLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _dataSet: *mut crate::System::Data::DataSet,
    pub _nodeToSchemaMap: *mut crate::System::Data::XmlToDatasetMap,
    pub _nodeToRowMap: *mut crate::System::Collections::Hashtable,
    pub _childRowsStack: *mut crate::System::Collections::Stack,
    pub _htableExcludedNS: *mut crate::System::Collections::Hashtable,
    pub _fIsXdr: bool,
    pub _isDiffgram: bool,
    pub _topMostNode: *mut crate::System::Xml::XmlElement,
    pub _ignoreSchema: bool,
    pub _dataTable: *mut crate::System::Data::DataTable,
    pub _isTableLevel: bool,
    pub _fromInference: bool,
    pub _dataReader: *mut crate::System::Xml::XmlReader,
    pub _XSD_XMLNS_NS: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _XDR_SCHEMA: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _XDRNS: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _SQL_SYNC: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _UPDGNS: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _XSD_SCHEMA: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _XSDNS: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _DFFNS: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _MSDNS: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _DIFFID: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _HASCHANGES: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _ROWORDER: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Data+XmlDataLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::XmlDataLoader => "System.Data"
    ."XmlDataLoader"
);
#[cfg(feature = "System+Data+XmlDataLoader")]
impl std::ops::Deref for crate::System::Data::XmlDataLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XmlDataLoader")]
impl std::ops::DerefMut for crate::System::Data::XmlDataLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XmlDataLoader")]
impl crate::System::Data::XmlDataLoader {
    pub fn AttachRows(
        &mut self,
        parentRow: *mut crate::System::Data::DataRow,
        parentElement: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AttachRows", (parentRow, parentElement))?;
        Ok(__cordl_ret)
    }
    pub fn CountNonNSAttributes(
        &mut self,
        node: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CountNonNSAttributes", (node))?;
        Ok(__cordl_ret)
    }
    pub fn FColumnElement(
        &mut self,
        e: *mut crate::System::Xml::XmlElement,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("FColumnElement", (e))?;
        Ok(__cordl_ret)
    }
    pub fn FExcludedNamespace(
        &mut self,
        ns: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("FExcludedNamespace", (ns))?;
        Ok(__cordl_ret)
    }
    pub fn FIgnoreNamespace_XmlNode0(
        &mut self,
        node: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("FIgnoreNamespace", (node))?;
        Ok(__cordl_ret)
    }
    pub fn FIgnoreNamespace_XmlReader1(
        &mut self,
        node: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("FIgnoreNamespace", (node))?;
        Ok(__cordl_ret)
    }
    pub fn GetInitialTextFromNodes(
        &mut self,
        n: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Xml::XmlNode>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("GetInitialTextFromNodes", (n))?;
        Ok(__cordl_ret)
    }
    pub fn GetRowFromElement(
        &mut self,
        e: *mut crate::System::Xml::XmlElement,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataRow> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRow = __cordl_object
            .invoke("GetRowFromElement", (e))?;
        Ok(__cordl_ret)
    }
    pub fn GetTextOnlyColumn(
        &mut self,
        row: *mut crate::System::Data::DataRow,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataColumn> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataColumn = __cordl_object
            .invoke("GetTextOnlyColumn", (row))?;
        Ok(__cordl_ret)
    }
    pub fn GetValueForTextOnlyColums(
        &mut self,
        n: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("GetValueForTextOnlyColums", (n))?;
        Ok(__cordl_ret)
    }
    pub fn InitNameTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitNameTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsTextLikeNode(
        &mut self,
        n: crate::System::Xml::XmlNodeType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsTextLikeNode", (n))?;
        Ok(__cordl_ret)
    }
    pub fn IsTextOnly(
        &mut self,
        c: *mut crate::System::Data::DataColumn,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsTextOnly", (c))?;
        Ok(__cordl_ret)
    }
    pub fn LoadColumn(
        &mut self,
        column: *mut crate::System::Data::DataColumn,
        foundColumns: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadColumn", (column, foundColumns))?;
        Ok(__cordl_ret)
    }
    pub fn LoadData_XmlDocument0(
        &mut self,
        xdoc: *mut crate::System::Xml::XmlDocument,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadData", (xdoc))?;
        Ok(__cordl_ret)
    }
    pub fn LoadData_XmlReader1(
        &mut self,
        reader: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadData", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn LoadRowData(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        rowElement: *mut crate::System::Xml::XmlElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadRowData", (row, rowElement))?;
        Ok(__cordl_ret)
    }
    pub fn LoadRows(
        &mut self,
        parentRow: *mut crate::System::Data::DataRow,
        parentElement: *mut crate::System::Xml::XmlNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadRows", (parentRow, parentElement))?;
        Ok(__cordl_ret)
    }
    pub fn LoadTable(
        &mut self,
        table: *mut crate::System::Data::DataTable,
        isNested: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadTable", (table, isNested))?;
        Ok(__cordl_ret)
    }
    pub fn LoadTopMostTable(
        &mut self,
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadTopMostTable", (table))?;
        Ok(__cordl_ret)
    }
    pub fn New_DataSet_XmlElement__cordl_bool1(
        dataset: *mut crate::System::Data::DataSet,
        IsXdr: bool,
        topNode: *mut crate::System::Xml::XmlElement,
        ignoreSchema: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dataset, IsXdr, topNode, ignoreSchema))?;
        Ok(__cordl_object)
    }
    pub fn New_DataSet__cordl_bool0(
        dataset: *mut crate::System::Data::DataSet,
        IsXdr: bool,
        ignoreSchema: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dataset, IsXdr, ignoreSchema))?;
        Ok(__cordl_object)
    }
    pub fn New_DataTable_XmlElement__cordl_bool3(
        datatable: *mut crate::System::Data::DataTable,
        IsXdr: bool,
        topNode: *mut crate::System::Xml::XmlElement,
        ignoreSchema: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (datatable, IsXdr, topNode, ignoreSchema))?;
        Ok(__cordl_object)
    }
    pub fn New_DataTable__cordl_bool2(
        datatable: *mut crate::System::Data::DataTable,
        IsXdr: bool,
        ignoreSchema: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (datatable, IsXdr, ignoreSchema))?;
        Ok(__cordl_object)
    }
    pub fn ProcessXsdSchema(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ProcessXsdSchema", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetRowValueFromXmlText(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        col: *mut crate::System::Data::DataColumn,
        xmlText: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRowValueFromXmlText", (row, col, xmlText))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DataSet_XmlElement__cordl_bool1(
        &mut self,
        dataset: *mut crate::System::Data::DataSet,
        IsXdr: bool,
        topNode: *mut crate::System::Xml::XmlElement,
        ignoreSchema: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dataset, IsXdr, topNode, ignoreSchema))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DataSet__cordl_bool0(
        &mut self,
        dataset: *mut crate::System::Data::DataSet,
        IsXdr: bool,
        ignoreSchema: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dataset, IsXdr, ignoreSchema))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DataTable_XmlElement__cordl_bool3(
        &mut self,
        datatable: *mut crate::System::Data::DataTable,
        IsXdr: bool,
        topNode: *mut crate::System::Xml::XmlElement,
        ignoreSchema: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (datatable, IsXdr, topNode, ignoreSchema))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DataTable__cordl_bool2(
        &mut self,
        datatable: *mut crate::System::Data::DataTable,
        IsXdr: bool,
        ignoreSchema: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (datatable, IsXdr, ignoreSchema))?;
        Ok(__cordl_ret)
    }
    pub fn get_FromInference(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_FromInference", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_FromInference(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_FromInference", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+XmlDataLoader")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::XmlDataLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
