#[cfg(feature = "GetXPlatformAccessTokenRequestSteam")]
#[repr(C)]
#[derive(Debug)]
pub struct GetXPlatformAccessTokenRequestSteam {
    __cordl_parent: crate::System::Object,
    pub platformToken: *mut crate::System::String,
    pub platformUserId: i64,
}
#[cfg(feature = "GetXPlatformAccessTokenRequestSteam")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for GetXPlatformAccessTokenRequestSteam => ""
    ."GetXPlatformAccessTokenRequestSteam"
);
#[cfg(feature = "GetXPlatformAccessTokenRequestSteam")]
impl std::ops::Deref for GetXPlatformAccessTokenRequestSteam {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GetXPlatformAccessTokenRequestSteam")]
impl std::ops::DerefMut for GetXPlatformAccessTokenRequestSteam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GetXPlatformAccessTokenRequestSteam")]
impl GetXPlatformAccessTokenRequestSteam {
    pub fn _ctor(
        &mut self,
        platformToken: *mut crate::System::String,
        platformUserId: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (platformToken, platformUserId))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        platformToken: *mut crate::System::String,
        platformUserId: i64,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (platformToken, platformUserId))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "GetXPlatformAccessTokenRequestSteam")]
impl quest_hook::libil2cpp::ObjectType for GetXPlatformAccessTokenRequestSteam {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
