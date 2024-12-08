#[cfg(feature = "Newtonsoft+Json+Utilities+TypeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+TypeExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::TypeExtensions =>
    "Newtonsoft.Json.Utilities"."TypeExtensions"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+TypeExtensions")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::TypeExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+TypeExtensions")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::TypeExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+TypeExtensions")]
impl crate::Newtonsoft::Json::Utilities::TypeExtensions {}
#[cfg(feature = "Newtonsoft+Json+Utilities+TypeExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::TypeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
