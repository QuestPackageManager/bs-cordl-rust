#[cfg(feature = "System+Data+DataRowBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct DataRowBuilder {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    pub _record: i32,
}
#[cfg(feature = "System+Data+DataRowBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::DataRowBuilder => "System.Data"
    ."DataRowBuilder"
);
#[cfg(feature = "System+Data+DataRowBuilder")]
impl std::ops::Deref for crate::System::Data::DataRowBuilder {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataRowBuilder")]
impl std::ops::DerefMut for crate::System::Data::DataRowBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataRowBuilder")]
impl crate::System::Data::DataRowBuilder {
    pub fn New(
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        record: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (table, record))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        table: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
        record: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (table, record))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+DataRowBuilder")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::DataRowBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
