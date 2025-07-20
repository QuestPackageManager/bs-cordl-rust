#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelSaveData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub song: crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_SongData,
    pub audio: crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_AudioData,
    pub songPreviewFilename: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub coverImageFilename: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub environmentNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub colorSchemes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_ColorScheme,
            >,
        >,
    >,
    pub difficultyBeatmaps: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_DifficultyBeatmap,
            >,
        >,
    >,
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapLevelSaveDataVersion4";
    const CLASS_NAME: &'static str = "BeatmapLevelSaveData";
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
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData")]
impl std::ops::Deref for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData")]
impl std::ops::DerefMut for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeatmapLevelSaveData_AudioData {
    pub songFilename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub songDuration: f32,
    pub audioDataFilename: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub bpm: f32,
    pub lufs: f32,
    pub previewStartTime: f32,
    pub previewDuration: f32,
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+AudioData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_AudioData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatmapLevelSaveDataVersion4";
    const CLASS_NAME: &'static str = "BeatmapLevelSaveData/AudioData";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+AudioData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_AudioData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+AudioData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_AudioData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+AudioData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_AudioData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+AudioData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_AudioData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeatmapLevelSaveData_BeatmapAuthors {
    pub mappers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub lighters: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+BeatmapAuthors")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_BeatmapAuthors {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatmapLevelSaveDataVersion4";
    const CLASS_NAME: &'static str = "BeatmapLevelSaveData/BeatmapAuthors";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+BeatmapAuthors")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_BeatmapAuthors {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+BeatmapAuthors")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_BeatmapAuthors {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+BeatmapAuthors")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_BeatmapAuthors {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+BeatmapAuthors")]
unsafe impl quest_hook::libil2cpp::Return
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_BeatmapAuthors {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
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
    pub colorSchemeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub overrideNotes: bool,
    pub saberAColor: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub saberBColor: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub obstaclesColor: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub overrideLights: bool,
    pub environmentColor0: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub environmentColor1: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub environmentColor0Boost: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub environmentColor1Boost: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+ColorScheme")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_ColorScheme {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapLevelSaveDataVersion4";
    const CLASS_NAME: &'static str = "BeatmapLevelSaveData/ColorScheme";
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
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+ColorScheme")]
impl std::ops::Deref
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_ColorScheme {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+ColorScheme")]
impl std::ops::DerefMut
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_ColorScheme {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
    pub characteristic: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub difficulty: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub beatmapAuthors: crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_BeatmapAuthors,
    pub environmentNameIdx: i32,
    pub beatmapColorSchemeIdx: i32,
    pub noteJumpMovementSpeed: f32,
    pub noteJumpStartBeatOffset: f32,
    pub lightshowDataFilename: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub beatmapDataFilename: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+DifficultyBeatmap")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_DifficultyBeatmap {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapLevelSaveDataVersion4";
    const CLASS_NAME: &'static str = "BeatmapLevelSaveData/DifficultyBeatmap";
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
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+DifficultyBeatmap")]
impl std::ops::Deref
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_DifficultyBeatmap {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+DifficultyBeatmap")]
impl std::ops::DerefMut
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_DifficultyBeatmap {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeatmapLevelSaveData_SongData {
    pub title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub subTitle: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub author: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+SongData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_SongData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatmapLevelSaveDataVersion4";
    const CLASS_NAME: &'static str = "BeatmapLevelSaveData/SongData";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+SongData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_SongData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+SongData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_SongData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+SongData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_SongData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+BeatmapLevelSaveData+SongData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::BeatmapLevelSaveDataVersion4::BeatmapLevelSaveData_SongData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
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
