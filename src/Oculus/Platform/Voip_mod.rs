#[cfg(feature = "Oculus+Platform+Voip")]
#[repr(C)]
#[derive(Debug)]
pub struct Voip {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+Voip")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Voip => "Oculus.Platform"
    ."Voip"
);
#[cfg(feature = "Oculus+Platform+Voip")]
impl std::ops::Deref for crate::Oculus::Platform::Voip {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Voip")]
impl std::ops::DerefMut for crate::Oculus::Platform::Voip {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Voip")]
impl crate::Oculus::Platform::Voip {}
#[cfg(feature = "Oculus+Platform+Voip")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Voip {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
