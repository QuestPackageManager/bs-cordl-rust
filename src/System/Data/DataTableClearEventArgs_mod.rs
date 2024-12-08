#[cfg(feature = "System+Data+DataTableClearEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct DataTableClearEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub _Table_k__BackingField: *mut crate::System::Data::DataTable,
}
#[cfg(feature = "System+Data+DataTableClearEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::DataTableClearEventArgs =>
    "System.Data"."DataTableClearEventArgs"
);
#[cfg(feature = "System+Data+DataTableClearEventArgs")]
impl std::ops::Deref for crate::System::Data::DataTableClearEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataTableClearEventArgs")]
impl std::ops::DerefMut for crate::System::Data::DataTableClearEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataTableClearEventArgs")]
impl crate::System::Data::DataTableClearEventArgs {
    pub fn _ctor(
        &mut self,
        dataTable: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dataTable))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        dataTable: *mut crate::System::Data::DataTable,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dataTable))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Data+DataTableClearEventArgs")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::DataTableClearEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
