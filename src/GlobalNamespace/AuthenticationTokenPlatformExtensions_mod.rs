#[cfg(feature = "AuthenticationTokenPlatformExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct AuthenticationTokenPlatformExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "AuthenticationTokenPlatformExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::AuthenticationTokenPlatformExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "AuthenticationTokenPlatformExtensions";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::AuthenticationTokenPlatformExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::GlobalNamespace::UserInfo_Platform),
                crate::GlobalNamespace::AuthenticationToken_Platform,
                1usize,
            >("ToAuthenticationTokenPlatform")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::AuthenticationTokenPlatformExtensions as
                    quest_hook::libil2cpp::Type > ::class(),
                    "ToAuthenticationTokenPlatform", 1usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::AuthenticationToken_Platform = unsafe {
            method.invoke_unchecked((), (platform))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToUserInfoPlatform(
        platform: crate::GlobalNamespace::AuthenticationToken_Platform,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::UserInfo_Platform> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::AuthenticationTokenPlatformExtensions as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::GlobalNamespace::AuthenticationToken_Platform),
                crate::GlobalNamespace::UserInfo_Platform,
                1usize,
            >("ToUserInfoPlatform")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::AuthenticationTokenPlatformExtensions as
                    quest_hook::libil2cpp::Type > ::class(), "ToUserInfoPlatform", 1usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::UserInfo_Platform = unsafe {
            method.invoke_unchecked((), (platform))?
        };
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
