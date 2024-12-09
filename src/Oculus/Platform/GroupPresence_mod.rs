#[cfg(feature = "Oculus+Platform+GroupPresence")]
#[repr(C)]
#[derive(Debug)]
pub struct GroupPresence {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+GroupPresence")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::GroupPresence =>
    "Oculus.Platform"."GroupPresence"
);
#[cfg(feature = "Oculus+Platform+GroupPresence")]
impl std::ops::Deref for crate::Oculus::Platform::GroupPresence {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+GroupPresence")]
impl std::ops::DerefMut for crate::Oculus::Platform::GroupPresence {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+GroupPresence")]
impl crate::Oculus::Platform::GroupPresence {}
#[cfg(feature = "Oculus+Platform+GroupPresence")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::GroupPresence {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
