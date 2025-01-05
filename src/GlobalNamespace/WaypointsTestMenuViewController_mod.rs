#[cfg(feature = "WaypointsTestMenuViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct WaypointsTestMenuViewController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _btsButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _cancelButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _progressText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _waypointsTestScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
    >,
    pub _previewLevels: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelSO>,
        >,
    >,
    pub _characteristics: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapCharacteristicSO>,
        >,
    >,
    pub _gameScenesManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameScenesManager,
    >,
    pub _playerDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerDataModel,
    >,
    pub _coroutineStarter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ICoroutineStarter,
    >,
    pub _environmentsListModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EnvironmentsListModel,
    >,
    pub _audioClipAsyncLoader: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AudioClipAsyncLoader,
    >,
    pub _beatmapDataLoader: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDataLoader,
    >,
    pub _settingsManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SettingsManager,
    >,
    pub _buttonBinder: quest_hook::libil2cpp::Gc<crate::HMUI::ButtonBinder>,
    pub _isCancelled: bool,
    pub _waitingForLevelFinish: bool,
}
#[cfg(feature = "WaypointsTestMenuViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::WaypointsTestMenuViewController
    => ""."WaypointsTestMenuViewController"
);
#[cfg(feature = "WaypointsTestMenuViewController")]
impl std::ops::Deref for crate::GlobalNamespace::WaypointsTestMenuViewController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "WaypointsTestMenuViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::WaypointsTestMenuViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "WaypointsTestMenuViewController")]
impl crate::GlobalNamespace::WaypointsTestMenuViewController {
    pub fn CheckBeatmaps(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("CheckBeatmaps", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMainGameSceneDidFinish(
        &mut self,
        data: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
        >,
        results: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMainGameSceneDidFinish", (data, results))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _HandleMainGameSceneDidFinish_b__19_0(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<HandleMainGameSceneDidFinish>b__19_0", (container))?;
        Ok(__cordl_ret.into())
    }
    pub fn _Start_b__16_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Start>b__16_0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _Start_b__16_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Start>b__16_1", ())?;
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
#[cfg(feature = "WaypointsTestMenuViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::WaypointsTestMenuViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
