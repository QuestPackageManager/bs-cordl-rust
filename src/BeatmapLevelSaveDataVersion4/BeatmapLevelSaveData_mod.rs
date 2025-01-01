#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelSaveData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub version: *mut quest_hook::libil2cpp::Il2CppString,
    pub song: crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_SongData,
    pub audio: crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_AudioData,
    pub songPreviewFilename: *mut quest_hook::libil2cpp::Il2CppString,
    pub coverImageFilename: *mut quest_hook::libil2cpp::Il2CppString,
    pub environmentNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub colorSchemes: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_ColorScheme,
    >,
    pub difficultyBeatmaps: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_DifficultyBeatmap,
    >,
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData =>
    "BeatmapLevelSaveDataVersion4"."BeatmapLevelSaveData"
);
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData")]
impl std::ops::Deref for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData")]
impl std::ops::DerefMut for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData")]
impl crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData {
    pub const kCurrentVersion: &'static str = "4.0.1";
    #[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+AudioData")]
    pub type AudioData = crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_AudioData;
    #[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+BeatmapAuthors")]
    pub type BeatmapAuthors = crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_BeatmapAuthors;
    #[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+ColorScheme")]
    pub type ColorScheme = crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_ColorScheme;
    #[cfg(
        feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+DifficultyBeatmap"
    )]
    pub type DifficultyBeatmap = crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_DifficultyBeatmap;
    #[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+SongData")]
    pub type SongData = crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_SongData;
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
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+AudioData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BeatmapLevelSaveData_AudioData {
    pub songFilename: *mut quest_hook::libil2cpp::Il2CppString,
    pub songDuration: f32,
    pub audioDataFilename: *mut quest_hook::libil2cpp::Il2CppString,
    pub bpm: f32,
    pub lufs: f32,
    pub previewStartTime: f32,
    pub previewDuration: f32,
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+AudioData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_AudioData =>
    "BeatmapLevelSaveDataVersion4"."BeatmapLevelSaveData/AudioData"
);
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+AudioData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_AudioData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+AudioData")]
impl crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_AudioData {}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+BeatmapAuthors")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BeatmapLevelSaveData_BeatmapAuthors {
    pub mappers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub lighters: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+BeatmapAuthors")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_BeatmapAuthors =>
    "BeatmapLevelSaveDataVersion4"."BeatmapLevelSaveData/BeatmapAuthors"
);
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+BeatmapAuthors")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_BeatmapAuthors {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+BeatmapAuthors")]
impl crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_BeatmapAuthors {}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+ColorScheme")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelSaveData_ColorScheme {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub colorSchemeName: *mut quest_hook::libil2cpp::Il2CppString,
    pub overrideNotes: bool,
    pub saberAColor: *mut quest_hook::libil2cpp::Il2CppString,
    pub saberBColor: *mut quest_hook::libil2cpp::Il2CppString,
    pub obstaclesColor: *mut quest_hook::libil2cpp::Il2CppString,
    pub overrideLights: bool,
    pub environmentColor0: *mut quest_hook::libil2cpp::Il2CppString,
    pub environmentColor1: *mut quest_hook::libil2cpp::Il2CppString,
    pub environmentColor0Boost: *mut quest_hook::libil2cpp::Il2CppString,
    pub environmentColor1Boost: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+ColorScheme")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_ColorScheme =>
    "BeatmapLevelSaveDataVersion4"."BeatmapLevelSaveData/ColorScheme"
);
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+ColorScheme")]
impl std::ops::Deref
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_ColorScheme {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+ColorScheme")]
impl std::ops::DerefMut
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_ColorScheme {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+ColorScheme")]
impl crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_ColorScheme {
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
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+ColorScheme")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_ColorScheme {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+DifficultyBeatmap")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelSaveData_DifficultyBeatmap {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub characteristic: *mut quest_hook::libil2cpp::Il2CppString,
    pub difficulty: *mut quest_hook::libil2cpp::Il2CppString,
    pub beatmapAuthors: crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_BeatmapAuthors,
    pub environmentNameIdx: i32,
    pub beatmapColorSchemeIdx: i32,
    pub noteJumpMovementSpeed: f32,
    pub noteJumpStartBeatOffset: f32,
    pub lightshowDataFilename: *mut quest_hook::libil2cpp::Il2CppString,
    pub beatmapDataFilename: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+DifficultyBeatmap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_DifficultyBeatmap =>
    "BeatmapLevelSaveDataVersion4"."BeatmapLevelSaveData/DifficultyBeatmap"
);
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+DifficultyBeatmap")]
impl std::ops::Deref
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_DifficultyBeatmap {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+DifficultyBeatmap")]
impl std::ops::DerefMut
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_DifficultyBeatmap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+DifficultyBeatmap")]
impl crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_DifficultyBeatmap {
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
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+DifficultyBeatmap")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_DifficultyBeatmap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+SongData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BeatmapLevelSaveData_SongData {
    pub title: *mut quest_hook::libil2cpp::Il2CppString,
    pub subTitle: *mut quest_hook::libil2cpp::Il2CppString,
    pub author: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+SongData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_SongData =>
    "BeatmapLevelSaveDataVersion4"."BeatmapLevelSaveData/SongData"
);
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+SongData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_SongData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+SongData")]
impl crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_SongData {}
