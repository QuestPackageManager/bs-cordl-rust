#[cfg(feature = "OculusBeatmapDataAssetFileModel")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusBeatmapDataAssetFileModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub levelDataAssetDownloadUpdateEvent: *mut crate::System::Action_1<
        crate::GlobalNamespace::LevelDataAssetDownloadUpdate,
    >,
    pub _assetIdToDownloadingData: *mut crate::System::Collections::Generic::Dictionary_2<
        u64,
        *mut crate::GlobalNamespace::OculusBeatmapDataAssetFileModel_LevelDownloadingData,
    >,
    pub _downloadedAssetBundleFiles: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _lastAssetFileDownloadUpdateForAssetIds: *mut crate::System::Collections::Generic::Dictionary_2<
        u64,
        *mut crate::Oculus::Platform::Models::AssetFileDownloadUpdate,
    >,
    pub _lastAssetFileDownloadUpdateTime: f32,
    pub _semaphoreSlim: *mut crate::System::Threading::SemaphoreSlim,
    pub _assetFileToAssetDetails: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::Oculus::Platform::Models::AssetDetails,
    >,
    pub _oculusPlatformAdditionalContentModel: *mut crate::GlobalNamespace::OculusPlatformAdditionalContentModel,
}
#[cfg(feature = "OculusBeatmapDataAssetFileModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OculusBeatmapDataAssetFileModel
    => ""."OculusBeatmapDataAssetFileModel"
);
#[cfg(feature = "OculusBeatmapDataAssetFileModel")]
impl std::ops::Deref for crate::GlobalNamespace::OculusBeatmapDataAssetFileModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusBeatmapDataAssetFileModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::OculusBeatmapDataAssetFileModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusBeatmapDataAssetFileModel")]
impl crate::GlobalNamespace::OculusBeatmapDataAssetFileModel {
    pub const kMaxTimeOutBeforeFail: f32 = 120f32;
    #[cfg(feature = "OculusBeatmapDataAssetFileModel+LevelDownloadingData")]
    pub type LevelDownloadingData = crate::GlobalNamespace::OculusBeatmapDataAssetFileModel_LevelDownloadingData;
    #[cfg(
        feature = "OculusBeatmapDataAssetFileModel+_GetAssetBundleFileForBeatmapLevelAsync_d__14"
    )]
    pub type _GetAssetBundleFileForBeatmapLevelAsync_d__14 = crate::GlobalNamespace::OculusBeatmapDataAssetFileModel__GetAssetBundleFileForBeatmapLevelAsync_d__14;
    #[cfg(
        feature = "OculusBeatmapDataAssetFileModel+_GetDownloadAssetBundleFileAsync_d__16"
    )]
    pub type _GetDownloadAssetBundleFileAsync_d__16 = crate::GlobalNamespace::OculusBeatmapDataAssetFileModel__GetDownloadAssetBundleFileAsync_d__16;
    #[cfg(
        feature = "OculusBeatmapDataAssetFileModel+_ReloadAssetDetailsForAllLevelsAsync_d__15"
    )]
    pub type _ReloadAssetDetailsForAllLevelsAsync_d__15 = crate::GlobalNamespace::OculusBeatmapDataAssetFileModel__ReloadAssetDetailsForAllLevelsAsync_d__15;
    #[cfg(
        feature = "OculusBeatmapDataAssetFileModel+_TryDeleteAssetBundleFileForBeatmapLevelAsync_d__13"
    )]
    pub type _TryDeleteAssetBundleFileForBeatmapLevelAsync_d__13 = crate::GlobalNamespace::OculusBeatmapDataAssetFileModel__TryDeleteAssetBundleFileForBeatmapLevelAsync_d__13;
    #[cfg(feature = "OculusBeatmapDataAssetFileModel+__c__DisplayClass13_0")]
    pub type __c__DisplayClass13_0 = crate::GlobalNamespace::OculusBeatmapDataAssetFileModel___c__DisplayClass13_0;
    #[cfg(feature = "OculusBeatmapDataAssetFileModel+__c__DisplayClass15_0")]
    pub type __c__DisplayClass15_0 = crate::GlobalNamespace::OculusBeatmapDataAssetFileModel___c__DisplayClass15_0;
    #[cfg(feature = "OculusBeatmapDataAssetFileModel+__c__DisplayClass16_0")]
    pub type __c__DisplayClass16_0 = crate::GlobalNamespace::OculusBeatmapDataAssetFileModel___c__DisplayClass16_0;
    pub fn GetAssetBundleFileForBeatmapLevelAsync(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::GetAssetBundleFileResult,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::GetAssetBundleFileResult,
            >,
        > = __cordl_object
            .invoke(
                "GetAssetBundleFileForBeatmapLevelAsync",
                (beatmapLevel, beatmapLevelDataVersion, cancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDownloadAssetBundleFileAsync(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        assetDetails: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::AssetDetails,
        >,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::GetAssetBundleFileResult,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::GetAssetBundleFileResult,
            >,
        > = __cordl_object
            .invoke(
                "GetDownloadAssetBundleFileAsync",
                (levelId, assetDetails, cancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTaskCompletionSourceForDownload(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        assetDetail: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::AssetDetails,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskCompletionSource_1<
                crate::GlobalNamespace::GetAssetBundleFileResult,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskCompletionSource_1<
                crate::GlobalNamespace::GetAssetBundleFileResult,
            >,
        > = __cordl_object
            .invoke("GetTaskCompletionSourceForDownload", (levelId, assetDetail))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleAssetFileDownloadUpdate(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1<
                *mut crate::Oculus::Platform::Models::AssetFileDownloadUpdate,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAssetFileDownloadUpdate", (msg))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        oculusPlatformAdditionalContentModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusPlatformAdditionalContentModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (oculusPlatformAdditionalContentModel))?;
        Ok(__cordl_object.into())
    }
    pub fn ReloadAssetDetailsForAllLevelsAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object
            .invoke("ReloadAssetDetailsForAllLevelsAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryDeleteAssetBundleFileForBeatmapLevelAsync(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object
            .invoke(
                "TryDeleteAssetBundleFileForBeatmapLevelAsync",
                (beatmapLevel, beatmapLevelDataVersion, cancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        oculusPlatformAdditionalContentModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OculusPlatformAdditionalContentModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (oculusPlatformAdditionalContentModel))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_levelDataAssetDownloadUpdateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::LevelDataAssetDownloadUpdate>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_levelDataAssetDownloadUpdateEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_levelDataAssetDownloadUpdateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::LevelDataAssetDownloadUpdate>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_levelDataAssetDownloadUpdateEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OculusBeatmapDataAssetFileModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OculusBeatmapDataAssetFileModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OculusBeatmapDataAssetFileModel+LevelDownloadingData")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusBeatmapDataAssetFileModel_LevelDownloadingData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub levelId: *mut quest_hook::libil2cpp::Il2CppString,
    pub assetBundlePath: *mut quest_hook::libil2cpp::Il2CppString,
    pub downloadAssetBundleFileTCS: *mut crate::System::Threading::Tasks::TaskCompletionSource_1<
        crate::GlobalNamespace::GetAssetBundleFileResult,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        assetBundlePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (levelId, assetBundlePath))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        assetBundlePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (levelId, assetBundlePath))?;
        Ok(__cordl_ret.into())
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
