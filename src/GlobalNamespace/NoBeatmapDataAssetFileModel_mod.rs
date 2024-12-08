#[cfg(feature = "NoBeatmapDataAssetFileModel")]
#[repr(C)]
#[derive(Debug)]
pub struct NoBeatmapDataAssetFileModel {
    __cordl_parent: crate::System::Object,
    pub levelDataAssetDownloadUpdateEvent: *mut crate::System::Action_1<
        crate::GlobalNamespace::LevelDataAssetDownloadUpdate,
    >,
}
#[cfg(feature = "NoBeatmapDataAssetFileModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoBeatmapDataAssetFileModel =>
    ""."NoBeatmapDataAssetFileModel"
);
#[cfg(feature = "NoBeatmapDataAssetFileModel")]
impl std::ops::Deref for crate::GlobalNamespace::NoBeatmapDataAssetFileModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoBeatmapDataAssetFileModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoBeatmapDataAssetFileModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoBeatmapDataAssetFileModel")]
impl crate::GlobalNamespace::NoBeatmapDataAssetFileModel {
    #[cfg(
        feature = "NoBeatmapDataAssetFileModel+_GetAssetBundleFileForBeatmapLevelAsync_d__3"
    )]
    pub type _GetAssetBundleFileForBeatmapLevelAsync_d__3 = crate::GlobalNamespace::NoBeatmapDataAssetFileModel__GetAssetBundleFileForBeatmapLevelAsync_d__3;
    pub fn GetAssetBundleFileForBeatmapLevelAsync(
        &mut self,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::GetAssetBundleFileResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::GetAssetBundleFileResult,
        > = __cordl_object
            .invoke(
                "GetAssetBundleFileForBeatmapLevelAsync",
                (beatmapLevel, beatmapLevelDataVersion, cancellationToken),
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
    pub fn TryDeleteAssetBundleFileForBeatmapLevelAsync(
        &mut self,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_levelDataAssetDownloadUpdateEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::GlobalNamespace::LevelDataAssetDownloadUpdate,
        >,
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
        value: *mut crate::System::Action_1<
            crate::GlobalNamespace::LevelDataAssetDownloadUpdate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_levelDataAssetDownloadUpdateEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "NoBeatmapDataAssetFileModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoBeatmapDataAssetFileModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
