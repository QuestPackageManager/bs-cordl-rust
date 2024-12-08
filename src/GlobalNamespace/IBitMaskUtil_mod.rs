#[cfg(feature = "IBitMaskUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct IBitMaskUtil {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "IBitMaskUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IBitMaskUtil => ""."IBitMaskUtil"
);
#[cfg(feature = "IBitMaskUtil")]
impl std::ops::Deref for IBitMaskUtil {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IBitMaskUtil")]
impl std::ops::DerefMut for IBitMaskUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IBitMaskUtil")]
impl IBitMaskUtil {}
#[cfg(feature = "IBitMaskUtil")]
impl quest_hook::libil2cpp::ObjectType for IBitMaskUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}