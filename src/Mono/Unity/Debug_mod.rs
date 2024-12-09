#[cfg(feature = "Mono+Unity+Debug")]
#[repr(C)]
#[derive(Debug)]
pub struct Debug {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Unity+Debug")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Unity::Debug => "Mono.Unity"."Debug"
);
#[cfg(feature = "Mono+Unity+Debug")]
impl std::ops::Deref for crate::Mono::Unity::Debug {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+Debug")]
impl std::ops::DerefMut for crate::Mono::Unity::Debug {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+Debug")]
impl crate::Mono::Unity::Debug {}
#[cfg(feature = "Mono+Unity+Debug")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Unity::Debug {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
