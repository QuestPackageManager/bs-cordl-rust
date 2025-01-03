#[cfg(feature = "AuthenticationTokenPlatformExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct AuthenticationTokenPlatformExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "AuthenticationTokenPlatformExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::AuthenticationTokenPlatformExtensions => ""
    ."AuthenticationTokenPlatformExtensions"
);
#[cfg(feature = "AuthenticationTokenPlatformExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::AuthenticationTokenPlatformExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AuthenticationTokenPlatformExtensions")]
impl std::ops::DerefMut
for crate::GlobalNamespace::AuthenticationTokenPlatformExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AuthenticationTokenPlatformExtensions")]
impl crate::GlobalNamespace::AuthenticationTokenPlatformExtensions {
    pub fn ToAuthenticationTokenPlatform(
        platform: crate::GlobalNamespace::UserInfo_Platform,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::AuthenticationToken_Platform,
    > {
        let __cordl_ret: crate::GlobalNamespace::AuthenticationToken_Platform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToAuthenticationTokenPlatform", (platform))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUserInfoPlatform(
        platform: crate::GlobalNamespace::AuthenticationToken_Platform,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::UserInfo_Platform> {
        let __cordl_ret: crate::GlobalNamespace::UserInfo_Platform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUserInfoPlatform", (platform))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AuthenticationTokenPlatformExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AuthenticationTokenPlatformExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
