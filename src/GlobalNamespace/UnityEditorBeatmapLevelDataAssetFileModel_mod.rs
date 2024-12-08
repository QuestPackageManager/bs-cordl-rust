#[cfg(feature = "UnityEditorBeatmapLevelDataAssetFileModel")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityEditorBeatmapLevelDataAssetFileModel {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEditorBeatmapLevelDataAssetFileModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for UnityEditorBeatmapLevelDataAssetFileModel => ""
    ."UnityEditorBeatmapLevelDataAssetFileModel"
);
#[cfg(feature = "UnityEditorBeatmapLevelDataAssetFileModel")]
impl std::ops::Deref for UnityEditorBeatmapLevelDataAssetFileModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEditorBeatmapLevelDataAssetFileModel")]
impl std::ops::DerefMut for UnityEditorBeatmapLevelDataAssetFileModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEditorBeatmapLevelDataAssetFileModel")]
impl UnityEditorBeatmapLevelDataAssetFileModel {}
#[cfg(feature = "UnityEditorBeatmapLevelDataAssetFileModel")]
impl quest_hook::libil2cpp::ObjectType for UnityEditorBeatmapLevelDataAssetFileModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
