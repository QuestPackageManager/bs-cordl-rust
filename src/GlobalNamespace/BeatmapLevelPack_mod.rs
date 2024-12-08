#[cfg(feature = "BeatmapLevelPack")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelPack {
    __cordl_parent: crate::System::Object,
    pub packID: *mut crate::System::String,
    pub packName: *mut crate::System::String,
    pub shortPackName: *mut crate::System::String,
    pub coverImage: *mut crate::UnityEngine::Sprite,
    pub smallCoverImage: *mut crate::UnityEngine::Sprite,
    pub beatmapLevels: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::BeatmapLevel,
    >,
    pub contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
}
#[cfg(feature = "BeatmapLevelPack")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapLevelPack => ""
    ."BeatmapLevelPack"
);
#[cfg(feature = "BeatmapLevelPack")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLevelPack {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelPack")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapLevelPack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelPack")]
impl crate::GlobalNamespace::BeatmapLevelPack {
    pub fn New(
        packID: *mut crate::System::String,
        packName: *mut crate::System::String,
        shortPackName: *mut crate::System::String,
        coverImage: *mut crate::UnityEngine::Sprite,
        smallCoverImage: *mut crate::UnityEngine::Sprite,
        beatmapLevels: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::BeatmapLevel,
        >,
        contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    packID,
                    packName,
                    shortPackName,
                    coverImage,
                    smallCoverImage,
                    beatmapLevels,
                    contentRating,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        packID: *mut crate::System::String,
        packName: *mut crate::System::String,
        shortPackName: *mut crate::System::String,
        coverImage: *mut crate::UnityEngine::Sprite,
        smallCoverImage: *mut crate::UnityEngine::Sprite,
        beatmapLevels: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::BeatmapLevel,
        >,
        contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    packID,
                    packName,
                    shortPackName,
                    coverImage,
                    smallCoverImage,
                    beatmapLevels,
                    contentRating,
                ),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapLevelPack")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatmapLevelPack {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
