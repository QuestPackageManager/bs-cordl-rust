#[cfg(feature = "Oculus+Platform+Notifications")]
#[repr(C)]
#[derive(Debug)]
pub struct Notifications {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Oculus+Platform+Notifications")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Notifications =>
    "Oculus.Platform"."Notifications"
);
#[cfg(feature = "Oculus+Platform+Notifications")]
impl std::ops::Deref for crate::Oculus::Platform::Notifications {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Notifications")]
impl std::ops::DerefMut for crate::Oculus::Platform::Notifications {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Notifications")]
impl crate::Oculus::Platform::Notifications {}
#[cfg(feature = "Oculus+Platform+Notifications")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Notifications {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
