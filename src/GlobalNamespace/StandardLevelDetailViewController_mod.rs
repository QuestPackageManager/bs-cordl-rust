#[cfg(feature = "StandardLevelDetailViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardLevelDetailViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _standardLevelDetailView: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::StandardLevelDetailView,
    >,
    pub _standardLevelBuyView: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::StandardLevelBuyView,
    >,
    pub _standardLevelBuyInfoView: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::StandardLevelBuyInfoView,
    >,
    pub _loadingControl: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LoadingControl,
    >,
    pub _noAllowedBeatmapInfoContainer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub _playerDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerDataModel,
    >,
    pub _additionalContentModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IAdditionalContentModel,
    >,
    pub _beatmapLevelsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsModel,
    >,
    pub _dlcPromoPanelModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DlcPromoPanelModel,
    >,
    pub _analyticsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IAnalyticsModel,
    >,
    pub _beatmapCharacteristicCollection: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCharacteristicCollection,
    >,
    pub didPressActionButtonEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            *mut crate::GlobalNamespace::StandardLevelDetailViewController,
        >,
    >,
    pub didPressOpenLevelPackButtonEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            *mut crate::GlobalNamespace::StandardLevelDetailViewController,
            *mut crate::GlobalNamespace::BeatmapLevelPack,
        >,
    >,
    pub levelFavoriteStatusDidChangeEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            *mut crate::GlobalNamespace::StandardLevelDetailViewController,
            bool,
        >,
    >,
    pub didPressPracticeButtonEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            *mut crate::GlobalNamespace::StandardLevelDetailViewController,
            *mut crate::GlobalNamespace::BeatmapLevel,
        >,
    >,
    pub didChangeDifficultyBeatmapEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            *mut crate::GlobalNamespace::StandardLevelDetailViewController,
        >,
    >,
    pub didChangeContentEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            *mut crate::GlobalNamespace::StandardLevelDetailViewController,
            crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
        >,
    >,
    pub _ownedObjectsEventBinder: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EventBinder,
    >,
    pub _eventBinder: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EventBinder>,
    pub _cancellationTokenSource: quest_hook::libil2cpp::Gc<
        crate::System::Threading::CancellationTokenSource,
    >,
    pub _beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    pub _pack: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
    pub _canBuyPack: bool,
    pub _allowedBeatmapDifficultyMask: crate::GlobalNamespace::BeatmapDifficultyMask,
    pub _notAllowedCharacteristics: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
    >,
    pub _contentIsOwnedAndReady: bool,
}
#[cfg(feature = "StandardLevelDetailViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::StandardLevelDetailViewController => ""
    ."StandardLevelDetailViewController"
);
#[cfg(feature = "StandardLevelDetailViewController")]
impl std::ops::Deref for crate::GlobalNamespace::StandardLevelDetailViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelDetailViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::StandardLevelDetailViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelDetailViewController")]
impl crate::GlobalNamespace::StandardLevelDetailViewController {
    pub const kLoadingDataErrorLocalizationKey: &'static str = "ERROR_LOADING_DATA";
    pub const kLoadingDataErrorNoInternetLocalizationKey: &'static str = "ERROR_LOADING_DATA_NO_INTERNET_MESSAGE";
    #[cfg(feature = "StandardLevelDetailViewController+ContentType")]
    pub type ContentType = crate::GlobalNamespace::StandardLevelDetailViewController_ContentType;
    #[cfg(
        feature = "StandardLevelDetailViewController+__BuyPackButtonWasPressed_b__55_0_d"
    )]
    pub type __BuyPackButtonWasPressed_b__55_0_d = crate::GlobalNamespace::StandardLevelDetailViewController___BuyPackButtonWasPressed_b__55_0_d;
    #[cfg(
        feature = "StandardLevelDetailViewController+__OpenLevelProductStoreOrShowBuyInfo_b__56_0_d"
    )]
    pub type __OpenLevelProductStoreOrShowBuyInfo_b__56_0_d = crate::GlobalNamespace::StandardLevelDetailViewController___OpenLevelProductStoreOrShowBuyInfo_b__56_0_d;
    #[cfg(
        feature = "StandardLevelDetailViewController+__OpenLevelProductStore_b__57_0_d"
    )]
    pub type __OpenLevelProductStore_b__57_0_d = crate::GlobalNamespace::StandardLevelDetailViewController___OpenLevelProductStore_b__57_0_d;
    #[cfg(
        feature = "StandardLevelDetailViewController+__RefreshAvailabilityIfNeeded_b__58_0_d"
    )]
    pub type __RefreshAvailabilityIfNeeded_b__58_0_d = crate::GlobalNamespace::StandardLevelDetailViewController___RefreshAvailabilityIfNeeded_b__58_0_d;
    pub fn BuyLevelButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuyLevelButtonWasPressed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn BuyPackButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuyPackButtonWasPressed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearSelected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearSelected", ())?;
        Ok(__cordl_ret.into())
    }
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleDidChangeDifficultyBeatmap(
        &mut self,
        view: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::StandardLevelDetailView>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDidChangeDifficultyBeatmap", (view))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleDidFavoriteToggleChange(
        &mut self,
        toggle: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Toggle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDidFavoriteToggleChange", (toggle))?;
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
    pub fn OpenLevelProductStore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OpenLevelProductStore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OpenLevelProductStoreOrShowBuyInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OpenLevelProductStoreOrShowBuyInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshAvailabilityIfNeeded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshAvailabilityIfNeeded", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshContentLevelDetailView(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshContentLevelDetailView", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetData_BeatmapLevelPack_BeatmapLevel__cordl_bool__cordl_bool_Il2CppString_BeatmapDifficultyMask_Il2CppArray1(
        &mut self,
        pack: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        hidePracticeButton: bool,
        canBuyPack: bool,
        playButtonText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        allowedBeatmapDifficultyMask: crate::GlobalNamespace::BeatmapDifficultyMask,
        notAllowedCharacteristics: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetData",
                (
                    pack,
                    beatmapLevel,
                    hidePracticeButton,
                    canBuyPack,
                    playButtonText,
                    allowedBeatmapDifficultyMask,
                    notAllowedCharacteristics,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetData_BeatmapLevel__cordl_bool_Il2CppString_BeatmapDifficultyMask_Il2CppArray0(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        hidePracticeButton: bool,
        playButtonText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        allowedBeatmapDifficultyMask: crate::GlobalNamespace::BeatmapDifficultyMask,
        notAllowedCharacteristics: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetData",
                (
                    beatmapLevel,
                    hidePracticeButton,
                    playButtonText,
                    allowedBeatmapDifficultyMask,
                    notAllowedCharacteristics,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowContent(
        &mut self,
        contentType: crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
        errorText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowContent", (contentType, errorText))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowLoadingAndDoSomething(
        &mut self,
        action: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                crate::System::Threading::CancellationToken,
                *mut crate::System::Threading::Tasks::Task,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowLoadingAndDoSomething", (action))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowOwnedContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowOwnedContent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _BuyPackButtonWasPressed_b__55_0(
        &mut self,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("<BuyPackButtonWasPressed>b__55_0", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn _DidActivate_b__47_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__47_0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _DidActivate_b__47_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__47_1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _DidActivate_b__47_2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__47_2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _DidActivate_b__47_3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__47_3", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _DidActivate_b__47_4(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__47_4", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _DidActivate_b__47_5(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__47_5", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _DidActivate_b__47_6(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__47_6", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _OpenLevelProductStoreOrShowBuyInfo_b__56_0(
        &mut self,
        _cordl__: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object
            .invoke("<OpenLevelProductStoreOrShowBuyInfo>b__56_0", (_cordl__))?;
        Ok(__cordl_ret.into())
    }
    pub fn _OpenLevelProductStore_b__57_0(
        &mut self,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("<OpenLevelProductStore>b__57_0", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn _RefreshAvailabilityIfNeeded_b__58_0(
        &mut self,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("<RefreshAvailabilityIfNeeded>b__58_0", (token))?;
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
    pub fn add_didChangeContentEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::StandardLevelDetailViewController,
                crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didChangeContentEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didChangeDifficultyBeatmapEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::GlobalNamespace::StandardLevelDetailViewController,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didChangeDifficultyBeatmapEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didPressActionButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::GlobalNamespace::StandardLevelDetailViewController,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didPressActionButtonEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didPressOpenLevelPackButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::StandardLevelDetailViewController,
                *mut crate::GlobalNamespace::BeatmapLevelPack,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didPressOpenLevelPackButtonEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didPressPracticeButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::StandardLevelDetailViewController,
                *mut crate::GlobalNamespace::BeatmapLevel,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didPressPracticeButtonEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_levelFavoriteStatusDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::StandardLevelDetailViewController,
                bool,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_levelFavoriteStatusDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapKey = __cordl_object
            .invoke("get_beatmapKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevel,
        > = __cordl_object.invoke("get_beatmapLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didChangeContentEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::StandardLevelDetailViewController,
                crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didChangeContentEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didChangeDifficultyBeatmapEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::GlobalNamespace::StandardLevelDetailViewController,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didChangeDifficultyBeatmapEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didPressActionButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::GlobalNamespace::StandardLevelDetailViewController,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didPressActionButtonEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didPressOpenLevelPackButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::StandardLevelDetailViewController,
                *mut crate::GlobalNamespace::BeatmapLevelPack,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didPressOpenLevelPackButtonEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didPressPracticeButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::StandardLevelDetailViewController,
                *mut crate::GlobalNamespace::BeatmapLevel,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didPressPracticeButtonEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_levelFavoriteStatusDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::StandardLevelDetailViewController,
                bool,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_levelFavoriteStatusDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "StandardLevelDetailViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StandardLevelDetailViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "StandardLevelDetailViewController+ContentType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandardLevelDetailViewController_ContentType {
    Buy = 4i32,
    BuyInfo = 5i32,
    Error = 6i32,
    Inactive = 7i32,
    Loading = 0i32,
    NoAllowedDifficultyBeatmap = 2i32,
    OwnedAndDownloading = 3i32,
    OwnedAndReady = 1i32,
}
#[cfg(feature = "StandardLevelDetailViewController+ContentType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::StandardLevelDetailViewController_ContentType => ""
    ."StandardLevelDetailViewController/ContentType"
);
#[cfg(feature = "StandardLevelDetailViewController+__BuyPackButtonWasPressed_b__55_0_d")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct StandardLevelDetailViewController___BuyPackButtonWasPressed_b__55_0_d {
    pub __1__state: i32,
    pub __t__builder: crate::System::Runtime::CompilerServices::AsyncTaskMethodBuilder,
    pub __4__this: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::StandardLevelDetailViewController,
    >,
    pub token: crate::System::Threading::CancellationToken,
    pub __u__1: crate::System::Runtime::CompilerServices::TaskAwaiter_1<
        crate::GlobalNamespace::OpenProductStoreResult,
    >,
}
#[cfg(feature = "StandardLevelDetailViewController+__BuyPackButtonWasPressed_b__55_0_d")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::StandardLevelDetailViewController___BuyPackButtonWasPressed_b__55_0_d
    => ""."StandardLevelDetailViewController/<<BuyPackButtonWasPressed>b__55_0>d"
);
#[cfg(feature = "StandardLevelDetailViewController+__BuyPackButtonWasPressed_b__55_0_d")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::StandardLevelDetailViewController___BuyPackButtonWasPressed_b__55_0_d {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "StandardLevelDetailViewController+__BuyPackButtonWasPressed_b__55_0_d")]
impl crate::GlobalNamespace::StandardLevelDetailViewController___BuyPackButtonWasPressed_b__55_0_d {
    pub fn MoveNext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStateMachine(
        &mut self,
        stateMachine: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::IAsyncStateMachine,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetStateMachine",
            (stateMachine),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "StandardLevelDetailViewController+__BuyPackButtonWasPressed_b__55_0_d")]
impl AsRef<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::GlobalNamespace::StandardLevelDetailViewController___BuyPackButtonWasPressed_b__55_0_d {
    fn as_ref(&self) -> &crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
#[cfg(feature = "StandardLevelDetailViewController+__BuyPackButtonWasPressed_b__55_0_d")]
impl AsMut<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::GlobalNamespace::StandardLevelDetailViewController___BuyPackButtonWasPressed_b__55_0_d {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
#[cfg(
    feature = "StandardLevelDetailViewController+__OpenLevelProductStoreOrShowBuyInfo_b__56_0_d"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct StandardLevelDetailViewController___OpenLevelProductStoreOrShowBuyInfo_b__56_0_d {
    pub __1__state: i32,
    pub __t__builder: crate::System::Runtime::CompilerServices::AsyncTaskMethodBuilder,
    pub __4__this: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::StandardLevelDetailViewController,
    >,
    pub __u__1: crate::System::Runtime::CompilerServices::TaskAwaiter_1<
        crate::GlobalNamespace::IsPackBetterBuyThanLevelResult,
    >,
}
#[cfg(
    feature = "StandardLevelDetailViewController+__OpenLevelProductStoreOrShowBuyInfo_b__56_0_d"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::StandardLevelDetailViewController___OpenLevelProductStoreOrShowBuyInfo_b__56_0_d
    => ""
    ."StandardLevelDetailViewController/<<OpenLevelProductStoreOrShowBuyInfo>b__56_0>d"
);
#[cfg(
    feature = "StandardLevelDetailViewController+__OpenLevelProductStoreOrShowBuyInfo_b__56_0_d"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::StandardLevelDetailViewController___OpenLevelProductStoreOrShowBuyInfo_b__56_0_d {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "StandardLevelDetailViewController+__OpenLevelProductStoreOrShowBuyInfo_b__56_0_d"
)]
impl crate::GlobalNamespace::StandardLevelDetailViewController___OpenLevelProductStoreOrShowBuyInfo_b__56_0_d {
    pub fn MoveNext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStateMachine(
        &mut self,
        stateMachine: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::IAsyncStateMachine,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetStateMachine",
            (stateMachine),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "StandardLevelDetailViewController+__OpenLevelProductStoreOrShowBuyInfo_b__56_0_d"
)]
impl AsRef<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::GlobalNamespace::StandardLevelDetailViewController___OpenLevelProductStoreOrShowBuyInfo_b__56_0_d {
    fn as_ref(&self) -> &crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
#[cfg(
    feature = "StandardLevelDetailViewController+__OpenLevelProductStoreOrShowBuyInfo_b__56_0_d"
)]
impl AsMut<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::GlobalNamespace::StandardLevelDetailViewController___OpenLevelProductStoreOrShowBuyInfo_b__56_0_d {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
#[cfg(feature = "StandardLevelDetailViewController+__OpenLevelProductStore_b__57_0_d")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct StandardLevelDetailViewController___OpenLevelProductStore_b__57_0_d {
    pub __1__state: i32,
    pub __t__builder: crate::System::Runtime::CompilerServices::AsyncTaskMethodBuilder,
    pub __4__this: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::StandardLevelDetailViewController,
    >,
    pub token: crate::System::Threading::CancellationToken,
    pub __u__1: crate::System::Runtime::CompilerServices::TaskAwaiter_1<
        crate::GlobalNamespace::OpenProductStoreResult,
    >,
}
#[cfg(feature = "StandardLevelDetailViewController+__OpenLevelProductStore_b__57_0_d")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::StandardLevelDetailViewController___OpenLevelProductStore_b__57_0_d
    => ""."StandardLevelDetailViewController/<<OpenLevelProductStore>b__57_0>d"
);
#[cfg(feature = "StandardLevelDetailViewController+__OpenLevelProductStore_b__57_0_d")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::StandardLevelDetailViewController___OpenLevelProductStore_b__57_0_d {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "StandardLevelDetailViewController+__OpenLevelProductStore_b__57_0_d")]
impl crate::GlobalNamespace::StandardLevelDetailViewController___OpenLevelProductStore_b__57_0_d {
    pub fn MoveNext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStateMachine(
        &mut self,
        stateMachine: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::IAsyncStateMachine,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetStateMachine",
            (stateMachine),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "StandardLevelDetailViewController+__OpenLevelProductStore_b__57_0_d")]
impl AsRef<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::GlobalNamespace::StandardLevelDetailViewController___OpenLevelProductStore_b__57_0_d {
    fn as_ref(&self) -> &crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
#[cfg(feature = "StandardLevelDetailViewController+__OpenLevelProductStore_b__57_0_d")]
impl AsMut<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::GlobalNamespace::StandardLevelDetailViewController___OpenLevelProductStore_b__57_0_d {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
#[cfg(
    feature = "StandardLevelDetailViewController+__RefreshAvailabilityIfNeeded_b__58_0_d"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct StandardLevelDetailViewController___RefreshAvailabilityIfNeeded_b__58_0_d {
    pub __1__state: i32,
    pub __t__builder: crate::System::Runtime::CompilerServices::AsyncTaskMethodBuilder,
    pub __4__this: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::StandardLevelDetailViewController,
    >,
    pub token: crate::System::Threading::CancellationToken,
    pub __u__1: crate::System::Runtime::CompilerServices::TaskAwaiter_1<
        crate::GlobalNamespace::EntitlementStatus,
    >,
}
#[cfg(
    feature = "StandardLevelDetailViewController+__RefreshAvailabilityIfNeeded_b__58_0_d"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::StandardLevelDetailViewController___RefreshAvailabilityIfNeeded_b__58_0_d
    => ""."StandardLevelDetailViewController/<<RefreshAvailabilityIfNeeded>b__58_0>d"
);
#[cfg(
    feature = "StandardLevelDetailViewController+__RefreshAvailabilityIfNeeded_b__58_0_d"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::StandardLevelDetailViewController___RefreshAvailabilityIfNeeded_b__58_0_d {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "StandardLevelDetailViewController+__RefreshAvailabilityIfNeeded_b__58_0_d"
)]
impl crate::GlobalNamespace::StandardLevelDetailViewController___RefreshAvailabilityIfNeeded_b__58_0_d {
    pub fn MoveNext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStateMachine(
        &mut self,
        stateMachine: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::IAsyncStateMachine,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetStateMachine",
            (stateMachine),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "StandardLevelDetailViewController+__RefreshAvailabilityIfNeeded_b__58_0_d"
)]
impl AsRef<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::GlobalNamespace::StandardLevelDetailViewController___RefreshAvailabilityIfNeeded_b__58_0_d {
    fn as_ref(&self) -> &crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
#[cfg(
    feature = "StandardLevelDetailViewController+__RefreshAvailabilityIfNeeded_b__58_0_d"
)]
impl AsMut<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::GlobalNamespace::StandardLevelDetailViewController___RefreshAvailabilityIfNeeded_b__58_0_d {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
