#[cfg(feature = "cordl_class_System+Net+Configuration+BypassElement")]
#[repr(C)]
#[derive(Debug)]
pub struct BypassElement {
    __cordl_parent: crate::System::Configuration::ConfigurationElement,
}
#[cfg(feature = "cordl_class_System+Net+Configuration+BypassElement")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::Configuration::BypassElement {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net.Configuration";
    const CLASS_NAME: &'static str = "BypassElement";
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
#[cfg(feature = "cordl_class_System+Net+Configuration+BypassElement")]
impl std::ops::Deref for crate::System::Net::Configuration::BypassElement {
    type Target = crate::System::Configuration::ConfigurationElement;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_System+Net+Configuration+BypassElement")]
impl std::ops::DerefMut for crate::System::Net::Configuration::BypassElement {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Configuration+BypassElement")]
impl crate::System::Net::Configuration::BypassElement {}
#[cfg(feature = "cordl_class_System+Net+Configuration+BypassElement")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Configuration::BypassElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
