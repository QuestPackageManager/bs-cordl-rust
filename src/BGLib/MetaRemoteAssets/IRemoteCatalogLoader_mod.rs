#[cfg(feature = "BGLib+MetaRemoteAssets+IRemoteCatalogLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct IRemoteCatalogLoader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGLib+MetaRemoteAssets+IRemoteCatalogLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::MetaRemoteAssets::IRemoteCatalogLoader =>
    "BGLib.MetaRemoteAssets"."IRemoteCatalogLoader"
);
#[cfg(feature = "BGLib+MetaRemoteAssets+IRemoteCatalogLoader")]
impl std::ops::Deref for crate::BGLib::MetaRemoteAssets::IRemoteCatalogLoader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+IRemoteCatalogLoader")]
impl std::ops::DerefMut for crate::BGLib::MetaRemoteAssets::IRemoteCatalogLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+IRemoteCatalogLoader")]
impl crate::BGLib::MetaRemoteAssets::IRemoteCatalogLoader {
    pub fn LoadRemoteCatalogAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<bool> = __cordl_object
            .invoke("LoadRemoteCatalogAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+IRemoteCatalogLoader")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::MetaRemoteAssets::IRemoteCatalogLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
