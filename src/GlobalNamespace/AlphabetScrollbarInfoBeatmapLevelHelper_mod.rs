#[cfg(feature = "AlphabetScrollbarInfoBeatmapLevelHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct AlphabetScrollbarInfoBeatmapLevelHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "AlphabetScrollbarInfoBeatmapLevelHelper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::AlphabetScrollbarInfoBeatmapLevelHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "AlphabetScrollbarInfoBeatmapLevelHelper";
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
#[cfg(feature = "AlphabetScrollbarInfoBeatmapLevelHelper")]
impl std::ops::Deref
for crate::GlobalNamespace::AlphabetScrollbarInfoBeatmapLevelHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AlphabetScrollbarInfoBeatmapLevelHelper")]
impl std::ops::DerefMut
for crate::GlobalNamespace::AlphabetScrollbarInfoBeatmapLevelHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AlphabetScrollbarInfoBeatmapLevelHelper")]
impl crate::GlobalNamespace::AlphabetScrollbarInfoBeatmapLevelHelper {
    pub const kFirstAlphabet: &'static str = "A";
    pub const kLastAlphabet: &'static str = "Z";
    pub const kMaxCharactersCount: i32 = 28i32;
    pub const kNonAlphabetChar: char = '#';
    pub fn CreateData(
        beatmapLevels: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
            >,
        >,
        sortBeatmapLevels: bool,
        sortedBeatmapLevels: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IReadOnlyList_1<
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::AlphabetScrollInfo_Data,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IReadOnlyList_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::BeatmapLevel,
                                    >,
                                >,
                            >,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Collections::Generic::IReadOnlyList_1<
                                        quest_hook::libil2cpp::Gc<
                                            crate::GlobalNamespace::BeatmapLevel,
                                        >,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IReadOnlyList_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::AlphabetScrollInfo_Data,
                                >,
                            >,
                        >,
                        3usize,
                    >("CreateData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateData", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::AlphabetScrollInfo_Data,
                >,
            >,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (beatmapLevels, sortBeatmapLevels, sortedBeatmapLevels),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsNumericOrSpecial(
        comparedChar: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        bool,
                        1usize,
                    >("IsNumericOrSpecial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsNumericOrSpecial", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (comparedChar))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AlphabetScrollbarInfoBeatmapLevelHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AlphabetScrollbarInfoBeatmapLevelHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
