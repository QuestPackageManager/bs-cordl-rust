#[cfg(feature = "System+Data+DataColumnChangeEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct DataColumnChangeEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub _column: *mut crate::System::Data::DataColumn,
    pub _Row_k__BackingField: *mut crate::System::Data::DataRow,
    pub _ProposedValue_k__BackingField: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Data+DataColumnChangeEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::DataColumnChangeEventArgs =>
    "System.Data"."DataColumnChangeEventArgs"
);
#[cfg(feature = "System+Data+DataColumnChangeEventArgs")]
impl std::ops::Deref for crate::System::Data::DataColumnChangeEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataColumnChangeEventArgs")]
impl std::ops::DerefMut for crate::System::Data::DataColumnChangeEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+DataColumnChangeEventArgs")]
impl crate::System::Data::DataColumnChangeEventArgs {
    pub fn InitializeColumnChangeEvent(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeColumnChangeEvent", (column, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_DataColumn_Il2CppObject1(
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (row, column, value))?;
        Ok(__cordl_object.into())
    }
    pub fn New_DataRow0(
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (row))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_DataColumn_Il2CppObject1(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (row, column, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DataRow0(
        &mut self,
        row: quest_hook::libil2cpp::Gc<crate::System::Data::DataRow>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (row))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ProposedValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_ProposedValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ProposedValue(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ProposedValue", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+DataColumnChangeEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Data::DataColumnChangeEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
