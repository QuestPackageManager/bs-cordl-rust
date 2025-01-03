#[cfg(feature = "BeatmapDataAssetsModel")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataAssetsModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDataAssetsModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapDataAssetsModel => ""
    ."BeatmapDataAssetsModel"
);
#[cfg(feature = "BeatmapDataAssetsModel")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDataAssetsModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataAssetsModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDataAssetsModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataAssetsModel")]
impl crate::GlobalNamespace::BeatmapDataAssetsModel {
    pub fn AssetBundleStreamingAssetsPathForAssetBundleName(
        assetBundleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AssetBundleStreamingAssetsPathForAssetBundleName",
                (assetBundleName),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetBundleNameForBeatmapLevel(
        levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        version: crate::GlobalNamespace::BeatmapLevelDataVersion,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssetBundleNameForBeatmapLevel", (levelID, version))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetBundleNameWithVersionForBeatmapLevelData(
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        bundleVersion: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetAssetBundleNameWithVersionForBeatmapLevelData",
                (levelId, dataVersion, bundleVersion),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetBundleStreamingAssetsPathForBeatmapLevelId(
        levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        version: crate::GlobalNamespace::BeatmapLevelDataVersion,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetAssetBundleStreamingAssetsPathForBeatmapLevelId",
                (levelID, version),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapLevelAssetFilenameForBeatmapLevel(
        levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBeatmapLevelAssetFilenameForBeatmapLevel", (levelID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapLevelDataAssetFilenameForBeatmapLevel(
        levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBeatmapLevelDataAssetFilenameForBeatmapLevel", (levelID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapLevelDataAssetNameForBeatmapLevel(
        levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBeatmapLevelDataAssetNameForBeatmapLevel", (levelID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapLevelPackAssetFilename(
        packId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBeatmapLevelPackAssetFilename", (packId))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataAssetsModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDataAssetsModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
