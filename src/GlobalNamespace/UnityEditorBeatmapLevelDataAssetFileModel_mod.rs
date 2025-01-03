#[cfg(feature = "UnityEditorBeatmapLevelDataAssetFileModel")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityEditorBeatmapLevelDataAssetFileModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEditorBeatmapLevelDataAssetFileModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::UnityEditorBeatmapLevelDataAssetFileModel => ""
    ."UnityEditorBeatmapLevelDataAssetFileModel"
);
#[cfg(feature = "UnityEditorBeatmapLevelDataAssetFileModel")]
impl std::ops::Deref
for crate::GlobalNamespace::UnityEditorBeatmapLevelDataAssetFileModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEditorBeatmapLevelDataAssetFileModel")]
impl std::ops::DerefMut
for crate::GlobalNamespace::UnityEditorBeatmapLevelDataAssetFileModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEditorBeatmapLevelDataAssetFileModel")]
impl crate::GlobalNamespace::UnityEditorBeatmapLevelDataAssetFileModel {
    pub fn LoadAllAssetsFromAssetDatabase<T>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadAllAssetsFromAssetDatabase", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadBeatmapLevelDataFromAssetDatabase(
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelDataSO>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelDataSO,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadBeatmapLevelDataFromAssetDatabase", (levelId))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadBeatmapLevelFromAssetDatabase(
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelSO>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelSO,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadBeatmapLevelFromAssetDatabase", (levelId))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadBeatmapLevelPackFromAssetDatabase(
        packId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPackSO>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPackSO,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadBeatmapLevelPackFromAssetDatabase", (packId))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadUniqueAssetFromAssetDatabase<T>(
        filename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadUniqueAssetFromAssetDatabase", (filename))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEditorBeatmapLevelDataAssetFileModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::UnityEditorBeatmapLevelDataAssetFileModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
