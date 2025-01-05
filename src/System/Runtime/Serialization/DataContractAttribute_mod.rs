#[cfg(feature = "System+Runtime+Serialization+DataContractAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct DataContractAttribute {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Attribute>,
    pub isReference: bool,
}
#[cfg(feature = "System+Runtime+Serialization+DataContractAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::DataContractAttribute =>
    "System.Runtime.Serialization"."DataContractAttribute"
);
#[cfg(feature = "System+Runtime+Serialization+DataContractAttribute")]
impl std::ops::Deref for crate::System::Runtime::Serialization::DataContractAttribute {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Attribute>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+DataContractAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::DataContractAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+DataContractAttribute")]
impl crate::System::Runtime::Serialization::DataContractAttribute {
    pub fn get_IsReference(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsReference", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Serialization+DataContractAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::DataContractAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
