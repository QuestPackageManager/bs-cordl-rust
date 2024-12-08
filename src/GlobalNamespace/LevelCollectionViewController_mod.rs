#[cfg(feature = "LevelCollectionViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelCollectionViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _levelCollectionTableView: *mut LevelCollectionTableView,
    pub _noDataInfoContainer: *mut crate::UnityEngine::RectTransform,
    pub _playerDataModel: *mut PlayerDataModel,
    pub _container: *mut crate::Zenject::DiContainer,
    pub _additionalContentModel: *mut IAdditionalContentModel,
    pub _songPreviewPlayer: *mut SongPreviewPlayer,
    pub _perceivedLoudnessPerLevelModel: *mut PerceivedLoudnessPerLevelModel,
    pub _audioClipAsyncLoader: *mut AudioClipAsyncLoader,
    pub didSelectLevelEvent: *mut crate::System::Action_2<
        *mut LevelCollectionViewController,
        *mut BeatmapLevel,
    >,
    pub didSelectHeaderEvent: *mut crate::System::Action_1<
        *mut LevelCollectionViewController,
    >,
    pub _showHeader: bool,
    pub _noDataInfoGO: *mut crate::UnityEngine::GameObject,
    pub _beatmapLevelToBeSelected: *mut BeatmapLevel,
    pub _crossfadeCancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
    pub _unloadLevelDebouncer: *mut crate::BGLib::UnityExtension::Debouncer_1<
        *mut BeatmapLevel,
    >,
}
#[cfg(feature = "LevelCollectionViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LevelCollectionViewController => ""
    ."LevelCollectionViewController"
);
#[cfg(feature = "LevelCollectionViewController")]
impl std::ops::Deref for LevelCollectionViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelCollectionViewController")]
impl std::ops::DerefMut for LevelCollectionViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelCollectionViewController")]
impl LevelCollectionViewController {
    #[cfg(
        feature = "LevelCollectionViewController+_SongPlayerCrossfadeToLevelAsync_d__26"
    )]
    pub type _SongPlayerCrossfadeToLevelAsync_d__26 = crate::GlobalNamespace::LevelCollectionViewController__SongPlayerCrossfadeToLevelAsync_d__26;
    #[cfg(feature = "LevelCollectionViewController+_SongPlayerCrossfadeToLevel_d__25")]
    pub type _SongPlayerCrossfadeToLevel_d__25 = crate::GlobalNamespace::LevelCollectionViewController__SongPlayerCrossfadeToLevel_d__25;
    #[cfg(feature = "LevelCollectionViewController+__c__DisplayClass26_0")]
    pub type __c__DisplayClass26_0 = crate::GlobalNamespace::LevelCollectionViewController___c__DisplayClass26_0;
    pub fn DidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DidActivate",
                (firstActivation, addedToHierarchy, screenSystemEnabling),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DidDeactivate", (removedFromHierarchy, screenSystemDisabling))?;
        Ok(__cordl_ret)
    }
    pub fn HandleAdditionalContentModelDidInvalidateData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAdditionalContentModelDidInvalidateData", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLevelCollectionTableViewDidSelectLevel(
        &mut self,
        tableView: *mut LevelCollectionTableView,
        level: *mut BeatmapLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLevelCollectionTableViewDidSelectLevel", (tableView, level))?;
        Ok(__cordl_ret)
    }
    pub fn HandleLevelCollectionTableViewDidSelectPack(
        &mut self,
        tableView: *mut LevelCollectionTableView,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLevelCollectionTableViewDidSelectPack", (tableView))?;
        Ok(__cordl_ret)
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn RefreshFavorites(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshFavorites", ())?;
        Ok(__cordl_ret)
    }
    pub fn SelectLevel(
        &mut self,
        beatmapLevel: *mut BeatmapLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectLevel", (beatmapLevel))?;
        Ok(__cordl_ret)
    }
    pub fn SetData(
        &mut self,
        beatmapLevels: *mut quest_hook::libil2cpp::Il2CppArray<*mut BeatmapLevel>,
        headerText: *mut crate::System::String,
        headerSprite: *mut crate::UnityEngine::Sprite,
        sortLevels: bool,
        sortBeatmapLevels: bool,
        noDataInfoPrefab: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetData",
                (
                    beatmapLevels,
                    headerText,
                    headerSprite,
                    sortLevels,
                    sortBeatmapLevels,
                    noDataInfoPrefab,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SongPlayerCrossfadeToLevel(
        &mut self,
        level: *mut BeatmapLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SongPlayerCrossfadeToLevel", (level))?;
        Ok(__cordl_ret)
    }
    pub fn SongPlayerCrossfadeToLevelAsync(
        &mut self,
        level: *mut BeatmapLevel,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("SongPlayerCrossfadeToLevelAsync", (level, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn UnloadPreviewAudioClip(
        &mut self,
        level: *mut BeatmapLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnloadPreviewAudioClip", (level))?;
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
    pub fn add_didSelectHeaderEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut LevelCollectionViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectHeaderEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didSelectLevelEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut LevelCollectionViewController,
            *mut BeatmapLevel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectLevelEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didSelectHeaderEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut LevelCollectionViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectHeaderEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didSelectLevelEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut LevelCollectionViewController,
            *mut BeatmapLevel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectLevelEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LevelCollectionViewController")]
impl quest_hook::libil2cpp::ObjectType for LevelCollectionViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
