#[cfg(feature = "System+Configuration+ConfigurationSectionGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct ConfigurationSectionGroup {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Configuration+ConfigurationSectionGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Configuration::ConfigurationSectionGroup
    => "System.Configuration"."ConfigurationSectionGroup"
);
#[cfg(feature = "System+Configuration+ConfigurationSectionGroup")]
impl std::ops::Deref for crate::System::Configuration::ConfigurationSectionGroup {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Configuration+ConfigurationSectionGroup")]
impl std::ops::DerefMut for crate::System::Configuration::ConfigurationSectionGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Configuration+ConfigurationSectionGroup")]
impl crate::System::Configuration::ConfigurationSectionGroup {}
#[cfg(feature = "System+Configuration+ConfigurationSectionGroup")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Configuration::ConfigurationSectionGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
