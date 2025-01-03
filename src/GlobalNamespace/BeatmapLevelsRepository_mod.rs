#[cfg(feature = "BeatmapLevelsRepository")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelsRepository {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _beatmapLevelPacks: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::BeatmapLevelPack>,
    >,
    pub _idToBeatmapLevelPack: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::GlobalNamespace::BeatmapLevelPack,
        >,
    >,
    pub _idToBeatmapLevel: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::GlobalNamespace::BeatmapLevel,
        >,
    >,
    pub _beatmapLevelIdToBeatmapLevelPackId: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    >,
}
#[cfg(feature = "BeatmapLevelsRepository")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapLevelsRepository => ""
    ."BeatmapLevelsRepository"
);
#[cfg(feature = "BeatmapLevelsRepository")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLevelsRepository {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelsRepository")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapLevelsRepository {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelsRepository")]
impl crate::GlobalNamespace::BeatmapLevelsRepository {
    pub fn AddBeatmapLevel(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        songPackId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddBeatmapLevel", (beatmapLevel, songPackId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapLevelById(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevel,
        > = __cordl_object.invoke("GetBeatmapLevelById", (levelId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapLevelPackByBeatmapLevelId(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        > = __cordl_object.invoke("GetBeatmapLevelPackByBeatmapLevelId", (levelId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapLevelPackByPackId(
        &mut self,
        packId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        > = __cordl_object.invoke("GetBeatmapLevelPackByPackId", (packId))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        beatmapLevelPacks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::BeatmapLevelPack,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beatmapLevelPacks))?;
        Ok(__cordl_object.into())
    }
    pub fn TryGetBeatmapLevelById(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapLevel: quest_hook::libil2cpp::ByRefMut<
            *mut crate::GlobalNamespace::BeatmapLevel,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetBeatmapLevelById", (levelId, beatmapLevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        beatmapLevelPacks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::BeatmapLevelPack,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beatmapLevelPacks))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapLevelPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::GlobalNamespace::BeatmapLevelPack,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::GlobalNamespace::BeatmapLevelPack,
            >,
        > = __cordl_object.invoke("get_beatmapLevelPacks", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapLevelsRepository")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapLevelsRepository {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
