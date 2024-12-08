#[cfg(feature = "Oculus+Platform+RichPresence")]
#[repr(C)]
#[derive(Debug)]
pub struct RichPresence {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Oculus+Platform+RichPresence")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::RichPresence =>
    "Oculus.Platform"."RichPresence"
);
#[cfg(feature = "Oculus+Platform+RichPresence")]
impl std::ops::Deref for crate::Oculus::Platform::RichPresence {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+RichPresence")]
impl std::ops::DerefMut for crate::Oculus::Platform::RichPresence {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+RichPresence")]
impl crate::Oculus::Platform::RichPresence {}
#[cfg(feature = "Oculus+Platform+RichPresence")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::RichPresence {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
