#[cfg(feature = "BeatmapDifficultyMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDifficultyMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDifficultyMethods")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapDifficultyMethods {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapDifficultyMethods";
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
#[cfg(feature = "BeatmapDifficultyMethods")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDifficultyMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDifficultyMethods")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDifficultyMethods {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDifficultyMethods")]
impl crate::GlobalNamespace::BeatmapDifficultyMethods {
    pub const kDefaultDifficultyNjs: f32 = 10f32;
    pub const kExpertDifficultyNjs: f32 = 12f32;
    pub const kExpertPlusDifficultyNjs: f32 = 16f32;
    pub const kFastNotesNjs: f32 = 20f32;
    pub fn DefaultNoteJumpMovementSpeed(
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::BeatmapDifficulty),
                        f32,
                        1usize,
                    >("DefaultNoteJumpMovementSpeed")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DefaultNoteJumpMovementSpeed", 1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (difficulty))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DefaultRating(
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::BeatmapDifficulty),
                        i32,
                        1usize,
                    >("DefaultRating")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DefaultRating", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (difficulty))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Name(
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::BeatmapDifficulty),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("Name")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Name",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked((), (difficulty))? };
        Ok(__cordl_ret.into())
    }
    pub fn NoteJumpMovementSpeed(
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
        noteJumpMovementSpeed: f32,
        fastNotes: bool,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::BeatmapDifficulty, f32, bool),
                        f32,
                        3usize,
                    >("NoteJumpMovementSpeed")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NoteJumpMovementSpeed", 3usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info
                .invoke_unchecked((), (difficulty, noteJumpMovementSpeed, fastNotes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShortName(
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::BeatmapDifficulty),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("ShortName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShortName", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked((), (difficulty))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDifficultyMethods")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDifficultyMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
