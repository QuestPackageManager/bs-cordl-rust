#[cfg(feature = "System+Net+Configuration+BypassElement")]
#[repr(C)]
#[derive(Debug)]
pub struct BypassElement {
    __cordl_parent: crate::System::Configuration::ConfigurationElement,
}
#[cfg(feature = "System+Net+Configuration+BypassElement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Configuration::BypassElement =>
    "System.Net.Configuration"."BypassElement"
);
#[cfg(feature = "System+Net+Configuration+BypassElement")]
impl std::ops::Deref for crate::System::Net::Configuration::BypassElement {
    type Target = crate::System::Configuration::ConfigurationElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Configuration+BypassElement")]
impl std::ops::DerefMut for crate::System::Net::Configuration::BypassElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Configuration+BypassElement")]
impl crate::System::Net::Configuration::BypassElement {}
#[cfg(feature = "System+Net+Configuration+BypassElement")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Configuration::BypassElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
