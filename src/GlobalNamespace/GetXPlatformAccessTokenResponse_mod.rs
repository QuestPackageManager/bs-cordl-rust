#[cfg(feature = "GetXPlatformAccessTokenResponse")]
#[repr(C)]
#[derive(Debug)]
pub struct GetXPlatformAccessTokenResponse {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub accessToken: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "GetXPlatformAccessTokenResponse")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GetXPlatformAccessTokenResponse
    => ""."GetXPlatformAccessTokenResponse"
);
#[cfg(feature = "GetXPlatformAccessTokenResponse")]
impl std::ops::Deref for crate::GlobalNamespace::GetXPlatformAccessTokenResponse {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GetXPlatformAccessTokenResponse")]
impl std::ops::DerefMut for crate::GlobalNamespace::GetXPlatformAccessTokenResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GetXPlatformAccessTokenResponse")]
impl crate::GlobalNamespace::GetXPlatformAccessTokenResponse {
    pub fn New(
        accessToken: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (accessToken))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        accessToken: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (accessToken))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "GetXPlatformAccessTokenResponse")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GetXPlatformAccessTokenResponse {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
