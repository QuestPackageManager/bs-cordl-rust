#[cfg(feature = "Newtonsoft+Json+JsonConvert")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonConvert {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Newtonsoft+Json+JsonConvert")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::JsonConvert =>
    "Newtonsoft.Json"."JsonConvert"
);
#[cfg(feature = "Newtonsoft+Json+JsonConvert")]
impl std::ops::Deref for crate::Newtonsoft::Json::JsonConvert {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonConvert")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::JsonConvert {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonConvert")]
impl crate::Newtonsoft::Json::JsonConvert {}
#[cfg(feature = "Newtonsoft+Json+JsonConvert")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::JsonConvert {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
