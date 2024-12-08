#[cfg(feature = "System+Data+DataRelationCollection+DataSetRelationCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct DataRelationCollection_DataSetRelationCollection {
    __cordl_parent: crate::System::Data::DataRelationCollection,
    pub _dataSet: *mut crate::System::Data::DataSet,
    pub _relations: *mut crate::System::Collections::ArrayList,
    pub _delayLoadingRelations: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Data::DataRelation,
    >,
}
#[cfg(feature = "System+Data+DataRelationCollection+DataSetRelationCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::DataRelationCollection_DataSetRelationCollection => "System.Data"
    ."DataRelationCollection/DataSetRelationCollection"
);
#[cfg(feature = "System+Data+DataRelationCollection+DataSetRelationCollection")]
impl std::ops::Deref
for crate::GlobalNamespace::DataRelationCollection_DataSetRelationCollection {
    type Target = crate::System::Data::DataRelationCollection;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataRelationCollection+DataSetRelationCollection")]
impl std::ops::DerefMut
for crate::GlobalNamespace::DataRelationCollection_DataSetRelationCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataRelationCollection+DataSetRelationCollection")]
impl crate::GlobalNamespace::DataRelationCollection_DataSetRelationCollection {
    pub fn AddCore(
        &mut self,
        relation: *mut crate::System::Data::DataRelation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCore", (relation))?;
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
    pub fn GetDataSet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataSet> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataSet = __cordl_object
            .invoke("GetDataSet", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        dataSet: *mut crate::System::Data::DataSet,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dataSet))?;
        Ok(__cordl_object)
    }
    pub fn RemoveCore(
        &mut self,
        relation: *mut crate::System::Data::DataRelation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveCore", (relation))?;
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
    pub fn get_Item_String1(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataRelation> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRelation = __cordl_object
            .invoke("get_Item", (name))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_i32_0(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataRelation> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataRelation = __cordl_object
            .invoke("get_Item", (index))?;
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
}
#[cfg(feature = "System+Data+DataRelationCollection+DataSetRelationCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DataRelationCollection_DataSetRelationCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}