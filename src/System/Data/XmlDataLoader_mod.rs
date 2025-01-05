#[cfg(feature = "System+Data+XmlDataLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlDataLoader {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _dataSet: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
    pub _nodeToSchemaMap: quest_hook::libil2cpp::Gc<
        crate::System::Data::XmlToDatasetMap,
    >,
    pub _nodeToRowMap: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    pub _childRowsStack: quest_hook::libil2cpp::Gc<crate::System::Collections::Stack>,
    pub _htableExcludedNS: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Hashtable,
    >,
    pub _fIsXdr: bool,
    pub _isDiffgram: bool,
    pub _topMostNode: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
    pub _ignoreSchema: bool,
    pub _dataTable: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    pub _isTableLevel: bool,
    pub _fromInference: bool,
    pub _dataReader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    pub _XSD_XMLNS_NS: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _XDR_SCHEMA: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _XDRNS: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _SQL_SYNC: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _UPDGNS: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _XSD_SCHEMA: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _XSDNS: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _DFFNS: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _MSDNS: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _DIFFID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _HASCHANGES: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _ROWORDER: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Data+XmlDataLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::XmlDataLoader => "System.Data"
    ."XmlDataLoader"
);
#[cfg(feature = "System+Data+XmlDataLoader")]
impl std::ops::Deref for crate::System::Data::XmlDataLoader {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
        parentRow: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        parentElement: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AttachRows", (parentRow, parentElement))?;
        Ok(__cordl_ret.into())
    }
    pub fn CountNonNSAttributes(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CountNonNSAttributes", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn FColumnElement(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("FColumnElement", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn FExcludedNamespace(
        &mut self,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("FExcludedNamespace", (ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn FIgnoreNamespace_Gc0(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("FIgnoreNamespace", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn FIgnoreNamespace_Gc1(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("FIgnoreNamespace", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInitialTextFromNodes(
        &mut self,
        n: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetInitialTextFromNodes", (n))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRowFromElement(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow> = __cordl_object
            .invoke("GetRowFromElement", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTextOnlyColumn(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn> = __cordl_object
            .invoke("GetTextOnlyColumn", (row))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValueForTextOnlyColums(
        &mut self,
        n: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetValueForTextOnlyColums", (n))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitNameTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitNameTable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTextLikeNode(
        &mut self,
        n: crate::System::Xml::XmlNodeType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsTextLikeNode", (n))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTextOnly(
        &mut self,
        c: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsTextOnly", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadColumn(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        foundColumns: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadColumn", (column, foundColumns))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadData_Gc0(
        &mut self,
        xdoc: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadData", (xdoc))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadData_Gc1(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadData", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadRowData(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        rowElement: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadRowData", (row, rowElement))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadRows(
        &mut self,
        parentRow: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        parentElement: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadRows", (parentRow, parentElement))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadTable(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        isNested: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadTable", (table, isNested))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadTopMostTable(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadTopMostTable", (table))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Gc__cordl_bool1(
        dataset: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
        IsXdr: bool,
        topNode: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
        ignoreSchema: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dataset, IsXdr, topNode, ignoreSchema))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Gc__cordl_bool3(
        datatable: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        IsXdr: bool,
        topNode: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
        ignoreSchema: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (datatable, IsXdr, topNode, ignoreSchema))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool0(
        dataset: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
        IsXdr: bool,
        ignoreSchema: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dataset, IsXdr, ignoreSchema))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool2(
        datatable: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        IsXdr: bool,
        ignoreSchema: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (datatable, IsXdr, ignoreSchema))?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessXsdSchema(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ProcessXsdSchema", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRowValueFromXmlText(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        col: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        xmlText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRowValueFromXmlText", (row, col, xmlText))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Gc__cordl_bool1(
        &mut self,
        dataset: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
        IsXdr: bool,
        topNode: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
        ignoreSchema: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dataset, IsXdr, topNode, ignoreSchema))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Gc__cordl_bool3(
        &mut self,
        datatable: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        IsXdr: bool,
        topNode: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlElement>,
        ignoreSchema: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (datatable, IsXdr, topNode, ignoreSchema))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool0(
        &mut self,
        dataset: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
        IsXdr: bool,
        ignoreSchema: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dataset, IsXdr, ignoreSchema))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool2(
        &mut self,
        datatable: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        IsXdr: bool,
        ignoreSchema: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (datatable, IsXdr, ignoreSchema))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FromInference(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_FromInference", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
