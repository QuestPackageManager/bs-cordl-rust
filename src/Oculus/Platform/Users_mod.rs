#[cfg(feature = "Oculus+Platform+Users")]
#[repr(C)]
#[derive(Debug)]
pub struct Users {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Oculus+Platform+Users")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Users => "Oculus.Platform"
    ."Users"
);
#[cfg(feature = "Oculus+Platform+Users")]
impl std::ops::Deref for crate::Oculus::Platform::Users {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Users")]
impl std::ops::DerefMut for crate::Oculus::Platform::Users {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Users")]
impl crate::Oculus::Platform::Users {}
#[cfg(feature = "Oculus+Platform+Users")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Users {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
