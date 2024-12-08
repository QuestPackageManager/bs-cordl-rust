#[cfg(feature = "IAuthenticationTokenProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IAuthenticationTokenProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IAuthenticationTokenProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IAuthenticationTokenProvider => ""
    ."IAuthenticationTokenProvider"
);
#[cfg(feature = "IAuthenticationTokenProvider")]
impl std::ops::Deref for IAuthenticationTokenProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IAuthenticationTokenProvider")]
impl std::ops::DerefMut for IAuthenticationTokenProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IAuthenticationTokenProvider")]
impl IAuthenticationTokenProvider {
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
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IAuthenticationTokenProvider")]
impl quest_hook::libil2cpp::ObjectType for IAuthenticationTokenProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
