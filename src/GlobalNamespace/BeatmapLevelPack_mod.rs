#[cfg(feature = "BeatmapLevelPack")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelPack {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub packID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub packName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub shortPackName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub coverImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub smallCoverImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub packBuyOption: crate::GlobalNamespace::PackBuyOption,
    pub contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
    pub _beatmapLevels: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        >,
    >,
    pub _additionalBeatmapLevels: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        >,
    >,
    pub _allBeatmapLevels: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        >,
    >,
}
#[cfg(feature = "BeatmapLevelPack")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::BeatmapLevelPack {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapLevelPack";
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
#[cfg(feature = "BeatmapLevelPack")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLevelPack {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelPack")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapLevelPack {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelPack")]
impl crate::GlobalNamespace::BeatmapLevelPack {
    pub const kFilteredLevelPackId: &'static str = "filtered_pack_id";
    pub fn AddAdditionalBeatmapLevel(
        &mut self,
        levelToAdd: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapLevel,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("AddAdditionalBeatmapLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddAdditionalBeatmapLevel", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (levelToAdd))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AllBeatmapLevels(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BeatmapLevel,
                                >,
                            >,
                        >,
                        0usize,
                    >("AllBeatmapLevels")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AllBeatmapLevels", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ClearAdditionalBeatmapLevels(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("ClearAdditionalBeatmapLevels")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearAdditionalBeatmapLevels", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateLevelPackForFiltering(
        beatmapLevels: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BeatmapLevel,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapLevelPack,
                        >,
                        1usize,
                    >("CreateLevelPackForFiltering")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateLevelPackForFiltering", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        > = unsafe { method.invoke_unchecked((), (beatmapLevels))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        packID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        packName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        shortPackName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        coverImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        smallCoverImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        packBuyOption: crate::GlobalNamespace::PackBuyOption,
        beatmapLevels: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
            >,
        >,
        contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    packID,
                    packName,
                    shortPackName,
                    coverImage,
                    smallCoverImage,
                    packBuyOption,
                    beatmapLevels,
                    contentRating,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn SetAdditionalBeatmapLevels(
        &mut self,
        additionalBeatmapLevels: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BeatmapLevel,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetAdditionalBeatmapLevels")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetAdditionalBeatmapLevels", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (additionalBeatmapLevels))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        packID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        packName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        shortPackName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        coverImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        smallCoverImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        packBuyOption: crate::GlobalNamespace::PackBuyOption,
        beatmapLevels: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
            >,
        >,
        contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                            crate::GlobalNamespace::PackBuyOption,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::BeatmapLevel,
                                    >,
                                >,
                            >,
                            crate::GlobalNamespace::PlayerSensitivityFlag,
                        ),
                        quest_hook::libil2cpp::Void,
                        8usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        packID,
                        packName,
                        shortPackName,
                        coverImage,
                        smallCoverImage,
                        packBuyOption,
                        beatmapLevels,
                        contentRating,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapLevelPack")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatmapLevelPack {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
