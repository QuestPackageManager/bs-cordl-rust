#[cfg(feature = "System+Runtime+Serialization+EnumMemberAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct EnumMemberAttribute {
    __cordl_parent: crate::System::Attribute,
    pub value: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+Runtime+Serialization+EnumMemberAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::EnumMemberAttribute =>
    "System.Runtime.Serialization"."EnumMemberAttribute"
);
#[cfg(feature = "System+Runtime+Serialization+EnumMemberAttribute")]
impl std::ops::Deref for crate::System::Runtime::Serialization::EnumMemberAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+EnumMemberAttribute")]
impl std::ops::DerefMut for crate::System::Runtime::Serialization::EnumMemberAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+EnumMemberAttribute")]
impl crate::System::Runtime::Serialization::EnumMemberAttribute {
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Value", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Serialization+EnumMemberAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::EnumMemberAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
