#[cfg(feature = "OculusBeatmapDataAssetFileModel+LevelDownloadingData")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusBeatmapDataAssetFileModel_LevelDownloadingData {
    __cordl_parent: crate::System::Object,
    pub levelId: *mut crate::System::String,
    pub assetBundlePath: *mut crate::System::String,
    pub downloadAssetBundleFileTCS: *mut crate::System::Threading::Tasks::TaskCompletionSource_1<
        GetAssetBundleFileResult,
    >,
}
#[cfg(feature = "OculusBeatmapDataAssetFileModel+LevelDownloadingData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OculusBeatmapDataAssetFileModel_LevelDownloadingData => ""
    ."OculusBeatmapDataAssetFileModel/LevelDownloadingData"
);
#[cfg(feature = "OculusBeatmapDataAssetFileModel+LevelDownloadingData")]
impl std::ops::Deref
for crate::GlobalNamespace::OculusBeatmapDataAssetFileModel_LevelDownloadingData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusBeatmapDataAssetFileModel+LevelDownloadingData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OculusBeatmapDataAssetFileModel_LevelDownloadingData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusBeatmapDataAssetFileModel+LevelDownloadingData")]
impl crate::GlobalNamespace::OculusBeatmapDataAssetFileModel_LevelDownloadingData {
    pub fn New(
        levelId: *mut crate::System::String,
        assetBundlePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (levelId, assetBundlePath))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        levelId: *mut crate::System::String,
        assetBundlePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (levelId, assetBundlePath))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OculusBeatmapDataAssetFileModel+LevelDownloadingData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OculusBeatmapDataAssetFileModel_LevelDownloadingData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OculusBeatmapDataAssetFileModel")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusBeatmapDataAssetFileModel {
    __cordl_parent: crate::System::Object,
    pub levelDataAssetDownloadUpdateEvent: *mut crate::System::Action_1<
        LevelDataAssetDownloadUpdate,
    >,
    pub _assetIdToDownloadingData: *mut crate::System::Collections::Generic::Dictionary_2<
        u64,
        *mut crate::GlobalNamespace::OculusBeatmapDataAssetFileModel_LevelDownloadingData,
    >,
    pub _downloadedAssetBundleFiles: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::System::String,
    >,
    pub _lastAssetFileDownloadUpdateForAssetIds: *mut crate::System::Collections::Generic::Dictionary_2<
        u64,
        *mut crate::Oculus::Platform::Models::AssetFileDownloadUpdate,
    >,
    pub _lastAssetFileDownloadUpdateTime: f32,
    pub _semaphoreSlim: *mut crate::System::Threading::SemaphoreSlim,
    pub _assetFileToAssetDetails: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::Oculus::Platform::Models::AssetDetails,
    >,
    pub _oculusPlatformAdditionalContentModel: *mut OculusPlatformAdditionalContentModel,
}
#[cfg(feature = "OculusBeatmapDataAssetFileModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OculusBeatmapDataAssetFileModel => ""
    ."OculusBeatmapDataAssetFileModel"
);
#[cfg(feature = "OculusBeatmapDataAssetFileModel")]
impl std::ops::Deref for OculusBeatmapDataAssetFileModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusBeatmapDataAssetFileModel")]
impl std::ops::DerefMut for OculusBeatmapDataAssetFileModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusBeatmapDataAssetFileModel")]
impl OculusBeatmapDataAssetFileModel {
    pub const kMaxTimeOutBeforeFail: f32 = 120f32;
    #[cfg(feature = "OculusBeatmapDataAssetFileModel+LevelDownloadingData")]
    pub type LevelDownloadingData = crate::GlobalNamespace::OculusBeatmapDataAssetFileModel_LevelDownloadingData;
    #[cfg(
        feature = "OculusBeatmapDataAssetFileModel+_TryDeleteAssetBundleFileForBeatmapLevelAsync_d__13"
    )]
    pub type _TryDeleteAssetBundleFileForBeatmapLevelAsync_d__13 = crate::GlobalNamespace::OculusBeatmapDataAssetFileModel__TryDeleteAssetBundleFileForBeatmapLevelAsync_d__13;
    #[cfg(feature = "OculusBeatmapDataAssetFileModel+__c__DisplayClass15_0")]
    pub type __c__DisplayClass15_0 = crate::GlobalNamespace::OculusBeatmapDataAssetFileModel___c__DisplayClass15_0;
    #[cfg(feature = "OculusBeatmapDataAssetFileModel+__c__DisplayClass16_0")]
    pub type __c__DisplayClass16_0 = crate::GlobalNamespace::OculusBeatmapDataAssetFileModel___c__DisplayClass16_0;
    #[cfg(feature = "OculusBeatmapDataAssetFileModel+__c__DisplayClass13_0")]
    pub type __c__DisplayClass13_0 = crate::GlobalNamespace::OculusBeatmapDataAssetFileModel___c__DisplayClass13_0;
    #[cfg(
        feature = "OculusBeatmapDataAssetFileModel+_ReloadAssetDetailsForAllLevelsAsync_d__15"
    )]
    pub type _ReloadAssetDetailsForAllLevelsAsync_d__15 = crate::GlobalNamespace::OculusBeatmapDataAssetFileModel__ReloadAssetDetailsForAllLevelsAsync_d__15;
    #[cfg(
        feature = "OculusBeatmapDataAssetFileModel+_GetDownloadAssetBundleFileAsync_d__16"
    )]
    pub type _GetDownloadAssetBundleFileAsync_d__16 = crate::GlobalNamespace::OculusBeatmapDataAssetFileModel__GetDownloadAssetBundleFileAsync_d__16;
    #[cfg(
        feature = "OculusBeatmapDataAssetFileModel+_GetAssetBundleFileForBeatmapLevelAsync_d__14"
    )]
    pub type _GetAssetBundleFileForBeatmapLevelAsync_d__14 = crate::GlobalNamespace::OculusBeatmapDataAssetFileModel__GetAssetBundleFileForBeatmapLevelAsync_d__14;
    pub fn GetAssetBundleFileForBeatmapLevelAsync(
        &mut self,
        beatmapLevel: *mut BeatmapLevel,
        beatmapLevelDataVersion: BeatmapLevelDataVersion,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<GetAssetBundleFileResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            GetAssetBundleFileResult,
        > = __cordl_object
            .invoke(
                "GetAssetBundleFileForBeatmapLevelAsync",
                (beatmapLevel, beatmapLevelDataVersion, cancellationToken),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetDownloadAssetBundleFileAsync(
        &mut self,
        levelId: *mut crate::System::String,
        assetDetails: *mut crate::Oculus::Platform::Models::AssetDetails,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<GetAssetBundleFileResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            GetAssetBundleFileResult,
        > = __cordl_object
            .invoke(
                "GetDownloadAssetBundleFileAsync",
                (levelId, assetDetails, cancellationToken),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetTaskCompletionSourceForDownload(
        &mut self,
        levelId: *mut crate::System::String,
        assetDetail: *mut crate::Oculus::Platform::Models::AssetDetails,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::TaskCompletionSource_1<
            GetAssetBundleFileResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::TaskCompletionSource_1<
            GetAssetBundleFileResult,
        > = __cordl_object
            .invoke("GetTaskCompletionSourceForDownload", (levelId, assetDetail))?;
        Ok(__cordl_ret)
    }
    pub fn HandleAssetFileDownloadUpdate(
        &mut self,
        msg: *mut crate::Oculus::Platform::Message_1<
            *mut crate::Oculus::Platform::Models::AssetFileDownloadUpdate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAssetFileDownloadUpdate", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        oculusPlatformAdditionalContentModel: *mut OculusPlatformAdditionalContentModel,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (oculusPlatformAdditionalContentModel))?;
        Ok(__cordl_object)
    }
    pub fn ReloadAssetDetailsForAllLevelsAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<bool> = __cordl_object
            .invoke("ReloadAssetDetailsForAllLevelsAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn TryDeleteAssetBundleFileForBeatmapLevelAsync(
        &mut self,
        beatmapLevel: *mut BeatmapLevel,
        beatmapLevelDataVersion: BeatmapLevelDataVersion,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<bool> = __cordl_object
            .invoke(
                "TryDeleteAssetBundleFileForBeatmapLevelAsync",
                (beatmapLevel, beatmapLevelDataVersion, cancellationToken),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        oculusPlatformAdditionalContentModel: *mut OculusPlatformAdditionalContentModel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (oculusPlatformAdditionalContentModel))?;
        Ok(__cordl_ret)
    }
    pub fn add_levelDataAssetDownloadUpdateEvent(
        &mut self,
        value: *mut crate::System::Action_1<LevelDataAssetDownloadUpdate>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_levelDataAssetDownloadUpdateEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_levelDataAssetDownloadUpdateEvent(
        &mut self,
        value: *mut crate::System::Action_1<LevelDataAssetDownloadUpdate>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_levelDataAssetDownloadUpdateEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OculusBeatmapDataAssetFileModel")]
impl quest_hook::libil2cpp::ObjectType for OculusBeatmapDataAssetFileModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
