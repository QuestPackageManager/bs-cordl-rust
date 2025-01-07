#[cfg(feature = "System+Configuration+ConfigurationElementCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct ConfigurationElementCollection {
    __cordl_parent: crate::System::Configuration::ConfigurationElement,
}
#[cfg(feature = "System+Configuration+ConfigurationElementCollection")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Configuration::ConfigurationElementCollection {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Configuration";
    const CLASS_NAME: &'static str = "ConfigurationElementCollection";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Configuration+ConfigurationElementCollection")]
impl std::ops::Deref for crate::System::Configuration::ConfigurationElementCollection {
    type Target = crate::System::Configuration::ConfigurationElement;
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
