#[cfg(feature = "DlcPromoPanelModel")]
#[repr(C)]
#[derive(Debug)]
pub struct DlcPromoPanelModel {
    __cordl_parent: crate::System::Object,
    pub _additionalContentModel: *mut crate::GlobalNamespace::IAdditionalContentModel,
    pub _beatmapLevelsModel: *mut crate::GlobalNamespace::BeatmapLevelsModel,
    pub _analyticsModel: *mut crate::GlobalNamespace::IAnalyticsModel,
    pub _dlcPromoPanelData: *mut crate::GlobalNamespace::DlcPromoPanelDataSO,
    pub _playerDataModel: *mut crate::GlobalNamespace::PlayerDataModel,
    pub _platformInit: *mut crate::GlobalNamespace::IPlatformInit,
    pub _metaRemoteAssetsManager: *mut crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager,
    pub _notOwnedMusicPackPromoInfos: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
    >,
    pub _ownedMusicPackPromoInfos: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
    >,
    pub _updatingNotOwnedPacks: bool,
    pub _initialized: bool,
    pub _random: *mut crate::System::Random,
    pub _promoInfos: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
    >,
    pub _defaultPromoInfo: *mut crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
    pub _initializationTask: *mut crate::System::Threading::Tasks::Task,
    pub _cacheNextPackDataTask: *mut crate::System::Threading::Tasks::Task_1<
        crate::System::ValueTuple_2<
            *mut crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
            bool,
        >,
    >,
    pub _loadPackPromoInfoHandles: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::GlobalNamespace::PackPromoInfoSO,
        >,
    >,
    pub hotReloadDidStart: *mut crate::System::Action,
    pub hotReloadDidFinish: *mut crate::System::Action,
}
#[cfg(feature = "DlcPromoPanelModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DlcPromoPanelModel => ""
    ."DlcPromoPanelModel"
);
#[cfg(feature = "DlcPromoPanelModel")]
impl std::ops::Deref for crate::GlobalNamespace::DlcPromoPanelModel {
    type Target = crate::System::Object;
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
    #[cfg(
        feature = "DlcPromoPanelModel+_GetPackDataForMainMenuPromoBannerInternal_d__34"
    )]
    pub type _GetPackDataForMainMenuPromoBannerInternal_d__34 = crate::GlobalNamespace::DlcPromoPanelModel__GetPackDataForMainMenuPromoBannerInternal_d__34;
    #[cfg(feature = "DlcPromoPanelModel+_GetPackDataForMainMenuPromoBanner_d__33")]
    pub type _GetPackDataForMainMenuPromoBanner_d__33 = crate::GlobalNamespace::DlcPromoPanelModel__GetPackDataForMainMenuPromoBanner_d__33;
    #[cfg(
        feature = "DlcPromoPanelModel+_HandleAdditionalContentModelDidInvalidateData_d__43"
    )]
    pub type _HandleAdditionalContentModelDidInvalidateData_d__43 = crate::GlobalNamespace::DlcPromoPanelModel__HandleAdditionalContentModelDidInvalidateData_d__43;
    #[cfg(feature = "DlcPromoPanelModel+_HandleDidCatalogLoadOrUpdate_d__29")]
    pub type _HandleDidCatalogLoadOrUpdate_d__29 = crate::GlobalNamespace::DlcPromoPanelModel__HandleDidCatalogLoadOrUpdate_d__29;
    #[cfg(feature = "DlcPromoPanelModel+_InitializeInternalAsync_d__27")]
    pub type _InitializeInternalAsync_d__27 = crate::GlobalNamespace::DlcPromoPanelModel__InitializeInternalAsync_d__27;
    #[cfg(feature = "DlcPromoPanelModel+_Initialize_d__25")]
    pub type _Initialize_d__25 = crate::GlobalNamespace::DlcPromoPanelModel__Initialize_d__25;
    #[cfg(feature = "DlcPromoPanelModel+_LoadPackPromoInfoAsync_d__32")]
    pub type _LoadPackPromoInfoAsync_d__32 = crate::GlobalNamespace::DlcPromoPanelModel__LoadPackPromoInfoAsync_d__32;
    #[cfg(feature = "DlcPromoPanelModel+_UpdateDlcPromoPanelDataAsync_d__30")]
    pub type _UpdateDlcPromoPanelDataAsync_d__30 = crate::GlobalNamespace::DlcPromoPanelModel__UpdateDlcPromoPanelDataAsync_d__30;
    #[cfg(feature = "DlcPromoPanelModel+_UpdateModelDataAsync_d__28")]
    pub type _UpdateModelDataAsync_d__28 = crate::GlobalNamespace::DlcPromoPanelModel__UpdateModelDataAsync_d__28;
    #[cfg(feature = "DlcPromoPanelModel+_UpdateOwnedPacksAsync_d__42")]
    pub type _UpdateOwnedPacksAsync_d__42 = crate::GlobalNamespace::DlcPromoPanelModel__UpdateOwnedPacksAsync_d__42;
    #[cfg(feature = "DlcPromoPanelModel+_UpdatePromoInfosAsync_d__31")]
    pub type _UpdatePromoInfosAsync_d__31 = crate::GlobalNamespace::DlcPromoPanelModel__UpdatePromoInfosAsync_d__31;
    #[cfg(feature = "DlcPromoPanelModel+__c")]
    pub type __c = crate::GlobalNamespace::DlcPromoPanelModel___c;
    #[cfg(feature = "DlcPromoPanelModel+__c__DisplayClass34_0")]
    pub type __c__DisplayClass34_0 = crate::GlobalNamespace::DlcPromoPanelModel___c__DisplayClass34_0;
    pub fn BuyLevelButtonWasPressed(
        &mut self,
        level: *mut crate::GlobalNamespace::BeatmapLevel,
        page: *mut crate::System::String,
        customText: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuyLevelButtonWasPressed", (level, page, customText))?;
        Ok(__cordl_ret)
    }
    pub fn BuyLevelButtonWasShown(
        &mut self,
        level: *mut crate::GlobalNamespace::BeatmapLevel,
        page: *mut crate::System::String,
        customText: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuyLevelButtonWasShown", (level, page, customText))?;
        Ok(__cordl_ret)
    }
    pub fn BuyPackButtonWasPressed(
        &mut self,
        pack: *mut crate::GlobalNamespace::BeatmapLevelPack,
        page: *mut crate::System::String,
        customText: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuyPackButtonWasPressed", (pack, page, customText))?;
        Ok(__cordl_ret)
    }
    pub fn BuyPackButtonWasShown(
        &mut self,
        pack: *mut crate::GlobalNamespace::BeatmapLevelPack,
        page: *mut crate::System::String,
        customText: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuyPackButtonWasShown", (pack, page, customText))?;
        Ok(__cordl_ret)
    }
    pub fn GetPackDataForMainMenuPromoBanner(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::System::ValueTuple_2<
                *mut crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
                bool,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::System::ValueTuple_2<
                *mut crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
                bool,
            >,
        > = __cordl_object.invoke("GetPackDataForMainMenuPromoBanner", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPackDataForMainMenuPromoBannerInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::System::ValueTuple_2<
                *mut crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
                bool,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::System::ValueTuple_2<
                *mut crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
                bool,
            >,
        > = __cordl_object.invoke("GetPackDataForMainMenuPromoBannerInternal", ())?;
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
    pub fn HandleDidCatalogLoadOrUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDidCatalogLoadOrUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitializeAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("InitializeAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitializeInternalAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("InitializeInternalAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadPackPromoInfoAsync(
        &mut self,
        pack: *mut crate::GlobalNamespace::PackDefinitionSO,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::GlobalNamespace::PackPromoInfoSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::GlobalNamespace::PackPromoInfoSO,
        > = __cordl_object.invoke("LoadPackPromoInfoAsync", (pack))?;
        Ok(__cordl_ret)
    }
    pub fn MainMenuDlcPromoBannerWasPressed(
        &mut self,
        packId: *mut crate::System::String,
        customText: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MainMenuDlcPromoBannerWasPressed", (packId, customText))?;
        Ok(__cordl_ret)
    }
    pub fn MainMenuDlcPromoBannerWasShown(
        &mut self,
        packId: *mut crate::System::String,
        customText: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MainMenuDlcPromoBannerWasShown", (packId, customText))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        additionalContentModel: *mut crate::GlobalNamespace::IAdditionalContentModel,
        beatmapLevelsModel: *mut crate::GlobalNamespace::BeatmapLevelsModel,
        analyticsModel: *mut crate::GlobalNamespace::IAnalyticsModel,
        defaultDlcPromoPanelData: *mut crate::GlobalNamespace::DlcPromoPanelDataSO,
        playerDataModel: *mut crate::GlobalNamespace::PlayerDataModel,
        platformInit: *mut crate::GlobalNamespace::IPlatformInit,
        metaRemoteAssetsManager: *mut crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
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
        Ok(__cordl_object)
    }
    pub fn UpdateDlcPromoPanelDataAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("UpdateDlcPromoPanelDataAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateModelDataAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("UpdateModelDataAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateOwnedPacksAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("UpdateOwnedPacksAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdatePromoInfosAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("UpdatePromoInfosAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        additionalContentModel: *mut crate::GlobalNamespace::IAdditionalContentModel,
        beatmapLevelsModel: *mut crate::GlobalNamespace::BeatmapLevelsModel,
        analyticsModel: *mut crate::GlobalNamespace::IAnalyticsModel,
        defaultDlcPromoPanelData: *mut crate::GlobalNamespace::DlcPromoPanelDataSO,
        playerDataModel: *mut crate::GlobalNamespace::PlayerDataModel,
        platformInit: *mut crate::GlobalNamespace::IPlatformInit,
        metaRemoteAssetsManager: *mut crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager,
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
        Ok(__cordl_ret)
    }
    pub fn add_hotReloadDidFinish(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_hotReloadDidFinish", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_hotReloadDidStart(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_hotReloadDidStart", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_hotReloadDidFinish(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_hotReloadDidFinish", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_hotReloadDidStart(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_hotReloadDidStart", (value))?;
        Ok(__cordl_ret)
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
#[cfg(feature = "DlcPromoPanelModel+PromoInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct DlcPromoPanelModel_PromoInfo {
    __cordl_parent: crate::System::Object,
    pub promoType: crate::GlobalNamespace::PromoInfo_DlcPromoPanelModel_PromoType,
    pub id: *mut crate::System::String,
    pub contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
    pub bannerImage: *mut crate::UnityEngine::Sprite,
    pub bannerPromoText: *mut crate::System::String,
    pub bannerPromoTextPosition: f32,
}
#[cfg(feature = "DlcPromoPanelModel+PromoInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DlcPromoPanelModel_PromoInfo =>
    ""."DlcPromoPanelModel/PromoInfo"
);
#[cfg(feature = "DlcPromoPanelModel+PromoInfo")]
impl std::ops::Deref for crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo {
    type Target = crate::System::Object;
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
        levelPromoInfo: *mut crate::GlobalNamespace::PackPromoInfoSO_LevelPromoInfo,
        contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (levelPromoInfo, contentRating))?;
        Ok(__cordl_object)
    }
    pub fn New_String_PromoBannerInfoSO_PlayerSensitivityFlag0(
        packId: *mut crate::System::String,
        promoBannerInfo: *mut crate::GlobalNamespace::PromoBannerInfoSO,
        contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (packId, promoBannerInfo, contentRating))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_PackPromoInfoSO_LevelPromoInfo_PlayerSensitivityFlag1(
        &mut self,
        levelPromoInfo: *mut crate::GlobalNamespace::PackPromoInfoSO_LevelPromoInfo,
        contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (levelPromoInfo, contentRating))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_PromoBannerInfoSO_PlayerSensitivityFlag0(
        &mut self,
        packId: *mut crate::System::String,
        promoBannerInfo: *mut crate::GlobalNamespace::PromoBannerInfoSO,
        contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (packId, promoBannerInfo, contentRating))?;
        Ok(__cordl_ret)
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromoInfo_DlcPromoPanelModel_PromoType {
    Level = 1i32,
    Pack = 0i32,
}
#[cfg(feature = "DlcPromoPanelModel+PromoInfo+PromoType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PromoInfo_DlcPromoPanelModel_PromoType => ""
    ."DlcPromoPanelModel/PromoInfo/PromoType"
);
