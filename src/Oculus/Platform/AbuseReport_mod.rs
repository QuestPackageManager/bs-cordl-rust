#[cfg(feature = "Oculus+Platform+AbuseReport")]
#[repr(C)]
#[derive(Debug)]
pub struct AbuseReport {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+AbuseReport")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::AbuseReport =>
    "Oculus.Platform"."AbuseReport"
);
#[cfg(feature = "Oculus+Platform+AbuseReport")]
impl std::ops::Deref for crate::Oculus::Platform::AbuseReport {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+AbuseReport")]
impl std::ops::DerefMut for crate::Oculus::Platform::AbuseReport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+AbuseReport")]
impl crate::Oculus::Platform::AbuseReport {}
#[cfg(feature = "Oculus+Platform+AbuseReport")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::AbuseReport {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
