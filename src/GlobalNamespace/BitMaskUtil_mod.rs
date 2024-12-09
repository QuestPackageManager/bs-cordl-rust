#[cfg(feature = "BitMaskUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct BitMaskUtil {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BitMaskUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BitMaskUtil => ""."BitMaskUtil"
);
#[cfg(feature = "BitMaskUtil")]
impl std::ops::Deref for crate::GlobalNamespace::BitMaskUtil {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BitMaskUtil")]
impl std::ops::DerefMut for crate::GlobalNamespace::BitMaskUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BitMaskUtil")]
impl crate::GlobalNamespace::BitMaskUtil {}
#[cfg(feature = "BitMaskUtil")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BitMaskUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
