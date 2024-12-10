#[cfg(feature = "PlatformInstallerSO")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformInstallerSO {
    __cordl_parent: crate::Zenject::ScriptableObjectInstaller,
    pub _setupData: *mut crate::GlobalNamespace::AppInitSetupData,
    pub _ps4AchievementIdsModel: *mut crate::GlobalNamespace::SonyAchievementIdsModelSO,
    pub _ps5AchievementIdsModel: *mut crate::GlobalNamespace::SonyAchievementIdsModelSO,
    pub _achievementIdsModel: *mut crate::GlobalNamespace::AchievementIdsModelSO,
    pub _mockPlatformAdditionalContentModelInitialData: *mut crate::GlobalNamespace::MockPlatformAdditionalContentModelInitialDataSO,
}
#[cfg(feature = "PlatformInstallerSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlatformInstallerSO => ""
    ."PlatformInstallerSO"
);
#[cfg(feature = "PlatformInstallerSO")]
impl std::ops::Deref for crate::GlobalNamespace::PlatformInstallerSO {
    type Target = crate::Zenject::ScriptableObjectInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformInstallerSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlatformInstallerSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformInstallerSO")]
impl crate::GlobalNamespace::PlatformInstallerSO {
    pub fn InstallBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", ())?;
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
#[cfg(feature = "PlatformInstallerSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PlatformInstallerSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
