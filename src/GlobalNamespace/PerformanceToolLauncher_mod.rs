#[cfg(feature = "PerformanceToolLauncher")]
#[repr(C)]
#[derive(Debug)]
pub struct PerformanceToolLauncher {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _assets: crate::GlobalNamespace::PerformanceToolLauncher_Assets,
}
#[cfg(feature = "PerformanceToolLauncher")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PerformanceToolLauncher => ""
    ."PerformanceToolLauncher"
);
#[cfg(feature = "PerformanceToolLauncher")]
impl std::ops::Deref for crate::GlobalNamespace::PerformanceToolLauncher {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PerformanceToolLauncher")]
impl std::ops::DerefMut for crate::GlobalNamespace::PerformanceToolLauncher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PerformanceToolLauncher")]
impl crate::GlobalNamespace::PerformanceToolLauncher {
    #[cfg(feature = "PerformanceToolLauncher+Assets")]
    pub type Assets = crate::GlobalNamespace::PerformanceToolLauncher_Assets;
    #[cfg(feature = "PerformanceToolLauncher+OverrideConfig")]
    pub type OverrideConfig = crate::GlobalNamespace::PerformanceToolLauncher_OverrideConfig;
    pub fn Initialize(
        &mut self,
        settingsManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SettingsManager,
        >,
        playerDataModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerDataModel,
        >,
        mainCamera: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MainCamera>,
        recPlayState: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RecPlayBehaviour_State,
        >,
        songController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SongController,
        >,
        gamePause: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IGamePause>,
        sceneSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayCoreSceneSetupData,
        >,
        overrideConfig: crate::System::Nullable_1<
            crate::GlobalNamespace::PerformanceToolLauncher_OverrideConfig,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Initialize",
                (
                    settingsManager,
                    playerDataModel,
                    mainCamera,
                    recPlayState,
                    songController,
                    gamePause,
                    sceneSetupData,
                    overrideConfig,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Run(
        recorder: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PerformanceRecorder>,
        songController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SongController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Run", (recorder, songController))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "PerformanceToolLauncher")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PerformanceToolLauncher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PerformanceToolLauncher+Assets")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PerformanceToolLauncher_Assets {
    pub visualizer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PerformanceVisualizer,
    >,
    pub recorder: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PerformanceRecorder>,
}
#[cfg(feature = "PerformanceToolLauncher+Assets")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PerformanceToolLauncher_Assets
    => ""."PerformanceToolLauncher/Assets"
);
#[cfg(feature = "PerformanceToolLauncher+Assets")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PerformanceToolLauncher_Assets {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PerformanceToolLauncher+Assets")]
impl crate::GlobalNamespace::PerformanceToolLauncher_Assets {}
#[cfg(feature = "PerformanceToolLauncher+OverrideConfig")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PerformanceToolLauncher_OverrideConfig {
    pub enableAutoplay: bool,
    pub enableRecording: bool,
}
#[cfg(feature = "PerformanceToolLauncher+OverrideConfig")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PerformanceToolLauncher_OverrideConfig => ""
    ."PerformanceToolLauncher/OverrideConfig"
);
#[cfg(feature = "PerformanceToolLauncher+OverrideConfig")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PerformanceToolLauncher_OverrideConfig {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PerformanceToolLauncher+OverrideConfig")]
impl crate::GlobalNamespace::PerformanceToolLauncher_OverrideConfig {}
