#[cfg(feature = "System+Data+XmlDataTreeWriter")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlDataTreeWriter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _xmlw: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
    pub _ds: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
    pub _dt: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    pub _dTables: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    pub _topLevelTables: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        >,
    >,
    pub _fFromTable: bool,
    pub _isDiffgram: bool,
    pub _rowsOrder: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
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
        dt: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateTablesHierarchy", (dt))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateToplevelTables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
            >,
        > = __cordl_object.invoke("CreateToplevelTables", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNestedChildRelations(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ArrayList,
        > = __cordl_object.invoke("GetNestedChildRelations", (row))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_DataSet0(
        ds: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ds))?;
        Ok(__cordl_object.into())
    }
    pub fn New_DataTable__cordl_bool1(
        dt: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        writeHierarchy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dt, writeHierarchy))?;
        Ok(__cordl_object.into())
    }
    pub fn PreserveSpace(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PreserveSpace", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn RowHasErrors(
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RowHasErrors", (row))?;
        Ok(__cordl_ret.into())
    }
    pub fn Save(
        &mut self,
        xw: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
        writeSchema: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Save", (xw, writeSchema))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveDiffgramData(
        &mut self,
        xw: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
        rowsOrder: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveDiffgramData", (xw, rowsOrder))?;
        Ok(__cordl_ret.into())
    }
    pub fn XmlDataRowWriter(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        encodedTableName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("XmlDataRowWriter", (row, encodedTableName))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DataSet0(
        &mut self,
        ds: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ds))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DataTable__cordl_bool1(
        &mut self,
        dt: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        writeHierarchy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dt, writeHierarchy))?;
        Ok(__cordl_ret.into())
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
