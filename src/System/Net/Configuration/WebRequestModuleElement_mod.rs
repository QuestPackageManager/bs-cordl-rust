#[cfg(feature = "System+Net+Configuration+WebRequestModuleElement")]
#[repr(C)]
#[derive(Debug)]
pub struct WebRequestModuleElement {
    __cordl_parent: crate::System::Configuration::ConfigurationElement,
}
#[cfg(feature = "System+Net+Configuration+WebRequestModuleElement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::Configuration::WebRequestModuleElement => "System.Net.Configuration"
    ."WebRequestModuleElement"
);
#[cfg(feature = "System+Net+Configuration+WebRequestModuleElement")]
impl std::ops::Deref for crate::System::Net::Configuration::WebRequestModuleElement {
    type Target = crate::System::Configuration::ConfigurationElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Configuration+WebRequestModuleElement")]
impl std::ops::DerefMut for crate::System::Net::Configuration::WebRequestModuleElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Configuration+WebRequestModuleElement")]
impl crate::System::Net::Configuration::WebRequestModuleElement {}
#[cfg(feature = "System+Net+Configuration+WebRequestModuleElement")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Configuration::WebRequestModuleElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
