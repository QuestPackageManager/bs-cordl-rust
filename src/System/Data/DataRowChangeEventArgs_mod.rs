#[cfg(feature = "System+Data+DataRowChangeEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct DataRowChangeEventArgs {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::EventArgs>,
    pub _Row_k__BackingField: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    pub _Action_k__BackingField: crate::System::Data::DataRowAction,
}
#[cfg(feature = "System+Data+DataRowChangeEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::DataRowChangeEventArgs =>
    "System.Data"."DataRowChangeEventArgs"
);
#[cfg(feature = "System+Data+DataRowChangeEventArgs")]
impl std::ops::Deref for crate::System::Data::DataRowChangeEventArgs {
    type Target = quest_hook::libil2cpp::Gc<crate::System::EventArgs>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataRowChangeEventArgs")]
impl std::ops::DerefMut for crate::System::Data::DataRowChangeEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataRowChangeEventArgs")]
impl crate::System::Data::DataRowChangeEventArgs {
    pub fn New(
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        action: crate::System::Data::DataRowAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (row, action))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        action: crate::System::Data::DataRowAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (row, action))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+DataRowChangeEventArgs")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::DataRowChangeEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
