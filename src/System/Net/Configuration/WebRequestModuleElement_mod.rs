#[cfg(feature = "cordl_class_System+Net+Configuration+WebRequestModuleElement")]
#[repr(C)]
#[derive(Debug)]
pub struct WebRequestModuleElement {
    __cordl_parent: crate::System::Configuration::ConfigurationElement,
}
#[cfg(feature = "cordl_class_System+Net+Configuration+WebRequestModuleElement")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::System::Net::Configuration::WebRequestModuleElement
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net.Configuration";
    const CLASS_NAME: &'static str = "WebRequestModuleElement";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "System+Net+Configuration+WebRequestModuleElement")]
impl std::ops::Deref for crate::System::Net::Configuration::WebRequestModuleElement {
    type Target = crate::System::Configuration::ConfigurationElement;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Configuration+WebRequestModuleElement")]
impl std::ops::DerefMut for crate::System::Net::Configuration::WebRequestModuleElement {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Configuration+WebRequestModuleElement")]
impl crate::System::Net::Configuration::WebRequestModuleElement {}
#[cfg(feature = "cordl_class_System+Net+Configuration+WebRequestModuleElement")]
impl quest_hook::libil2cpp::ObjectType
    for crate::System::Net::Configuration::WebRequestModuleElement
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
