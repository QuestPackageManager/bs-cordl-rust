#[cfg(feature = "OculusPlatformExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusPlatformExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OculusPlatformExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OculusPlatformExtensions => ""
    ."OculusPlatformExtensions"
);
#[cfg(feature = "OculusPlatformExtensions")]
impl std::ops::Deref for OculusPlatformExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusPlatformExtensions")]
impl std::ops::DerefMut for OculusPlatformExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusPlatformExtensions")]
impl OculusPlatformExtensions {}
#[cfg(feature = "OculusPlatformExtensions")]
impl quest_hook::libil2cpp::ObjectType for OculusPlatformExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
