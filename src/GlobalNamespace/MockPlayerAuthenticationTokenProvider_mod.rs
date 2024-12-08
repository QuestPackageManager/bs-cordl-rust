#[cfg(feature = "MockPlayerAuthenticationTokenProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct MockPlayerAuthenticationTokenProvider {
    __cordl_parent: crate::System::Object,
    pub _userId: *mut crate::System::String,
    pub _password: *mut crate::System::String,
    pub _hashedUserId_k__BackingField: *mut crate::System::String,
    pub _userName_k__BackingField: *mut crate::System::String,
    pub _platform_k__BackingField: crate::GlobalNamespace::AuthenticationToken_Platform,
    pub _mockTokenData: XPlatformAccessTokenData,
}
#[cfg(feature = "MockPlayerAuthenticationTokenProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MockPlayerAuthenticationTokenProvider => ""
    ."MockPlayerAuthenticationTokenProvider"
);
#[cfg(feature = "MockPlayerAuthenticationTokenProvider")]
impl std::ops::Deref for MockPlayerAuthenticationTokenProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerAuthenticationTokenProvider")]
impl std::ops::DerefMut for MockPlayerAuthenticationTokenProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerAuthenticationTokenProvider")]
impl MockPlayerAuthenticationTokenProvider {
    pub fn GetAuthenticationToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<AuthenticationToken>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            AuthenticationToken,
        > = __cordl_object.invoke("GetAuthenticationToken", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetTokenPlatform(
        &mut self,
        tokenPlatformEnvironment: PlatformEnvironment,
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
        *mut crate::System::Threading::Tasks::Task_1<XPlatformAccessTokenData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            XPlatformAccessTokenData,
        > = __cordl_object
            .invoke("GetXPlatformAccessToken", (cancellationToken, skipCache))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        userId: *mut crate::System::String,
        userName: *mut crate::System::String,
        password: *mut crate::System::String,
        mockTokenData: XPlatformAccessTokenData,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (userId, userName, password, mockTokenData))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        userId: *mut crate::System::String,
        userName: *mut crate::System::String,
        password: *mut crate::System::String,
        mockTokenData: XPlatformAccessTokenData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (userId, userName, password, mockTokenData))?;
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
#[cfg(feature = "MockPlayerAuthenticationTokenProvider")]
impl quest_hook::libil2cpp::ObjectType for MockPlayerAuthenticationTokenProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
