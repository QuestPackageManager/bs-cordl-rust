#[cfg(feature = "Oculus+Platform+Entitlements")]
#[repr(C)]
#[derive(Debug)]
pub struct Entitlements {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Oculus+Platform+Entitlements")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Entitlements =>
    "Oculus.Platform"."Entitlements"
);
#[cfg(feature = "Oculus+Platform+Entitlements")]
impl std::ops::Deref for crate::Oculus::Platform::Entitlements {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Entitlements")]
impl std::ops::DerefMut for crate::Oculus::Platform::Entitlements {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Entitlements")]
impl crate::Oculus::Platform::Entitlements {}
#[cfg(feature = "Oculus+Platform+Entitlements")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Entitlements {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}