#[cfg(feature = "System+Xml+Serialization+XmlMembersMapping")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlMembersMapping {
    __cordl_parent: crate::System::Xml::Serialization::XmlMapping,
    pub _hasWrapperElement: bool,
    pub _mapping: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Xml::Serialization::XmlMemberMapping,
    >,
}
#[cfg(feature = "System+Xml+Serialization+XmlMembersMapping")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Serialization::XmlMembersMapping =>
    "System.Xml.Serialization"."XmlMembersMapping"
);
#[cfg(feature = "System+Xml+Serialization+XmlMembersMapping")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlMembersMapping {
    type Target = crate::System::Xml::Serialization::XmlMapping;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlMembersMapping")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::XmlMembersMapping {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlMembersMapping")]
impl crate::System::Xml::Serialization::XmlMembersMapping {
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasWrapperElement(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasWrapperElement", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlMembersMapping")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlMembersMapping {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
