#[cfg(feature = "PlatformInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformInstaller {
    __cordl_parent: crate::Zenject::Installer_3<
        *mut AppInitSetupData,
        *mut MockPlatformAdditionalContentModelInitialDataSO,
        *mut PlatformInstaller,
    >,
    pub _isTest: bool,
    pub _mockPlatformAdditionalContentModelInitialData: *mut MockPlatformAdditionalContentModelInitialDataSO,
}
#[cfg(feature = "PlatformInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PlatformInstaller => ""."PlatformInstaller"
);
#[cfg(feature = "PlatformInstaller")]
impl std::ops::Deref for PlatformInstaller {
    type Target = crate::Zenject::Installer_3<
        *mut AppInitSetupData,
        *mut MockPlatformAdditionalContentModelInitialDataSO,
        *mut PlatformInstaller,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformInstaller")]
impl std::ops::DerefMut for PlatformInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformInstaller")]
impl PlatformInstaller {
    pub fn BindMockPlatformAdditionalContentModel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindMockPlatformAdditionalContentModel", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindAdditionalContentModel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindAdditionalContentModel", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindAchievementsHandler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindAchievementsHandler", ())?;
        Ok(__cordl_ret)
    }
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
    pub fn _ctor(
        &mut self,
        appInitSetupData: *mut AppInitSetupData,
        mockPlatformAdditionalContentModelInitialData: *mut MockPlatformAdditionalContentModelInitialDataSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (appInitSetupData, mockPlatformAdditionalContentModelInitialData),
            )?;
        Ok(__cordl_ret)
    }
    pub fn BindBeatmapDataAssetFileModel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindBeatmapDataAssetFileModel", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindAnalyticsModel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindAnalyticsModel", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindPlatformUserModel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindPlatformUserModel", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        appInitSetupData: *mut AppInitSetupData,
        mockPlatformAdditionalContentModelInitialData: *mut MockPlatformAdditionalContentModelInitialDataSO,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (appInitSetupData, mockPlatformAdditionalContentModelInitialData),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PlatformInstaller")]
impl quest_hook::libil2cpp::ObjectType for PlatformInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
