#[cfg(feature = "System+Net+Configuration+ConnectionManagementElement")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectionManagementElement {
    __cordl_parent: crate::System::Configuration::ConfigurationElement,
}
#[cfg(feature = "System+Net+Configuration+ConnectionManagementElement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::Configuration::ConnectionManagementElement =>
    "System.Net.Configuration"."ConnectionManagementElement"
);
#[cfg(feature = "System+Net+Configuration+ConnectionManagementElement")]
impl std::ops::Deref for crate::System::Net::Configuration::ConnectionManagementElement {
    type Target = crate::System::Configuration::ConfigurationElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Configuration+ConnectionManagementElement")]
impl std::ops::DerefMut
for crate::System::Net::Configuration::ConnectionManagementElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Configuration+ConnectionManagementElement")]
impl crate::System::Net::Configuration::ConnectionManagementElement {}
#[cfg(feature = "System+Net+Configuration+ConnectionManagementElement")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Configuration::ConnectionManagementElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
