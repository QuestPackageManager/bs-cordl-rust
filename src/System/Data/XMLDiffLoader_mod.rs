#[cfg(feature = "System+Data+XMLDiffLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct XMLDiffLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _tables: *mut crate::System::Collections::ArrayList,
    pub _dataSet: *mut crate::System::Data::DataSet,
    pub _dataTable: *mut crate::System::Data::DataTable,
}
#[cfg(feature = "System+Data+XMLDiffLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::XMLDiffLoader => "System.Data"
    ."XMLDiffLoader"
);
#[cfg(feature = "System+Data+XMLDiffLoader")]
impl std::ops::Deref for crate::System::Data::XMLDiffLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XMLDiffLoader")]
impl std::ops::DerefMut for crate::System::Data::XMLDiffLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+XMLDiffLoader")]
impl crate::System::Data::XMLDiffLoader {
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
    pub fn GetTable(
        &mut self,
        tableName: *mut quest_hook::libil2cpp::Il2CppString,
        ns: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataTable = __cordl_object
            .invoke("GetTable", (tableName, ns))?;
        Ok(__cordl_ret)
    }
    pub fn LoadDiffGram_DataSet0(
        &mut self,
        ds: *mut crate::System::Data::DataSet,
        dataTextReader: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadDiffGram", (ds, dataTextReader))?;
        Ok(__cordl_ret)
    }
    pub fn LoadDiffGram_DataTable1(
        &mut self,
        dt: *mut crate::System::Data::DataTable,
        dataTextReader: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadDiffGram", (dt, dataTextReader))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ProcessDiffs_ArrayList1(
        &mut self,
        tableList: *mut crate::System::Collections::ArrayList,
        ssync: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessDiffs", (tableList, ssync))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessDiffs_DataSet0(
        &mut self,
        ds: *mut crate::System::Data::DataSet,
        ssync: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessDiffs", (ds, ssync))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessErrors_ArrayList1(
        &mut self,
        dt: *mut crate::System::Collections::ArrayList,
        ssync: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessErrors", (dt, ssync))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessErrors_DataSet0(
        &mut self,
        ds: *mut crate::System::Data::DataSet,
        ssync: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessErrors", (ds, ssync))?;
        Ok(__cordl_ret)
    }
    pub fn ReadOldRowData(
        &mut self,
        ds: *mut crate::System::Data::DataSet,
        table: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Data::DataTable>,
        pos: quest_hook::libil2cpp::ByRefMut<i32>,
        row: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ReadOldRowData", (ds, table, pos, row))?;
        Ok(__cordl_ret)
    }
    pub fn SkipWhitespaces(
        &mut self,
        reader: *mut crate::System::Xml::XmlReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SkipWhitespaces", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+XMLDiffLoader")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::XMLDiffLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
