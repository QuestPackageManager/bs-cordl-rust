#[cfg(feature = "IBitMaskUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct IBitMaskUtil {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IBitMaskUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IBitMaskUtil => ""
    ."IBitMaskUtil"
);
#[cfg(feature = "IBitMaskUtil")]
impl std::ops::Deref for crate::GlobalNamespace::IBitMaskUtil {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IBitMaskUtil")]
impl std::ops::DerefMut for crate::GlobalNamespace::IBitMaskUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IBitMaskUtil")]
impl crate::GlobalNamespace::IBitMaskUtil {}
#[cfg(feature = "IBitMaskUtil")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IBitMaskUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
