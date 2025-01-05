#[cfg(feature = "Mono+Runtime")]
#[repr(C)]
#[derive(Debug)]
pub struct Runtime {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Mono+Runtime")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Runtime => "Mono"."Runtime"
);
#[cfg(feature = "Mono+Runtime")]
impl std::ops::Deref for crate::Mono::Runtime {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Runtime")]
impl std::ops::DerefMut for crate::Mono::Runtime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Runtime")]
impl crate::Mono::Runtime {}
#[cfg(feature = "Mono+Runtime")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Runtime {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
