#[cfg(feature = "System+Net+Configuration+ConnectionManagementElement")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectionManagementElement {
    __cordl_parent: crate::System::Configuration::ConfigurationElement,
}
#[cfg(feature = "System+Net+Configuration+ConnectionManagementElement")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::Configuration::ConnectionManagementElement {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net.Configuration";
    const CLASS_NAME: &'static str = "ConnectionManagementElement";
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
