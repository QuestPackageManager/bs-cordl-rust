#[cfg(feature = "BeatmapEditorSceneSetupData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapEditorSceneSetupData {
    __cordl_parent: SceneSetupData,
    pub _levelDirPath: *mut crate::System::String,
    pub _levelAssetPath: *mut crate::System::String,
}
#[cfg(feature = "BeatmapEditorSceneSetupData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapEditorSceneSetupData => ""
    ."BeatmapEditorSceneSetupData"
);
#[cfg(feature = "BeatmapEditorSceneSetupData")]
impl std::ops::Deref for BeatmapEditorSceneSetupData {
    type Target = SceneSetupData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEditorSceneSetupData")]
impl std::ops::DerefMut for BeatmapEditorSceneSetupData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEditorSceneSetupData")]
impl BeatmapEditorSceneSetupData {
    pub fn New(
        levelDirPath: *mut crate::System::String,
        levelAssetPath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (levelDirPath, levelAssetPath))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        levelDirPath: *mut crate::System::String,
        levelAssetPath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (levelDirPath, levelAssetPath))?;
        Ok(__cordl_ret)
    }
    pub fn get_levelAssetPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_levelAssetPath", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_levelDirPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_levelDirPath", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapEditorSceneSetupData")]
impl quest_hook::libil2cpp::ObjectType for BeatmapEditorSceneSetupData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
