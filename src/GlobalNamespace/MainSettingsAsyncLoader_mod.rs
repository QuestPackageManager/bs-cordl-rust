#[cfg(feature = "MainSettingsAsyncLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct MainSettingsAsyncLoader {
    __cordl_parent: crate::BGLib::AppFlow::Initialization::AsyncInstaller,
    pub _networkConfig: *mut crate::GlobalNamespace::NetworkConfigSO,
    pub _hapticFeedbackControllerPrefab: *mut crate::GlobalNamespace::HapticFeedbackManager,
    pub _audioManager: *mut crate::GlobalNamespace::AudioManagerSO,
    pub _setupData: *mut crate::GlobalNamespace::AppInitSetupData,
    pub _mainSettingsHandler: *mut crate::BeatSaber::GameSettings::MainSettingsHandler,
    pub _graphicSettingsHandler: *mut crate::BeatSaber::GameSettings::GraphicSettingsHandler,
    pub _flushingService: *mut crate::BGLib::SaveDataCore::SaveDataFlushingService,
}
#[cfg(feature = "MainSettingsAsyncLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MainSettingsAsyncLoader => ""
    ."MainSettingsAsyncLoader"
);
#[cfg(feature = "MainSettingsAsyncLoader")]
impl std::ops::Deref for crate::GlobalNamespace::MainSettingsAsyncLoader {
    type Target = crate::BGLib::AppFlow::Initialization::AsyncInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MainSettingsAsyncLoader")]
impl std::ops::DerefMut for crate::GlobalNamespace::MainSettingsAsyncLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MainSettingsAsyncLoader")]
impl crate::GlobalNamespace::MainSettingsAsyncLoader {
    #[cfg(feature = "MainSettingsAsyncLoader+_LoadResourcesBeforeInstallAsync_d__10")]
    pub type _LoadResourcesBeforeInstallAsync_d__10 = crate::GlobalNamespace::MainSettingsAsyncLoader__LoadResourcesBeforeInstallAsync_d__10;
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
    pub fn InstallBindingsThatRelyOnSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindingsThatRelyOnSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadResourcesBeforeInstall(
        &mut self,
        registry: *mut crate::BGLib::AppFlow::Initialization::AsyncInstaller_IInstallerRegistry,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadResourcesBeforeInstall", (registry, container))?;
        Ok(__cordl_ret)
    }
    pub fn LoadResourcesBeforeInstallAsync(
        &mut self,
        registry: *mut crate::BGLib::AppFlow::Initialization::AsyncInstaller_IInstallerRegistry,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("LoadResourcesBeforeInstallAsync", (registry, container))?;
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
    pub fn get_isRunningFromTests(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isRunningFromTests", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MainSettingsAsyncLoader")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MainSettingsAsyncLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
