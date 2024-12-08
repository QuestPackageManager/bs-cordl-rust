#[cfg(feature = "Oculus+Platform+UserAgeCategory")]
#[repr(C)]
#[derive(Debug)]
pub struct UserAgeCategory {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Oculus+Platform+UserAgeCategory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::UserAgeCategory =>
    "Oculus.Platform"."UserAgeCategory"
);
#[cfg(feature = "Oculus+Platform+UserAgeCategory")]
impl std::ops::Deref for crate::Oculus::Platform::UserAgeCategory {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+UserAgeCategory")]
impl std::ops::DerefMut for crate::Oculus::Platform::UserAgeCategory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+UserAgeCategory")]
impl crate::Oculus::Platform::UserAgeCategory {}
#[cfg(feature = "Oculus+Platform+UserAgeCategory")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::UserAgeCategory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
