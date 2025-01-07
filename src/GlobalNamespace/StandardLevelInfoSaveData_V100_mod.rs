#[cfg(feature = "StandardLevelInfoSaveData_V100")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardLevelInfoSaveData_V100 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _songName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _songSubName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _songAuthorName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _levelAuthorName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _beatsPerMinute: f32,
    pub _songTimeOffset: f32,
    pub _shuffle: f32,
    pub _shufflePeriod: f32,
    pub _previewStartTime: f32,
    pub _previewDuration: f32,
    pub _songFilename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _coverImageFilename: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _environmentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _difficultyBeatmaps: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::StandardLevelInfoSaveData_V100_DifficultyBeatmap,
            >,
        >,
    >,
}
#[cfg(feature = "StandardLevelInfoSaveData_V100")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::StandardLevelInfoSaveData_V100 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "StandardLevelInfoSaveData_V100";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "StandardLevelInfoSaveData_V100")]
impl std::ops::Deref for crate::GlobalNamespace::StandardLevelInfoSaveData_V100 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beatsPerMinute(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_beatsPerMinute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_coverImageFilename(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_coverImageFilename", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_difficultyBeatmaps(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::StandardLevelInfoSaveData_V100_DifficultyBeatmap,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::StandardLevelInfoSaveData_V100_DifficultyBeatmap,
                >,
            >,
        > = __cordl_object.invoke("get_difficultyBeatmaps", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_environmentName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_environmentName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_levelAuthorName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_levelAuthorName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_previewDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_previewDuration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_previewStartTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_previewStartTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_shuffle(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_shuffle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_shufflePeriod(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_shufflePeriod", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songAuthorName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_songAuthorName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songFilename(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_songFilename", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_songName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songSubName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_songSubName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songTimeOffset(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_songTimeOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_version", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "StandardLevelInfoSaveData_V100+DifficultyBeatmap")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardLevelInfoSaveData_V100_DifficultyBeatmap {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _difficulty: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _difficultyRank: i32,
    pub _beatmapFilename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _noteJumpMovementSpeed: f32,
    pub _noteJumpStartBeatOffset: i32,
}
#[cfg(feature = "StandardLevelInfoSaveData_V100+DifficultyBeatmap")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::StandardLevelInfoSaveData_V100_DifficultyBeatmap {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "DifficultyBeatmap";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "StandardLevelInfoSaveData_V100+DifficultyBeatmap")]
impl std::ops::Deref
for crate::GlobalNamespace::StandardLevelInfoSaveData_V100_DifficultyBeatmap {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        difficultyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        difficultyRank: i32,
        beatmapFilename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        noteJumpMovementSpeed: f32,
        noteJumpStartBeatOffset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        difficultyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        difficultyRank: i32,
        beatmapFilename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapFilename(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_beatmapFilename", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_difficulty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_difficulty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_difficultyRank(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_difficultyRank", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noteJumpMovementSpeed(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_noteJumpMovementSpeed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noteJumpStartBeatOffset(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_noteJumpStartBeatOffset", ())?;
        Ok(__cordl_ret.into())
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
