#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+NameInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct NameInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub NIFullName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub NIobjectId: i64,
    pub NIassemId: i64,
    pub NIprimitiveTypeEnum: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
    pub NItype: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub NIisSealed: bool,
    pub NIisArray: bool,
    pub NIisArrayItem: bool,
    pub NItransmitTypeOnObject: bool,
    pub NItransmitTypeOnMember: bool,
    pub NIisParentTypeOnObject: bool,
    pub NIarrayEnum: crate::System::Runtime::Serialization::Formatters::Binary::InternalArrayTypeE,
    pub NIsealedStatusChecked: bool,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+NameInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::NameInfo =>
    "System.Runtime.Serialization.Formatters.Binary"."NameInfo"
);
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+NameInfo")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::NameInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+NameInfo")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::NameInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+NameInfo")]
impl crate::System::Runtime::Serialization::Formatters::Binary::NameInfo {
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSealed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsSealed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NIname(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_NIname", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_NIname(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_NIname", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+NameInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::NameInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
