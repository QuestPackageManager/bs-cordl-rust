#[cfg(feature = "System+Configuration+ConfigurationElementCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct ConfigurationElementCollection {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::Configuration::ConfigurationElement,
    >,
}
#[cfg(feature = "System+Configuration+ConfigurationElementCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Configuration::ConfigurationElementCollection => "System.Configuration"
    ."ConfigurationElementCollection"
);
#[cfg(feature = "System+Configuration+ConfigurationElementCollection")]
impl std::ops::Deref for crate::System::Configuration::ConfigurationElementCollection {
    type Target = quest_hook::libil2cpp::Gc<
        crate::System::Configuration::ConfigurationElement,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Configuration+ConfigurationElementCollection")]
impl std::ops::DerefMut
for crate::System::Configuration::ConfigurationElementCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Configuration+ConfigurationElementCollection")]
impl crate::System::Configuration::ConfigurationElementCollection {}
#[cfg(feature = "System+Configuration+ConfigurationElementCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Configuration::ConfigurationElementCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
