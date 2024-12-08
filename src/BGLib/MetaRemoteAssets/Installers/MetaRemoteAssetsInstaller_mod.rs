#[cfg(feature = "BGLib+MetaRemoteAssets+Installers+MetaRemoteAssetsInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct MetaRemoteAssetsInstaller {
    __cordl_parent: crate::Zenject::ScriptableObjectInstaller,
    pub _appInitSetupData: *mut AppInitSetupData,
}
#[cfg(feature = "BGLib+MetaRemoteAssets+Installers+MetaRemoteAssetsInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::MetaRemoteAssets::Installers::MetaRemoteAssetsInstaller =>
    "BGLib.MetaRemoteAssets.Installers"."MetaRemoteAssetsInstaller"
);
#[cfg(feature = "BGLib+MetaRemoteAssets+Installers+MetaRemoteAssetsInstaller")]
impl std::ops::Deref
for crate::BGLib::MetaRemoteAssets::Installers::MetaRemoteAssetsInstaller {
    type Target = crate::Zenject::ScriptableObjectInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+Installers+MetaRemoteAssetsInstaller")]
impl std::ops::DerefMut
for crate::BGLib::MetaRemoteAssets::Installers::MetaRemoteAssetsInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+Installers+MetaRemoteAssetsInstaller")]
impl crate::BGLib::MetaRemoteAssets::Installers::MetaRemoteAssetsInstaller {
    pub fn InstallBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", ())?;
        Ok(__cordl_ret)
    }
    pub fn InstallRemoteCatalogLoader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallRemoteCatalogLoader", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+Installers+MetaRemoteAssetsInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::MetaRemoteAssets::Installers::MetaRemoteAssetsInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}