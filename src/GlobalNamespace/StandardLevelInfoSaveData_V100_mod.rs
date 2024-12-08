#[cfg(feature = "StandardLevelInfoSaveData_V100+DifficultyBeatmap")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardLevelInfoSaveData_V100_DifficultyBeatmap {
    __cordl_parent: crate::System::Object,
    pub _difficulty: *mut crate::System::String,
    pub _difficultyRank: i32,
    pub _beatmapFilename: *mut crate::System::String,
    pub _noteJumpMovementSpeed: f32,
    pub _noteJumpStartBeatOffset: i32,
}
#[cfg(feature = "StandardLevelInfoSaveData_V100+DifficultyBeatmap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::StandardLevelInfoSaveData_V100_DifficultyBeatmap => ""
    ."StandardLevelInfoSaveData_V100/DifficultyBeatmap"
);
#[cfg(feature = "StandardLevelInfoSaveData_V100+DifficultyBeatmap")]
impl std::ops::Deref
for crate::GlobalNamespace::StandardLevelInfoSaveData_V100_DifficultyBeatmap {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelInfoSaveData_V100+DifficultyBeatmap")]
impl std::ops::DerefMut
for crate::GlobalNamespace::StandardLevelInfoSaveData_V100_DifficultyBeatmap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelInfoSaveData_V100+DifficultyBeatmap")]
impl crate::GlobalNamespace::StandardLevelInfoSaveData_V100_DifficultyBeatmap {
    pub fn New(
        difficultyName: *mut crate::System::String,
        difficultyRank: i32,
        beatmapFilename: *mut crate::System::String,
        noteJumpMovementSpeed: f32,
        noteJumpStartBeatOffset: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    difficultyName,
                    difficultyRank,
                    beatmapFilename,
                    noteJumpMovementSpeed,
                    noteJumpStartBeatOffset,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        difficultyName: *mut crate::System::String,
        difficultyRank: i32,
        beatmapFilename: *mut crate::System::String,
        noteJumpMovementSpeed: f32,
        noteJumpStartBeatOffset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    difficultyName,
                    difficultyRank,
                    beatmapFilename,
                    noteJumpMovementSpeed,
                    noteJumpStartBeatOffset,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapFilename(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_beatmapFilename", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_difficulty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_difficulty", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_difficultyRank(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_difficultyRank", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_noteJumpMovementSpeed(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_noteJumpMovementSpeed", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_noteJumpStartBeatOffset(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_noteJumpStartBeatOffset", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "StandardLevelInfoSaveData_V100+DifficultyBeatmap")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StandardLevelInfoSaveData_V100_DifficultyBeatmap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "StandardLevelInfoSaveData_V100")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardLevelInfoSaveData_V100 {
    __cordl_parent: crate::System::Object,
    pub _version: *mut crate::System::String,
    pub _songName: *mut crate::System::String,
    pub _songSubName: *mut crate::System::String,
    pub _songAuthorName: *mut crate::System::String,
    pub _levelAuthorName: *mut crate::System::String,
    pub _beatsPerMinute: f32,
    pub _songTimeOffset: f32,
    pub _shuffle: f32,
    pub _shufflePeriod: f32,
    pub _previewStartTime: f32,
    pub _previewDuration: f32,
    pub _songFilename: *mut crate::System::String,
    pub _coverImageFilename: *mut crate::System::String,
    pub _environmentName: *mut crate::System::String,
    pub _difficultyBeatmaps: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::StandardLevelInfoSaveData_V100_DifficultyBeatmap,
    >,
}
#[cfg(feature = "StandardLevelInfoSaveData_V100")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::StandardLevelInfoSaveData_V100
    => ""."StandardLevelInfoSaveData_V100"
);
#[cfg(feature = "StandardLevelInfoSaveData_V100")]
impl std::ops::Deref for crate::GlobalNamespace::StandardLevelInfoSaveData_V100 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelInfoSaveData_V100")]
impl std::ops::DerefMut for crate::GlobalNamespace::StandardLevelInfoSaveData_V100 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelInfoSaveData_V100")]
impl crate::GlobalNamespace::StandardLevelInfoSaveData_V100 {
    pub const kCurrentVersion: &'static str = "1.0.0";
    #[cfg(feature = "StandardLevelInfoSaveData_V100+DifficultyBeatmap")]
    pub type DifficultyBeatmap = crate::GlobalNamespace::StandardLevelInfoSaveData_V100_DifficultyBeatmap;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_beatsPerMinute(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_beatsPerMinute", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_coverImageFilename(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_coverImageFilename", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_difficultyBeatmaps(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::StandardLevelInfoSaveData_V100_DifficultyBeatmap,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::StandardLevelInfoSaveData_V100_DifficultyBeatmap,
        > = __cordl_object.invoke("get_difficultyBeatmaps", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_environmentName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_environmentName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_levelAuthorName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_levelAuthorName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_previewDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_previewDuration", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_previewStartTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_previewStartTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_shuffle(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_shuffle", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_shufflePeriod(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_shufflePeriod", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_songAuthorName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_songAuthorName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_songFilename(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_songFilename", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_songName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_songName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_songSubName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_songSubName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_songTimeOffset(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_songTimeOffset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_version", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "StandardLevelInfoSaveData_V100")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StandardLevelInfoSaveData_V100 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
