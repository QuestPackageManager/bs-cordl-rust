#[cfg(feature = "System+Xml+XmlComplianceUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlComplianceUtil {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Xml+XmlComplianceUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlComplianceUtil => "System.Xml"
    ."XmlComplianceUtil"
);
#[cfg(feature = "System+Xml+XmlComplianceUtil")]
impl std::ops::Deref for crate::System::Xml::XmlComplianceUtil {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlComplianceUtil")]
impl std::ops::DerefMut for crate::System::Xml::XmlComplianceUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlComplianceUtil")]
impl crate::System::Xml::XmlComplianceUtil {}
#[cfg(feature = "System+Xml+XmlComplianceUtil")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlComplianceUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
