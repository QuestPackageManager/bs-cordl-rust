#[cfg(feature = "Oculus+Platform+Achievements")]
#[repr(C)]
#[derive(Debug)]
pub struct Achievements {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+Achievements")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Achievements =>
    "Oculus.Platform"."Achievements"
);
#[cfg(feature = "Oculus+Platform+Achievements")]
impl std::ops::Deref for crate::Oculus::Platform::Achievements {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Achievements")]
impl std::ops::DerefMut for crate::Oculus::Platform::Achievements {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Achievements")]
impl crate::Oculus::Platform::Achievements {}
#[cfg(feature = "Oculus+Platform+Achievements")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Achievements {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
