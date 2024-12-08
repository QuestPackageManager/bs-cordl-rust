#[cfg(feature = "GetXPlatformAccessTokenRequestSony")]
#[repr(C)]
#[derive(Debug)]
pub struct GetXPlatformAccessTokenRequestSony {
    __cordl_parent: crate::System::Object,
    pub platformToken: *mut crate::System::String,
    pub platformEnvironment: PlatformEnvironment,
}
#[cfg(feature = "GetXPlatformAccessTokenRequestSony")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for GetXPlatformAccessTokenRequestSony => ""
    ."GetXPlatformAccessTokenRequestSony"
);
#[cfg(feature = "GetXPlatformAccessTokenRequestSony")]
impl std::ops::Deref for GetXPlatformAccessTokenRequestSony {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GetXPlatformAccessTokenRequestSony")]
impl std::ops::DerefMut for GetXPlatformAccessTokenRequestSony {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GetXPlatformAccessTokenRequestSony")]
impl GetXPlatformAccessTokenRequestSony {
    pub fn _ctor(
        &mut self,
        platformToken: *mut crate::System::String,
        platformEnvironment: PlatformEnvironment,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (platformToken, platformEnvironment))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        platformToken: *mut crate::System::String,
        platformEnvironment: PlatformEnvironment,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (platformToken, platformEnvironment))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "GetXPlatformAccessTokenRequestSony")]
impl quest_hook::libil2cpp::ObjectType for GetXPlatformAccessTokenRequestSony {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
