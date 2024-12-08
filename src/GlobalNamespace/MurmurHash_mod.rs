#[cfg(feature = "MurmurHash")]
#[repr(C)]
#[derive(Debug)]
pub struct MurmurHash {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "MurmurHash")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MurmurHash => ""."MurmurHash"
);
#[cfg(feature = "MurmurHash")]
impl std::ops::Deref for crate::GlobalNamespace::MurmurHash {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MurmurHash")]
impl std::ops::DerefMut for crate::GlobalNamespace::MurmurHash {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MurmurHash")]
impl crate::GlobalNamespace::MurmurHash {}
#[cfg(feature = "MurmurHash")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MurmurHash {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
