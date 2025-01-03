#[cfg(feature = "BeatmapLevelPack")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelPack {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub packID: *mut quest_hook::libil2cpp::Il2CppString,
    pub packName: *mut quest_hook::libil2cpp::Il2CppString,
    pub shortPackName: *mut quest_hook::libil2cpp::Il2CppString,
    pub coverImage: *mut crate::UnityEngine::Sprite,
    pub smallCoverImage: *mut crate::UnityEngine::Sprite,
    pub packBuyOption: crate::GlobalNamespace::PackBuyOption,
    pub contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
    pub _beatmapLevels: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::BeatmapLevel,
    >,
    pub _additionalBeatmapLevels: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::BeatmapLevel,
    >,
    pub _allBeatmapLevels: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::BeatmapLevel,
    >,
}
#[cfg(feature = "BeatmapLevelPack")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapLevelPack => ""
    ."BeatmapLevelPack"
);
#[cfg(feature = "BeatmapLevelPack")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLevelPack {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub const kFilteredLevelPackId: &'static str = "filtered_pack_id";
    pub fn AddAdditionalBeatmapLevel(
        &mut self,
        levelToAdd: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAdditionalBeatmapLevel", (levelToAdd))?;
        Ok(__cordl_ret.into())
    }
    pub fn AllBeatmapLevels(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::BeatmapLevel,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::BeatmapLevel,
            >,
        > = __cordl_object.invoke("AllBeatmapLevels", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearAdditionalBeatmapLevels(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearAdditionalBeatmapLevels", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateLevelPackForFiltering(
        beatmapLevels: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::BeatmapLevel>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateLevelPackForFiltering", (beatmapLevels))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        packID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        packName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        shortPackName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        coverImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        smallCoverImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        packBuyOption: crate::GlobalNamespace::PackBuyOption,
        beatmapLevels: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::BeatmapLevel>,
        >,
        contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
                    packBuyOption,
                    beatmapLevels,
                    contentRating,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn SetAdditionalBeatmapLevels(
        &mut self,
        additionalBeatmapLevels: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::BeatmapLevel,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAdditionalBeatmapLevels", (additionalBeatmapLevels))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        packID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        packName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        shortPackName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        coverImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        smallCoverImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        packBuyOption: crate::GlobalNamespace::PackBuyOption,
        beatmapLevels: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::BeatmapLevel>,
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
                    packBuyOption,
                    beatmapLevels,
                    contentRating,
                ),
            )?;
        Ok(__cordl_ret.into())
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
