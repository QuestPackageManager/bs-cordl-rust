#[cfg(feature = "System+Xml+Serialization+XmlCustomFormatter")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlCustomFormatter {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Xml+Serialization+XmlCustomFormatter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Serialization::XmlCustomFormatter
    => "System.Xml.Serialization"."XmlCustomFormatter"
);
#[cfg(feature = "System+Xml+Serialization+XmlCustomFormatter")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlCustomFormatter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlCustomFormatter")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::XmlCustomFormatter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlCustomFormatter")]
impl crate::System::Xml::Serialization::XmlCustomFormatter {}
#[cfg(feature = "System+Xml+Serialization+XmlCustomFormatter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlCustomFormatter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}