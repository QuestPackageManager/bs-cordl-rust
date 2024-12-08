#[cfg(feature = "PlatformAuthenticationTokenProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformAuthenticationTokenProvider {
    __cordl_parent: crate::System::Object,
    pub _platformUserModel: *mut crate::GlobalNamespace::IPlatformUserModel,
    pub _userId: *mut crate::System::String,
    pub _userName: *mut crate::System::String,
    pub _hashedUserId: *mut crate::System::String,
    pub _platform: crate::GlobalNamespace::AuthenticationToken_Platform,
    pub _cachedXPlatformAccessToken: crate::GlobalNamespace::XPlatformAccessTokenData,
    pub _xPlatformAccessTokenTask: *mut crate::System::Threading::Tasks::Task_1<
        crate::GlobalNamespace::XPlatformAccessTokenData,
    >,
    pub _xPlatformTokenRetryDelayExpiration: f32,
}
#[cfg(feature = "PlatformAuthenticationTokenProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlatformAuthenticationTokenProvider => ""
    ."PlatformAuthenticationTokenProvider"
);
#[cfg(feature = "PlatformAuthenticationTokenProvider")]
impl std::ops::Deref for crate::GlobalNamespace::PlatformAuthenticationTokenProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformAuthenticationTokenProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlatformAuthenticationTokenProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformAuthenticationTokenProvider")]
impl crate::GlobalNamespace::PlatformAuthenticationTokenProvider {
    pub const kXPlatformTokenRetryDelayLengthSeconds: i32 = 30i32;
    #[cfg(feature = "PlatformAuthenticationTokenProvider+_GetAuthenticationToken_d__16")]
    pub type _GetAuthenticationToken_d__16 = crate::GlobalNamespace::PlatformAuthenticationTokenProvider__GetAuthenticationToken_d__16;
    #[cfg(
        feature = "PlatformAuthenticationTokenProvider+_GetXPlatformAccessToken_d__18"
    )]
    pub type _GetXPlatformAccessToken_d__18 = crate::GlobalNamespace::PlatformAuthenticationTokenProvider__GetXPlatformAccessToken_d__18;
    #[cfg(
        feature = "PlatformAuthenticationTokenProvider+_RequestXPlatformAccessToken_d__19"
    )]
    pub type _RequestXPlatformAccessToken_d__19 = crate::GlobalNamespace::PlatformAuthenticationTokenProvider__RequestXPlatformAccessToken_d__19;
    pub fn GetAuthenticationToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::AuthenticationToken,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::AuthenticationToken,
        > = __cordl_object.invoke("GetAuthenticationToken", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetTokenPlatform(
        &mut self,
        tokenPlatformEnvironment: crate::GlobalNamespace::PlatformEnvironment,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::AuthenticationToken_Platform,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::AuthenticationToken_Platform = __cordl_object
            .invoke("GetTokenPlatform", (tokenPlatformEnvironment))?;
        Ok(__cordl_ret)
    }
    pub fn GetXPlatformAccessToken(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
        skipCache: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::XPlatformAccessTokenData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::XPlatformAccessTokenData,
        > = __cordl_object
            .invoke("GetXPlatformAccessToken", (cancellationToken, skipCache))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        platformUserModel: *mut crate::GlobalNamespace::IPlatformUserModel,
        userInfo: *mut crate::GlobalNamespace::UserInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (platformUserModel, userInfo))?;
        Ok(__cordl_object)
    }
    pub fn RequestXPlatformAccessToken(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::XPlatformAccessTokenData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::XPlatformAccessTokenData,
        > = __cordl_object.invoke("RequestXPlatformAccessToken", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        platformUserModel: *mut crate::GlobalNamespace::IPlatformUserModel,
        userInfo: *mut crate::GlobalNamespace::UserInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (platformUserModel, userInfo))?;
        Ok(__cordl_ret)
    }
    pub fn get_hashedUserId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_hashedUserId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_platform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::AuthenticationToken_Platform,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::AuthenticationToken_Platform = __cordl_object
            .invoke("get_platform", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_userName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_userName", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PlatformAuthenticationTokenProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlatformAuthenticationTokenProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
