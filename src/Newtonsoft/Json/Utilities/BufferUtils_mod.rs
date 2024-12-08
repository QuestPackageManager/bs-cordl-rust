#[cfg(feature = "Newtonsoft+Json+Utilities+BufferUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct BufferUtils {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+BufferUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::BufferUtils =>
    "Newtonsoft.Json.Utilities"."BufferUtils"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+BufferUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::BufferUtils {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+BufferUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::BufferUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+BufferUtils")]
impl crate::Newtonsoft::Json::Utilities::BufferUtils {}
#[cfg(feature = "Newtonsoft+Json+Utilities+BufferUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::BufferUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}