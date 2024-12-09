#[cfg(feature = "Mono+Security+BitConverterLE")]
#[repr(C)]
#[derive(Debug)]
pub struct BitConverterLE {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Security+BitConverterLE")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::BitConverterLE =>
    "Mono.Security"."BitConverterLE"
);
#[cfg(feature = "Mono+Security+BitConverterLE")]
impl std::ops::Deref for crate::Mono::Security::BitConverterLE {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+BitConverterLE")]
impl std::ops::DerefMut for crate::Mono::Security::BitConverterLE {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+BitConverterLE")]
impl crate::Mono::Security::BitConverterLE {}
#[cfg(feature = "Mono+Security+BitConverterLE")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Security::BitConverterLE {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
