#[cfg(feature = "BGNet+Core+IPlatformAccessTokenFetcher")]
#[repr(C)]
#[derive(Debug)]
pub struct IPlatformAccessTokenFetcher {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGNet+Core+IPlatformAccessTokenFetcher")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGNet::Core::IPlatformAccessTokenFetcher =>
    "BGNet.Core"."IPlatformAccessTokenFetcher"
);
#[cfg(feature = "BGNet+Core+IPlatformAccessTokenFetcher")]
impl std::ops::Deref for crate::BGNet::Core::IPlatformAccessTokenFetcher {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Core+IPlatformAccessTokenFetcher")]
impl std::ops::DerefMut for crate::BGNet::Core::IPlatformAccessTokenFetcher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Core+IPlatformAccessTokenFetcher")]
impl crate::BGNet::Core::IPlatformAccessTokenFetcher {
    pub fn GetXPlatformAccessToken(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
        skipCache: bool,
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
            .invoke("GetXPlatformAccessToken", (cancellationToken, skipCache))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "BGNet+Core+IPlatformAccessTokenFetcher")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGNet::Core::IPlatformAccessTokenFetcher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
