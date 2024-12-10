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
    pub fn AddCache(
        &mut self,
        relation: quest_hook::libil2cpp::Gc<crate::System::Data::DataRelation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCache", (relation))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddCore(
        &mut self,
        relation: quest_hook::libil2cpp::Gc<crate::System::Data::DataRelation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCore", (relation))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureDataSet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureDataSet", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDataSet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet> = __cordl_object
            .invoke("GetDataSet", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        fParentCollection: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (table, fParentCollection))?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveCache(
        &mut self,
        relation: quest_hook::libil2cpp::Gc<crate::System::Data::DataRelation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveCache", (relation))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveCore(
        &mut self,
        relation: quest_hook::libil2cpp::Gc<crate::System::Data::DataRelation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveCore", (relation))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        fParentCollection: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (table, fParentCollection))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_RelationPropertyChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::CollectionChangeEventHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_RelationPropertyChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item_Il2CppString1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRelation>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataRelation> = __cordl_object
            .invoke("get_Item", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item_i32_0(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::DataRelation>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::DataRelation> = __cordl_object
            .invoke("get_Item", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_List(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ArrayList,
        > = __cordl_object.invoke("get_List", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_RelationPropertyChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::CollectionChangeEventHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_RelationPropertyChanged", (value))?;
        Ok(__cordl_ret.into())
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
