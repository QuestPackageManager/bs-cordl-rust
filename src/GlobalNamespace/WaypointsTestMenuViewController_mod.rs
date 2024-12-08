#[cfg(feature = "WaypointsTestMenuViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct WaypointsTestMenuViewController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _btsButton: *mut crate::UnityEngine::UI::Button,
    pub _cancelButton: *mut crate::UnityEngine::UI::Button,
    pub _progressText: *mut crate::TMPro::TextMeshProUGUI,
    pub _waypointsTestScenesTransitionSetupData: *mut StandardLevelScenesTransitionSetupDataSO,
    pub _previewLevels: *mut crate::System::Collections::Generic::List_1<
        *mut BeatmapLevelSO,
    >,
    pub _characteristics: *mut crate::System::Collections::Generic::List_1<
        *mut BeatmapCharacteristicSO,
    >,
    pub _gameScenesManager: *mut GameScenesManager,
    pub _playerDataModel: *mut PlayerDataModel,
    pub _coroutineStarter: *mut ICoroutineStarter,
    pub _environmentsListModel: *mut EnvironmentsListModel,
    pub _audioClipAsyncLoader: *mut AudioClipAsyncLoader,
    pub _beatmapDataLoader: *mut BeatmapDataLoader,
    pub _graphicSettingsHandler: *mut crate::BeatSaber::GameSettings::GraphicSettingsHandler,
    pub _buttonBinder: *mut crate::HMUI::ButtonBinder,
    pub _isCancelled: bool,
    pub _waitingForLevelFinish: bool,
}
#[cfg(feature = "WaypointsTestMenuViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for WaypointsTestMenuViewController => ""
    ."WaypointsTestMenuViewController"
);
#[cfg(feature = "WaypointsTestMenuViewController")]
impl std::ops::Deref for WaypointsTestMenuViewController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "WaypointsTestMenuViewController")]
impl std::ops::DerefMut for WaypointsTestMenuViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "WaypointsTestMenuViewController")]
impl WaypointsTestMenuViewController {
    #[cfg(feature = "WaypointsTestMenuViewController+_CheckBeatmaps_d__18")]
    pub type _CheckBeatmaps_d__18 = crate::GlobalNamespace::WaypointsTestMenuViewController__CheckBeatmaps_d__18;
    pub fn CheckBeatmaps(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("CheckBeatmaps", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleMainGameSceneDidFinish(
        &mut self,
        data: *mut StandardLevelScenesTransitionSetupDataSO,
        results: *mut LevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMainGameSceneDidFinish", (data, results))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn _HandleMainGameSceneDidFinish_b__19_0(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<HandleMainGameSceneDidFinish>b__19_0", (container))?;
        Ok(__cordl_ret)
    }
    pub fn _Start_b__16_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Start>b__16_0", ())?;
        Ok(__cordl_ret)
    }
    pub fn _Start_b__16_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Start>b__16_1", ())?;
        Ok(__cordl_ret)
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
#[cfg(feature = "WaypointsTestMenuViewController")]
impl quest_hook::libil2cpp::ObjectType for WaypointsTestMenuViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
