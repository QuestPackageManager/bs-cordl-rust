#[cfg(feature = "Newtonsoft+Json+Utilities+JsonTokenUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonTokenUtils {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+JsonTokenUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::JsonTokenUtils =>
    "Newtonsoft.Json.Utilities"."JsonTokenUtils"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+JsonTokenUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::JsonTokenUtils {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+JsonTokenUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::JsonTokenUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+JsonTokenUtils")]
impl crate::Newtonsoft::Json::Utilities::JsonTokenUtils {}
#[cfg(feature = "Newtonsoft+Json+Utilities+JsonTokenUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::JsonTokenUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
