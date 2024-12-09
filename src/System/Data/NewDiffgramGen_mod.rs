#[cfg(feature = "System+Data+NewDiffgramGen")]
#[repr(C)]
#[derive(Debug)]
pub struct NewDiffgramGen {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _doc: *mut crate::System::Xml::XmlDocument,
    pub _ds: *mut crate::System::Data::DataSet,
    pub _dt: *mut crate::System::Data::DataTable,
    pub _xmlw: *mut crate::System::Xml::XmlWriter,
    pub _fBefore: bool,
    pub _fErrors: bool,
    pub _rowsOrder: *mut crate::System::Collections::Hashtable,
    pub _tables: *mut crate::System::Collections::ArrayList,
    pub _writeHierarchy: bool,
}
#[cfg(feature = "System+Data+NewDiffgramGen")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::NewDiffgramGen => "System.Data"
    ."NewDiffgramGen"
);
#[cfg(feature = "System+Data+NewDiffgramGen")]
impl std::ops::Deref for crate::System::Data::NewDiffgramGen {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+NewDiffgramGen")]
impl std::ops::DerefMut for crate::System::Data::NewDiffgramGen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+NewDiffgramGen")]
impl crate::System::Data::NewDiffgramGen {
    pub fn CreateTableHierarchy(
        &mut self,
        dt: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateTableHierarchy", (dt))?;
        Ok(__cordl_ret)
    }
    pub fn DoAssignments(
        &mut self,
        tables: *mut crate::System::Collections::ArrayList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoAssignments", (tables))?;
        Ok(__cordl_ret)
    }
    pub fn EmptyData(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EmptyData", ())?;
        Ok(__cordl_ret)
    }
    pub fn GenerateColumn(
        &mut self,
        row: *mut crate::System::Data::DataRow,
        col: *mut crate::System::Data::DataColumn,
        version: crate::System::Data::DataRowVersion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateColumn", (row, col, version))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateRow(
        &mut self,
        row: *mut crate::System::Data::DataRow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateRow", (row))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateTable(
        &mut self,
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateTable", (table))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateTableErrors(
        &mut self,
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateTableErrors", (table))?;
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
    pub fn Save_DataTable1(
        &mut self,
        xmlw: *mut crate::System::Xml::XmlWriter,
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Save", (xmlw, table))?;
        Ok(__cordl_ret)
    }
    pub fn Save_XmlWriter0(
        &mut self,
        xmlw: *mut crate::System::Xml::XmlWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Save", (xmlw))?;
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
#[cfg(feature = "System+Data+NewDiffgramGen")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::NewDiffgramGen {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
