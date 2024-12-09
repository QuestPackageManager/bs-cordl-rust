#[cfg(feature = "Mono+RuntimeMarshal")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeMarshal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+RuntimeMarshal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::RuntimeMarshal => "Mono"."RuntimeMarshal"
);
#[cfg(feature = "Mono+RuntimeMarshal")]
impl std::ops::Deref for crate::Mono::RuntimeMarshal {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+RuntimeMarshal")]
impl std::ops::DerefMut for crate::Mono::RuntimeMarshal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+RuntimeMarshal")]
impl crate::Mono::RuntimeMarshal {}
#[cfg(feature = "Mono+RuntimeMarshal")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::RuntimeMarshal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
