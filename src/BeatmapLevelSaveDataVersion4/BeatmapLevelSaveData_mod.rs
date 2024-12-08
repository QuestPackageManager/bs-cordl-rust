#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+AudioData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BeatmapLevelSaveData_AudioData {
    pub songFilename: *mut crate::System::String,
    pub songDuration: f32,
    pub audioDataFilename: *mut crate::System::String,
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
    pub mappers: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub lighters: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
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
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelSaveData {
    __cordl_parent: crate::System::Object,
    pub version: *mut crate::System::String,
    pub song: crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_SongData,
    pub audio: crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_AudioData,
    pub songPreviewFilename: *mut crate::System::String,
    pub coverImageFilename: *mut crate::System::String,
    pub environmentNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
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
    type Target = crate::System::Object;
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
    pub const kCurrentVersion: &'static str = "4.0.0";
    #[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+AudioData")]
    pub type AudioData = crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_AudioData;
    #[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+ColorScheme")]
    pub type ColorScheme = crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_ColorScheme;
    #[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+SongData")]
    pub type SongData = crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_SongData;
    #[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+BeatmapAuthors")]
    pub type BeatmapAuthors = crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_BeatmapAuthors;
    #[cfg(
        feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+DifficultyBeatmap"
    )]
    pub type DifficultyBeatmap = crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_DifficultyBeatmap;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+ColorScheme")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelSaveData_ColorScheme {
    __cordl_parent: crate::System::Object,
    pub colorSchemeName: *mut crate::System::String,
    pub saberAColor: *mut crate::System::String,
    pub saberBColor: *mut crate::System::String,
    pub obstaclesColor: *mut crate::System::String,
    pub environmentColor0: *mut crate::System::String,
    pub environmentColor1: *mut crate::System::String,
    pub environmentColor0Boost: *mut crate::System::String,
    pub environmentColor1Boost: *mut crate::System::String,
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
    type Target = crate::System::Object;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    __cordl_parent: crate::System::Object,
    pub characteristic: *mut crate::System::String,
    pub difficulty: *mut crate::System::String,
    pub beatmapAuthors: crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_BeatmapAuthors,
    pub environmentNameIdx: i32,
    pub beatmapColorSchemeIdx: i32,
    pub noteJumpMovementSpeed: f32,
    pub noteJumpStartBeatOffset: f32,
    pub lightshowDataFilename: *mut crate::System::String,
    pub beatmapDataFilename: *mut crate::System::String,
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
    type Target = crate::System::Object;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub title: *mut crate::System::String,
    pub subTitle: *mut crate::System::String,
    pub author: *mut crate::System::String,
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
