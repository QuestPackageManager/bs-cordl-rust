#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsRemoteCatalogLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct MetaRemoteAssetsRemoteCatalogLoader {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsRemoteCatalogLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::MetaRemoteAssets::MetaRemoteAssetsRemoteCatalogLoader =>
    "BGLib.MetaRemoteAssets"."MetaRemoteAssetsRemoteCatalogLoader"
);
#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsRemoteCatalogLoader")]
impl std::ops::Deref
for crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsRemoteCatalogLoader {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsRemoteCatalogLoader")]
impl std::ops::DerefMut
for crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsRemoteCatalogLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsRemoteCatalogLoader")]
impl crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsRemoteCatalogLoader {
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsRemoteCatalogLoader")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsRemoteCatalogLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsRemoteCatalogLoader")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::BGLib::MetaRemoteAssets::IRemoteCatalogLoader>,
> for crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsRemoteCatalogLoader {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::BGLib::MetaRemoteAssets::IRemoteCatalogLoader,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsRemoteCatalogLoader")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::BGLib::MetaRemoteAssets::IRemoteCatalogLoader>,
> for crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsRemoteCatalogLoader {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::BGLib::MetaRemoteAssets::IRemoteCatalogLoader,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
