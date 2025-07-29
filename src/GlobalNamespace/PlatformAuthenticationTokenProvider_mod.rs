#[cfg(feature = "cordl_class_PlatformAuthenticationTokenProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformAuthenticationTokenProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _platformUserModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IPlatformUserModel,
    >,
    pub _userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _userName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _hashedUserId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _platform: crate::GlobalNamespace::AuthenticationToken_Platform,
    pub _cachedXPlatformAccessToken: crate::GlobalNamespace::XPlatformAccessTokenData,
    pub _xPlatformAccessTokenTask: quest_hook::libil2cpp::Gc<
        crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::XPlatformAccessTokenData,
        >,
    >,
    pub _xPlatformTokenRetryDelayExpiration: f32,
}
#[cfg(feature = "cordl_class_PlatformAuthenticationTokenProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlatformAuthenticationTokenProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlatformAuthenticationTokenProvider";
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
#[cfg(feature = "cordl_class_PlatformAuthenticationTokenProvider")]
impl std::ops::Deref for crate::GlobalNamespace::PlatformAuthenticationTokenProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_PlatformAuthenticationTokenProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlatformAuthenticationTokenProvider {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformAuthenticationTokenProvider")]
impl crate::GlobalNamespace::PlatformAuthenticationTokenProvider {
    pub const kXPlatformTokenRetryDelayLengthSeconds: i32 = 30i32;
    pub fn GetAuthenticationToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::AuthenticationToken,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<
                                crate::GlobalNamespace::AuthenticationToken,
                            >,
                        >,
                        0usize,
                    >("GetAuthenticationToken")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAuthenticationToken", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::AuthenticationToken,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTokenPlatform(
        &mut self,
        tokenPlatformEnvironment: crate::GlobalNamespace::PlatformEnvironment,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::AuthenticationToken_Platform,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::PlatformEnvironment),
                        crate::GlobalNamespace::AuthenticationToken_Platform,
                        1usize,
                    >("GetTokenPlatform")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTokenPlatform", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::AuthenticationToken_Platform = unsafe {
            cordl_method_info.invoke_unchecked(self, (tokenPlatformEnvironment))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetXPlatformAccessToken(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
        skipCacheRead: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::XPlatformAccessTokenData,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Threading::CancellationToken, bool),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<
                                crate::GlobalNamespace::XPlatformAccessTokenData,
                            >,
                        >,
                        2usize,
                    >("GetXPlatformAccessToken")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetXPlatformAccessToken", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::XPlatformAccessTokenData,
            >,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (cancellationToken, skipCacheRead))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        platformUserModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPlatformUserModel,
        >,
        userInfo: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::UserInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (platformUserModel, userInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn RequestXPlatformAccessToken(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::XPlatformAccessTokenData,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Threading::CancellationToken),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<
                                crate::GlobalNamespace::XPlatformAccessTokenData,
                            >,
                        >,
                        1usize,
                    >("RequestXPlatformAccessToken")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RequestXPlatformAccessToken", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::XPlatformAccessTokenData,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (cancellationToken))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        platformUserModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPlatformUserModel,
        >,
        userInfo: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::UserInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IPlatformUserModel,
                            >,
                            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::UserInfo>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (platformUserModel, userInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_hashedUserId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_hashedUserId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_hashedUserId", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_platform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::AuthenticationToken_Platform,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::AuthenticationToken_Platform,
                        0usize,
                    >("get_platform")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_platform", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::AuthenticationToken_Platform = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_userName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_userName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_userName", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_PlatformAuthenticationTokenProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlatformAuthenticationTokenProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlatformAuthenticationTokenProvider")]
impl AsRef<crate::BGNet::Core::IPlatformAccessTokenFetcher>
for crate::GlobalNamespace::PlatformAuthenticationTokenProvider {
    fn as_ref(&self) -> &crate::BGNet::Core::IPlatformAccessTokenFetcher {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PlatformAuthenticationTokenProvider")]
impl AsMut<crate::BGNet::Core::IPlatformAccessTokenFetcher>
for crate::GlobalNamespace::PlatformAuthenticationTokenProvider {
    fn as_mut(&mut self) -> &mut crate::BGNet::Core::IPlatformAccessTokenFetcher {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PlatformAuthenticationTokenProvider")]
impl AsRef<crate::GlobalNamespace::IAuthenticationTokenProvider>
for crate::GlobalNamespace::PlatformAuthenticationTokenProvider {
    fn as_ref(&self) -> &crate::GlobalNamespace::IAuthenticationTokenProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PlatformAuthenticationTokenProvider")]
impl AsMut<crate::GlobalNamespace::IAuthenticationTokenProvider>
for crate::GlobalNamespace::PlatformAuthenticationTokenProvider {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IAuthenticationTokenProvider {
        unsafe { std::mem::transmute(self) }
    }
}
