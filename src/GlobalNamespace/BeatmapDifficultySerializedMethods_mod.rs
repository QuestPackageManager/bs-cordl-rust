#[cfg(feature = "BeatmapDifficultySerializedMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDifficultySerializedMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDifficultySerializedMethods")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapDifficultySerializedMethods {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapDifficultySerializedMethods";
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
#[cfg(feature = "BeatmapDifficultySerializedMethods")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDifficultySerializedMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDifficultySerializedMethods")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDifficultySerializedMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDifficultySerializedMethods")]
impl crate::GlobalNamespace::BeatmapDifficultySerializedMethods {
    pub const kDifficultyEasySerializedName: &'static str = "Easy";
    pub const kDifficultyExpertPlusNameSerializedLegacy: &'static str = "Expert+";
    pub const kDifficultyExpertPlusSerializedName: &'static str = "ExpertPlus";
    pub const kDifficultyExpertSerializedName: &'static str = "Expert";
    pub const kDifficultyHardSerializedName: &'static str = "Hard";
    pub const kDifficultyNormalSerializedName: &'static str = "Normal";
    pub const kDifficultyUnknownSerializedName: &'static str = "Unknown";
    pub fn BeatmapDifficultyFromSerializedName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        difficulty: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::BeatmapDifficulty,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::GlobalNamespace::BeatmapDifficulty,
                            >,
                        ),
                        bool,
                        2usize,
                    >("BeatmapDifficultyFromSerializedName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "BeatmapDifficultyFromSerializedName", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (name, difficulty))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SerializedName(
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::BeatmapDifficulty),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("SerializedName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SerializedName", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (difficulty))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDifficultySerializedMethods")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDifficultySerializedMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
