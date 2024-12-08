#[cfg(feature = "Oculus+Platform+Parties")]
#[repr(C)]
#[derive(Debug)]
pub struct Parties {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Oculus+Platform+Parties")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Parties => "Oculus.Platform"
    ."Parties"
);
#[cfg(feature = "Oculus+Platform+Parties")]
impl std::ops::Deref for crate::Oculus::Platform::Parties {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Parties")]
impl std::ops::DerefMut for crate::Oculus::Platform::Parties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Parties")]
impl crate::Oculus::Platform::Parties {}
#[cfg(feature = "Oculus+Platform+Parties")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Parties {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}