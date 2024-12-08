#[cfg(feature = "AuthenticationTokenPlatformExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct AuthenticationTokenPlatformExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "AuthenticationTokenPlatformExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for AuthenticationTokenPlatformExtensions => ""
    ."AuthenticationTokenPlatformExtensions"
);
#[cfg(feature = "AuthenticationTokenPlatformExtensions")]
impl std::ops::Deref for AuthenticationTokenPlatformExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AuthenticationTokenPlatformExtensions")]
impl std::ops::DerefMut for AuthenticationTokenPlatformExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AuthenticationTokenPlatformExtensions")]
impl AuthenticationTokenPlatformExtensions {}
#[cfg(feature = "AuthenticationTokenPlatformExtensions")]
impl quest_hook::libil2cpp::ObjectType for AuthenticationTokenPlatformExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
