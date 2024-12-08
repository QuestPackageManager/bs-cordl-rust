#[cfg(feature = "PerformanceToolLauncher+Assets")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PerformanceToolLauncher_Assets {
    pub visualizer: *mut PerformanceVisualizer,
    pub recorder: *mut PerformanceRecorder,
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
#[derive(Debug, Clone)]
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
#[cfg(feature = "PerformanceToolLauncher")]
#[repr(C)]
#[derive(Debug)]
pub struct PerformanceToolLauncher {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _assets: crate::GlobalNamespace::PerformanceToolLauncher_Assets,
}
#[cfg(feature = "PerformanceToolLauncher")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PerformanceToolLauncher => ""."PerformanceToolLauncher"
);
#[cfg(feature = "PerformanceToolLauncher")]
impl std::ops::Deref for PerformanceToolLauncher {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PerformanceToolLauncher")]
impl std::ops::DerefMut for PerformanceToolLauncher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PerformanceToolLauncher")]
impl PerformanceToolLauncher {
    #[cfg(feature = "PerformanceToolLauncher+Assets")]
    pub type Assets = crate::GlobalNamespace::PerformanceToolLauncher_Assets;
    #[cfg(feature = "PerformanceToolLauncher+__c__DisplayClass4_0")]
    pub type __c__DisplayClass4_0 = crate::GlobalNamespace::PerformanceToolLauncher___c__DisplayClass4_0;
    #[cfg(feature = "PerformanceToolLauncher+OverrideConfig")]
    pub type OverrideConfig = crate::GlobalNamespace::PerformanceToolLauncher_OverrideConfig;
    pub fn Initialize(
        &mut self,
        sceneContext: *mut crate::Zenject::SceneContext,
        mainSettingsHandler: *mut crate::BeatSaber::GameSettings::MainSettingsHandler,
        graphicSettingsHandler: *mut crate::BeatSaber::GameSettings::GraphicSettingsHandler,
        playerDataModel: *mut PlayerDataModel,
        mainCamera: *mut MainCamera,
        songController: *mut SongController,
        gamePause: *mut IGamePause,
        sceneSetupData: *mut GameplayCoreSceneSetupData,
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
                    sceneContext,
                    mainSettingsHandler,
                    graphicSettingsHandler,
                    playerDataModel,
                    mainCamera,
                    songController,
                    gamePause,
                    sceneSetupData,
                    overrideConfig,
                ),
            )?;
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
#[cfg(feature = "PerformanceToolLauncher")]
impl quest_hook::libil2cpp::ObjectType for PerformanceToolLauncher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
