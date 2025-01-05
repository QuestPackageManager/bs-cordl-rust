#[cfg(feature = "GetXPlatformAccessTokenRequestSony")]
#[repr(C)]
#[derive(Debug)]
pub struct GetXPlatformAccessTokenRequestSony {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub platformToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub platformEnvironment: crate::GlobalNamespace::PlatformEnvironment,
}
#[cfg(feature = "GetXPlatformAccessTokenRequestSony")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GetXPlatformAccessTokenRequestSony => ""
    ."GetXPlatformAccessTokenRequestSony"
);
#[cfg(feature = "GetXPlatformAccessTokenRequestSony")]
impl std::ops::Deref for crate::GlobalNamespace::GetXPlatformAccessTokenRequestSony {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GetXPlatformAccessTokenRequestSony")]
impl std::ops::DerefMut for crate::GlobalNamespace::GetXPlatformAccessTokenRequestSony {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GetXPlatformAccessTokenRequestSony")]
impl crate::GlobalNamespace::GetXPlatformAccessTokenRequestSony {
    pub fn New(
        platformToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        platformEnvironment: crate::GlobalNamespace::PlatformEnvironment,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (platformToken, platformEnvironment))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        platformToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        platformEnvironment: crate::GlobalNamespace::PlatformEnvironment,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (platformToken, platformEnvironment))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GetXPlatformAccessTokenRequestSony")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GetXPlatformAccessTokenRequestSony {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
