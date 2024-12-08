#[cfg(feature = "BeatmapLevelsRepository")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelsRepository {
    __cordl_parent: crate::System::Object,
    pub _beatmapLevelPacks: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut BeatmapLevelPack,
    >,
    pub _idToBeatmapLevelPack: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut BeatmapLevelPack,
    >,
    pub _idToBeatmapLevel: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut BeatmapLevel,
    >,
    pub _beatmapLevelIdToBeatmapLevelPackId: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::System::String,
    >,
}
#[cfg(feature = "BeatmapLevelsRepository")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapLevelsRepository => ""."BeatmapLevelsRepository"
);
#[cfg(feature = "BeatmapLevelsRepository")]
impl std::ops::Deref for BeatmapLevelsRepository {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelsRepository")]
impl std::ops::DerefMut for BeatmapLevelsRepository {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelsRepository")]
impl BeatmapLevelsRepository {
    pub fn GetBeatmapLevelById(
        &mut self,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapLevel> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapLevel = __cordl_object
            .invoke("GetBeatmapLevelById", (levelId))?;
        Ok(__cordl_ret)
    }
    pub fn GetBeatmapLevelPackByBeatmapLevelId(
        &mut self,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapLevelPack> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapLevelPack = __cordl_object
            .invoke("GetBeatmapLevelPackByBeatmapLevelId", (levelId))?;
        Ok(__cordl_ret)
    }
    pub fn GetBeatmapLevelPackByPackId(
        &mut self,
        packId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapLevelPack> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapLevelPack = __cordl_object
            .invoke("GetBeatmapLevelPackByPackId", (packId))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        beatmapLevelPacks: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut BeatmapLevelPack,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beatmapLevelPacks))?;
        Ok(__cordl_object)
    }
    pub fn TryGetBeatmapLevelById(
        &mut self,
        levelId: *mut crate::System::String,
        beatmapLevel: quest_hook::libil2cpp::ByRefMut<*mut BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetBeatmapLevelById", (levelId, beatmapLevel))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        beatmapLevelPacks: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut BeatmapLevelPack,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beatmapLevelPacks))?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapLevelPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<*mut BeatmapLevelPack>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut BeatmapLevelPack,
        > = __cordl_object.invoke("get_beatmapLevelPacks", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapLevelsRepository")]
impl quest_hook::libil2cpp::ObjectType for BeatmapLevelsRepository {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
