#[cfg(feature = "BGLib+JsonExtension+JsonSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGLib+JsonExtension+JsonSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::JsonExtension::JsonSettings =>
    "BGLib.JsonExtension"."JsonSettings"
);
#[cfg(feature = "BGLib+JsonExtension+JsonSettings")]
impl std::ops::Deref for crate::BGLib::JsonExtension::JsonSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+JsonExtension+JsonSettings")]
impl std::ops::DerefMut for crate::BGLib::JsonExtension::JsonSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+JsonExtension+JsonSettings")]
impl crate::BGLib::JsonExtension::JsonSettings {}
#[cfg(feature = "BGLib+JsonExtension+JsonSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::BGLib::JsonExtension::JsonSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
