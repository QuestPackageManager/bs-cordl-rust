#[cfg(feature = "DlcPromoPanelModel")]
#[repr(C)]
#[derive(Debug)]
pub struct DlcPromoPanelModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _additionalContentModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IAdditionalContentModel,
    >,
    pub _beatmapLevelsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsModel,
    >,
    pub _analyticsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IAnalyticsModel,
    >,
    pub _dlcPromoPanelData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DlcPromoPanelDataSO,
    >,
    pub _playerDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerDataModel,
    >,
    pub _platformInit: quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::IPlatformInit>,
    pub _metaRemoteAssetsManager: quest_hook::libil2cpp::Gc<
        crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager,
    >,
    pub _notOwnedMusicPackPromoInfos: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
            >,
        >,
    >,
    pub _ownedMusicPackPromoInfos: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
            >,
        >,
    >,
    pub _updatingNotOwnedPacks: bool,
    pub _initialized: bool,
    pub _random: quest_hook::libil2cpp::Gc<crate::System::Random>,
    pub _promoInfos: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
            >,
        >,
    >,
    pub _defaultPromoInfo: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
    >,
    pub _initializationTask: quest_hook::libil2cpp::Gc<
        crate::System::Threading::Tasks::Task,
    >,
    pub _cacheNextPackDataTask: quest_hook::libil2cpp::Gc<
        crate::System::Threading::Tasks::Task_1<
            crate::System::ValueTuple_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
                >,
                bool,
            >,
        >,
    >,
    pub _loadDlcPromoPanelDataHandle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::DlcPromoPanelDataSO>,
    >,
    pub _loadPackPromoInfoHandles: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PackPromoInfoSO>,
            >,
        >,
    >,
    pub hotReloadDidStart: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub hotReloadDidFinish: quest_hook::libil2cpp::Gc<crate::System::Action>,
}
#[cfg(feature = "DlcPromoPanelModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DlcPromoPanelModel => ""
    ."DlcPromoPanelModel"
);
#[cfg(feature = "DlcPromoPanelModel")]
impl std::ops::Deref for crate::GlobalNamespace::DlcPromoPanelModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DlcPromoPanelModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::DlcPromoPanelModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DlcPromoPanelModel")]
impl crate::GlobalNamespace::DlcPromoPanelModel {
    #[cfg(feature = "DlcPromoPanelModel+PromoInfo")]
    pub type PromoInfo = crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo;
    pub fn BuyLevelButtonWasPressed(
        &mut self,
        level: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        page: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        customText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuyLevelButtonWasPressed", (level, page, customText))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuyLevelButtonWasShown(
        &mut self,
        level: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        page: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        customText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuyLevelButtonWasShown", (level, page, customText))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuyPackButtonWasPressed(
        &mut self,
        pack: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
        page: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        customText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuyPackButtonWasPressed", (pack, page, customText))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuyPackButtonWasShown(
        &mut self,
        pack: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
        page: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        customText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuyPackButtonWasShown", (pack, page, customText))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExperimentEventData(
        itemId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        page: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        customText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetExperimentEventData", (itemId, page, customText))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPackDataForMainMenuPromoBanner(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::ValueTuple_2<
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
                    >,
                    bool,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::ValueTuple_2<
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
                    >,
                    bool,
                >,
            >,
        > = __cordl_object.invoke("GetPackDataForMainMenuPromoBanner", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPackDataForMainMenuPromoBannerInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::ValueTuple_2<
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
                    >,
                    bool,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::System::ValueTuple_2<
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
                    >,
                    bool,
                >,
            >,
        > = __cordl_object.invoke("GetPackDataForMainMenuPromoBannerInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleAdditionalContentModelDidInvalidateData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAdditionalContentModelDidInvalidateData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleDidCatalogLoadOrUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDidCatalogLoadOrUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("InitializeAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeInternalAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("InitializeInternalAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadPackPromoInfoAsync(
        &mut self,
        pack: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PackDefinitionSO>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PackPromoInfoSO>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PackPromoInfoSO>,
            >,
        > = __cordl_object.invoke("LoadPackPromoInfoAsync", (pack))?;
        Ok(__cordl_ret.into())
    }
    pub fn MainMenuDlcPromoBannerWasPressed(
        &mut self,
        packId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        customText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MainMenuDlcPromoBannerWasPressed", (packId, customText))?;
        Ok(__cordl_ret.into())
    }
    pub fn MainMenuDlcPromoBannerWasShown(
        &mut self,
        packId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        customText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MainMenuDlcPromoBannerWasShown", (packId, customText))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        additionalContentModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IAdditionalContentModel,
        >,
        beatmapLevelsModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsModel,
        >,
        analyticsModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IAnalyticsModel,
        >,
        defaultDlcPromoPanelData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DlcPromoPanelDataSO,
        >,
        playerDataModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerDataModel,
        >,
        platformInit: quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::IPlatformInit>,
        metaRemoteAssetsManager: quest_hook::libil2cpp::Gc<
            crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    additionalContentModel,
                    beatmapLevelsModel,
                    analyticsModel,
                    defaultDlcPromoPanelData,
                    playerDataModel,
                    platformInit,
                    metaRemoteAssetsManager,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateDlcPromoPanelDataAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("UpdateDlcPromoPanelDataAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateModelDataAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("UpdateModelDataAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateOwnedPacksAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("UpdateOwnedPacksAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdatePromoInfosAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("UpdatePromoInfosAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        additionalContentModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IAdditionalContentModel,
        >,
        beatmapLevelsModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsModel,
        >,
        analyticsModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IAnalyticsModel,
        >,
        defaultDlcPromoPanelData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DlcPromoPanelDataSO,
        >,
        playerDataModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerDataModel,
        >,
        platformInit: quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::IPlatformInit>,
        metaRemoteAssetsManager: quest_hook::libil2cpp::Gc<
            crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    additionalContentModel,
                    beatmapLevelsModel,
                    analyticsModel,
                    defaultDlcPromoPanelData,
                    playerDataModel,
                    platformInit,
                    metaRemoteAssetsManager,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn add_hotReloadDidFinish(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_hotReloadDidFinish", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_hotReloadDidStart(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_hotReloadDidStart", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_hotReloadDidFinish(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_hotReloadDidFinish", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_hotReloadDidStart(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_hotReloadDidStart", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DlcPromoPanelModel")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::DlcPromoPanelModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "DlcPromoPanelModel")]
impl AsRef<crate::Zenject::IInitializable>
for crate::GlobalNamespace::DlcPromoPanelModel {
    fn as_ref(&self) -> &crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "DlcPromoPanelModel")]
impl AsMut<crate::Zenject::IInitializable>
for crate::GlobalNamespace::DlcPromoPanelModel {
    fn as_mut(&mut self) -> &mut crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "DlcPromoPanelModel+PromoInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct DlcPromoPanelModel_PromoInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub promoType: crate::GlobalNamespace::PromoInfo_DlcPromoPanelModel_PromoType,
    pub id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
    pub bannerImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub bannerPromoText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub bannerPromoTextPosition: f32,
}
#[cfg(feature = "DlcPromoPanelModel+PromoInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DlcPromoPanelModel_PromoInfo =>
    ""."DlcPromoPanelModel/PromoInfo"
);
#[cfg(feature = "DlcPromoPanelModel+PromoInfo")]
impl std::ops::Deref for crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DlcPromoPanelModel+PromoInfo")]
impl std::ops::DerefMut for crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DlcPromoPanelModel+PromoInfo")]
impl crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo {
    #[cfg(feature = "DlcPromoPanelModel+PromoInfo+PromoType")]
    pub type PromoType = crate::GlobalNamespace::PromoInfo_DlcPromoPanelModel_PromoType;
    pub fn New_PackPromoInfoSO_LevelPromoInfo_PlayerSensitivityFlag1(
        levelPromoInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PackPromoInfoSO_LevelPromoInfo,
        >,
        contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (levelPromoInfo, contentRating))?;
        Ok(__cordl_object.into())
    }
    pub fn New_PromoInfo_DlcPromoPanelModel_PromoType_Il2CppString_PromoBannerInfoSO_PlayerSensitivityFlag0(
        promoType: crate::GlobalNamespace::PromoInfo_DlcPromoPanelModel_PromoType,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        promoBannerInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PromoBannerInfoSO,
        >,
        contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (promoType, id, promoBannerInfo, contentRating))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_PackPromoInfoSO_LevelPromoInfo_PlayerSensitivityFlag1(
        &mut self,
        levelPromoInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PackPromoInfoSO_LevelPromoInfo,
        >,
        contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (levelPromoInfo, contentRating))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PromoInfo_DlcPromoPanelModel_PromoType_Il2CppString_PromoBannerInfoSO_PlayerSensitivityFlag0(
        &mut self,
        promoType: crate::GlobalNamespace::PromoInfo_DlcPromoPanelModel_PromoType,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        promoBannerInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PromoBannerInfoSO,
        >,
        contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (promoType, id, promoBannerInfo, contentRating))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DlcPromoPanelModel+PromoInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "DlcPromoPanelModel+PromoInfo+PromoType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PromoInfo_DlcPromoPanelModel_PromoType {
    #[default]
    Level = 1i32,
    Pack = 0i32,
    Store = 2i32,
}
#[cfg(feature = "DlcPromoPanelModel+PromoInfo+PromoType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PromoInfo_DlcPromoPanelModel_PromoType => ""
    ."DlcPromoPanelModel/PromoInfo/PromoType"
);
