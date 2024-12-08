#[cfg(feature = "System+Data+Merger")]
#[repr(C)]
#[derive(Debug)]
pub struct Merger {
    __cordl_parent: crate::System::Object,
    pub _dataSet: *mut crate::System::Data::DataSet,
    pub _dataTable: *mut crate::System::Data::DataTable,
    pub _preserveChanges: bool,
    pub _missingSchemaAction: crate::System::Data::MissingSchemaAction,
    pub _isStandAlonetable: bool,
    pub _IgnoreNSforTableLookup: bool,
}
#[cfg(feature = "System+Data+Merger")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::Merger => "System.Data"."Merger"
);
#[cfg(feature = "System+Data+Merger")]
impl std::ops::Deref for crate::System::Data::Merger {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Merger")]
impl std::ops::DerefMut for crate::System::Data::Merger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Merger")]
impl crate::System::Data::Merger {
    pub fn GetSrcKey(
        &mut self,
        src: *mut crate::System::Data::DataTable,
        dst: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::DataKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::DataKey = __cordl_object
            .invoke("GetSrcKey", (src, dst))?;
        Ok(__cordl_ret)
    }
    pub fn MergeRelation(
        &mut self,
        relation: *mut crate::System::Data::DataRelation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MergeRelation", (relation))?;
        Ok(__cordl_ret)
    }
    pub fn MergeConstraints_DataSet0(
        &mut self,
        source: *mut crate::System::Data::DataSet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MergeConstraints", (source))?;
        Ok(__cordl_ret)
    }
    pub fn MergeConstraints_DataTable1(
        &mut self,
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MergeConstraints", (table))?;
        Ok(__cordl_ret)
    }
    pub fn MergeTable_DataTable0(
        &mut self,
        src: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MergeTable", (src))?;
        Ok(__cordl_ret)
    }
    pub fn MergeTable_DataTable1(
        &mut self,
        src: *mut crate::System::Data::DataTable,
        dst: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MergeTable", (src, dst))?;
        Ok(__cordl_ret)
    }
    pub fn MergeSchema(
        &mut self,
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataTable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataTable = __cordl_object
            .invoke("MergeSchema", (table))?;
        Ok(__cordl_ret)
    }
    pub fn MergeDataSet(
        &mut self,
        source: *mut crate::System::Data::DataSet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MergeDataSet", (source))?;
        Ok(__cordl_ret)
    }
    pub fn MergeExtendedProperties(
        &mut self,
        src: *mut crate::System::Data::PropertyCollection,
        dst: *mut crate::System::Data::PropertyCollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MergeExtendedProperties", (src, dst))?;
        Ok(__cordl_ret)
    }
    pub fn MergeTableData(
        &mut self,
        src: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MergeTableData", (src))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DataSet0(
        &mut self,
        dataSet: *mut crate::System::Data::DataSet,
        preserveChanges: bool,
        missingSchemaAction: crate::System::Data::MissingSchemaAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dataSet, preserveChanges, missingSchemaAction))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DataTable1(
        &mut self,
        dataTable: *mut crate::System::Data::DataTable,
        preserveChanges: bool,
        missingSchemaAction: crate::System::Data::MissingSchemaAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dataTable, preserveChanges, missingSchemaAction))?;
        Ok(__cordl_ret)
    }
    pub fn New_DataSet0(
        dataSet: *mut crate::System::Data::DataSet,
        preserveChanges: bool,
        missingSchemaAction: crate::System::Data::MissingSchemaAction,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dataSet, preserveChanges, missingSchemaAction))?;
        Ok(__cordl_object)
    }
    pub fn New_DataTable1(
        dataTable: *mut crate::System::Data::DataTable,
        preserveChanges: bool,
        missingSchemaAction: crate::System::Data::MissingSchemaAction,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dataTable, preserveChanges, missingSchemaAction))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Data+Merger")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::Merger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
