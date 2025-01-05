#[cfg(feature = "BeatmapEditorSceneSetupData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapEditorSceneSetupData {
    __cordl_parent: crate::GlobalNamespace::SceneSetupData,
    pub _levelDirPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _levelAssetPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "BeatmapEditorSceneSetupData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapEditorSceneSetupData =>
    ""."BeatmapEditorSceneSetupData"
);
#[cfg(feature = "BeatmapEditorSceneSetupData")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapEditorSceneSetupData {
    type Target = crate::GlobalNamespace::SceneSetupData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEditorSceneSetupData")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapEditorSceneSetupData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEditorSceneSetupData")]
impl crate::GlobalNamespace::BeatmapEditorSceneSetupData {
    pub fn New(
        levelDirPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        levelAssetPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (levelDirPath, levelAssetPath))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        levelDirPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        levelAssetPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (levelDirPath, levelAssetPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_levelAssetPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_levelAssetPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_levelDirPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_levelDirPath", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapEditorSceneSetupData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapEditorSceneSetupData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
