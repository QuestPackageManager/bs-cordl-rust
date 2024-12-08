#[cfg(feature = "System+Data+DataViewManagerListItemTypeDescriptor")]
#[repr(C)]
#[derive(Debug)]
pub struct DataViewManagerListItemTypeDescriptor {
    __cordl_parent: crate::System::Object,
    pub _dataViewManager: *mut crate::System::Data::DataViewManager,
}
#[cfg(feature = "System+Data+DataViewManagerListItemTypeDescriptor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Data::DataViewManagerListItemTypeDescriptor => "System.Data"
    ."DataViewManagerListItemTypeDescriptor"
);
#[cfg(feature = "System+Data+DataViewManagerListItemTypeDescriptor")]
impl std::ops::Deref for crate::System::Data::DataViewManagerListItemTypeDescriptor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataViewManagerListItemTypeDescriptor")]
impl std::ops::DerefMut for crate::System::Data::DataViewManagerListItemTypeDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataViewManagerListItemTypeDescriptor")]
impl crate::System::Data::DataViewManagerListItemTypeDescriptor {
    pub fn GetDataView(
        &mut self,
        table: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::DataView> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::DataView = __cordl_object
            .invoke("GetDataView", (table))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+DataViewManagerListItemTypeDescriptor")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Data::DataViewManagerListItemTypeDescriptor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
