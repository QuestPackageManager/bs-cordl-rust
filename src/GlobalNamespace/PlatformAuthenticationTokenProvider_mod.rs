#[cfg(feature = "PlatformAuthenticationTokenProvider")]
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
#[cfg(feature = "PlatformAuthenticationTokenProvider")]
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
#[cfg(feature = "PlatformAuthenticationTokenProvider")]
impl std::ops::Deref for crate::GlobalNamespace::PlatformAuthenticationTokenProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn GetAuthenticationToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::AuthenticationToken,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::AuthenticationToken,
            >,
        > = __cordl_object.invoke("GetAuthenticationToken", ())?;
        Ok(__cordl_ret.into())
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::XPlatformAccessTokenData,
            >,
        > = __cordl_object
            .invoke("GetXPlatformAccessToken", (cancellationToken, skipCacheRead))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::XPlatformAccessTokenData,
            >,
        > = __cordl_object.invoke("RequestXPlatformAccessToken", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        platformUserModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPlatformUserModel,
        >,
        userInfo: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::UserInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (platformUserModel, userInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hashedUserId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_hashedUserId", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_userName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_userName", ())?;
        Ok(__cordl_ret.into())
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
