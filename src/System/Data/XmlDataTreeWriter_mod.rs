#[cfg(feature = "System+Data+XmlDataTreeWriter")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlDataTreeWriter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _xmlw: *mut crate::System::Xml::XmlWriter,
    pub _ds: *mut crate::System::Data::DataSet,
    pub _dt: *mut crate::System::Data::DataTable,
    pub _dTables: *mut crate::System::Collections::ArrayList,
    pub _topLevelTables: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Data::DataTable,
    >,
    pub _fFromTable: bool,
    pub _isDiffgram: bool,
    pub _rowsOrder: *mut crate::System::Collections::Hashtable,
    pub _writeHierarchy: bool,
}
#[cfg(feature = "System+Data+XmlDataTreeWriter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::XmlDataTreeWriter => "System.Data"
    ."XmlDataTreeWriter"
);
#[cfg(feature = "System+Data+XmlDataTreeWriter")]
impl std::ops::Deref for crate::System::Data::XmlDataTreeWriter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XmlDataTreeWriter")]
impl std::ops::DerefMut for crate::System::Data::XmlDataTreeWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XmlDataTreeWriter")]
impl crate::System::Data::XmlDataTreeWriter {
    pub fn CreateTablesHierarchy(
        &mut self,
        dt: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateTablesHierarchy", (dt))?;
        Ok(__cordl_ret)
    }
    pub fn CreateToplevelTables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Data::DataTable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Data::DataTable,
        > = __cordl_object.invoke("CreateToplevelTables", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetNestedChildRelations(
        &mut self,
        row: *mut crate::System::Data::DataRow,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ArrayList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ArrayList = __cordl_object
            .invoke("GetNestedChildRelations", (row))?;
        Ok(__cordl_ret)
    }
    pub fn New_DataSet0(
        ds: *mut crate::System::Data::DataSet,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ds))?;
        Ok(__cordl_object)
    }
    pub fn New_DataTable__cordl_bool1(
        dt: *mut crate::System::Data::DataTable,
        writeHierarchy: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dt, writeHierarchy))?;
        Ok(__cordl_object)
    }
    pub fn Save(
        &mut self,
        xw: *mut crate::System::Xml::XmlWriter,
        writeSchema: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Save", (xw, writeSchema))?;
        Ok(__cordl_ret)
    }
    pub fn SaveDiffgramData(
        &mut self,
        xw: *mut crate::System::Xml::XmlWriter,
        rowsOrder: *mut crate::System::Collections::Hashtable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveDiffgramData", (xw, rowsOrder))?;
        Ok(__cordl_ret)
    }
    pub fn XmlDataRowWriter(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        encodedTableName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("XmlDataRowWriter", (row, encodedTableName))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DataSet0(
        &mut self,
        ds: *mut crate::System::Data::DataSet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ds))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DataTable__cordl_bool1(
        &mut self,
        dt: *mut crate::System::Data::DataTable,
        writeHierarchy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dt, writeHierarchy))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+XmlDataTreeWriter")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::XmlDataTreeWriter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
