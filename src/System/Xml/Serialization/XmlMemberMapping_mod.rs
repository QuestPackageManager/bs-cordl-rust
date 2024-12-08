#[cfg(feature = "System+Xml+Serialization+XmlMemberMapping")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlMemberMapping {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Xml+Serialization+XmlMemberMapping")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Serialization::XmlMemberMapping =>
    "System.Xml.Serialization"."XmlMemberMapping"
);
#[cfg(feature = "System+Xml+Serialization+XmlMemberMapping")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlMemberMapping {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlMemberMapping")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::XmlMemberMapping {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlMemberMapping")]
impl crate::System::Xml::Serialization::XmlMemberMapping {}
#[cfg(feature = "System+Xml+Serialization+XmlMemberMapping")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlMemberMapping {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
