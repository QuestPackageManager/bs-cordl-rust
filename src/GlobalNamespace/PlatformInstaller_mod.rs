#[cfg(feature = "PlatformInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformInstaller {
    __cordl_parent: crate::Zenject::Installer_3<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AppInitSetupData>,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MockPlatformAdditionalContentModelInitialDataSO,
        >,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlatformInstaller>,
    >,
    pub _isTest: bool,
    pub _mockPlatformAdditionalContentModelInitialData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MockPlatformAdditionalContentModelInitialDataSO,
    >,
}
#[cfg(feature = "PlatformInstaller")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::PlatformInstaller {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlatformInstaller";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "PlatformInstaller")]
impl std::ops::Deref for crate::GlobalNamespace::PlatformInstaller {
    type Target = crate::Zenject::Installer_3<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AppInitSetupData>,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MockPlatformAdditionalContentModelInitialDataSO,
        >,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlatformInstaller>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformInstaller")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlatformInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformInstaller")]
impl crate::GlobalNamespace::PlatformInstaller {
    pub fn BindAchievementsHandler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindAchievementsHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn BindAdditionalContentModel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindAdditionalContentModel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn BindAnalyticsModel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindAnalyticsModel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn BindBeatmapDataAssetFileModel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindBeatmapDataAssetFileModel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn BindMockPlatformAdditionalContentModel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindMockPlatformAdditionalContentModel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn BindPSPlusHandler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindPSPlusHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn BindPlatformUserModel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindPlatformUserModel", ())?;
        Ok(__cordl_ret.into())
    }
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
    pub fn New(
        appInitSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AppInitSetupData,
        >,
        mockPlatformAdditionalContentModelInitialData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MockPlatformAdditionalContentModelInitialDataSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (appInitSetupData, mockPlatformAdditionalContentModelInitialData),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        appInitSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AppInitSetupData,
        >,
        mockPlatformAdditionalContentModelInitialData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MockPlatformAdditionalContentModelInitialDataSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (appInitSetupData, mockPlatformAdditionalContentModelInitialData),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlatformInstaller")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PlatformInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
