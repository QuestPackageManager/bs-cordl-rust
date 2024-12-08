#[cfg(feature = "System+Xml+XmlConfiguration+XmlReaderSection")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlReaderSection {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Xml+XmlConfiguration+XmlReaderSection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlConfiguration::XmlReaderSection
    => "System.Xml.XmlConfiguration"."XmlReaderSection"
);
#[cfg(feature = "System+Xml+XmlConfiguration+XmlReaderSection")]
impl std::ops::Deref for crate::System::Xml::XmlConfiguration::XmlReaderSection {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlConfiguration+XmlReaderSection")]
impl std::ops::DerefMut for crate::System::Xml::XmlConfiguration::XmlReaderSection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlConfiguration+XmlReaderSection")]
impl crate::System::Xml::XmlConfiguration::XmlReaderSection {}
#[cfg(feature = "System+Xml+XmlConfiguration+XmlReaderSection")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::XmlConfiguration::XmlReaderSection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
