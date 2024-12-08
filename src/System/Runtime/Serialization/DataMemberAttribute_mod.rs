#[cfg(feature = "System+Runtime+Serialization+DataMemberAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct DataMemberAttribute {
    __cordl_parent: crate::System::Attribute,
    pub name: *mut crate::System::String,
    pub order: i32,
    pub isRequired: bool,
    pub emitDefaultValue: bool,
}
#[cfg(feature = "System+Runtime+Serialization+DataMemberAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::DataMemberAttribute =>
    "System.Runtime.Serialization"."DataMemberAttribute"
);
#[cfg(feature = "System+Runtime+Serialization+DataMemberAttribute")]
impl std::ops::Deref for crate::System::Runtime::Serialization::DataMemberAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+DataMemberAttribute")]
impl std::ops::DerefMut for crate::System::Runtime::Serialization::DataMemberAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+DataMemberAttribute")]
impl crate::System::Runtime::Serialization::DataMemberAttribute {
    pub fn get_EmitDefaultValue(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_EmitDefaultValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsRequired(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsRequired", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Order(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Order", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Serialization+DataMemberAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::DataMemberAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
