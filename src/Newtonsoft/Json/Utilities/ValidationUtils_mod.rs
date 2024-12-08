#[cfg(feature = "Newtonsoft+Json+Utilities+ValidationUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct ValidationUtils {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ValidationUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::ValidationUtils =>
    "Newtonsoft.Json.Utilities"."ValidationUtils"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+ValidationUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::ValidationUtils {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ValidationUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::ValidationUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ValidationUtils")]
impl crate::Newtonsoft::Json::Utilities::ValidationUtils {}
#[cfg(feature = "Newtonsoft+Json+Utilities+ValidationUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::ValidationUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}