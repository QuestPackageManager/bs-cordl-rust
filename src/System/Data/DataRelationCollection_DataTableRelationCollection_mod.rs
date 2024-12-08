#[cfg(feature = "System+Data+DataRelationCollection+DataTableRelationCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct DataRelationCollection_DataTableRelationCollection {
    __cordl_parent: crate::System::Data::DataRelationCollection,
    pub _table: *mut crate::System::Data::DataTable,
    pub _relations: *mut crate::System::Collections::ArrayList,
    pub _fParentCollection: bool,
    pub RelationPropertyChanged: *mut crate::System::ComponentModel::CollectionChangeEventHandler,
}
#[cfg(feature = "System+Data+DataRelationCollection+DataTableRelationCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::DataRelationCollection_DataTableRelationCollection =>
    "System.Data"."DataRelationCollection/DataTableRelationCollection"
);
#[cfg(feature = "System+Data+DataRelationCollection+DataTableRelationCollection")]
impl std::ops::Deref
for crate::GlobalNamespace::DataRelationCollection_DataTableRelationCollection {
    type Target = crate::System::Data::DataRelationCollection;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataRelationCollection+DataTableRelationCollection")]
impl std::ops::DerefMut
for crate::GlobalNamespace::DataRelationCollection_DataTableRelationCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataRelationCollection+DataTableRelationCollection")]
impl crate::GlobalNamespace::DataRelationCollection_DataTableRelationCollection {
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
    pub fn add_RelationPropertyChanged(
        &mut self,
        value: *mut crate::System::ComponentModel::CollectionChangeEventHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_RelationPropertyChanged", (value))?;
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
    pub fn AddCache(
        &mut self,
        relation: *mut crate::System::Data::DataRelation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCache", (relation))?;
        Ok(__cordl_ret)
    }
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
    pub fn _ctor(
        &mut self,
        table: *mut crate::System::Data::DataTable,
        fParentCollection: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (table, fParentCollection))?;
        Ok(__cordl_ret)
    }
    pub fn remove_RelationPropertyChanged(
        &mut self,
        value: *mut crate::System::ComponentModel::CollectionChangeEventHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_RelationPropertyChanged", (value))?;
        Ok(__cordl_ret)
    }
    pub fn EnsureDataSet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureDataSet", ())?;
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
    pub fn RemoveCache(
        &mut self,
        relation: *mut crate::System::Data::DataRelation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveCache", (relation))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        table: *mut crate::System::Data::DataTable,
        fParentCollection: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (table, fParentCollection))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Data+DataRelationCollection+DataTableRelationCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DataRelationCollection_DataTableRelationCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
