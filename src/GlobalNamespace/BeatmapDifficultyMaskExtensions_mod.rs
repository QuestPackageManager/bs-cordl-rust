#[cfg(feature = "BeatmapDifficultyMaskExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDifficultyMaskExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "BeatmapDifficultyMaskExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapDifficultyMaskExtensions
    => ""."BeatmapDifficultyMaskExtensions"
);
#[cfg(feature = "BeatmapDifficultyMaskExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDifficultyMaskExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDifficultyMaskExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDifficultyMaskExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDifficultyMaskExtensions")]
impl crate::GlobalNamespace::BeatmapDifficultyMaskExtensions {
    pub fn Contains_BeatmapDifficulty0(
        mask: crate::GlobalNamespace::BeatmapDifficultyMask,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Contains", (mask, difficulty))?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains_BeatmapDifficultyMask1(
        mask: crate::GlobalNamespace::BeatmapDifficultyMask,
        other: crate::GlobalNamespace::BeatmapDifficultyMask,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Contains", (mask, other))?;
        Ok(__cordl_ret.into())
    }
    pub fn DifferenceFrom(
        mask: crate::GlobalNamespace::BeatmapDifficultyMask,
        other: crate::GlobalNamespace::BeatmapDifficultyMask,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DifferenceFrom", (mask, other))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromMask(
        mask: crate::GlobalNamespace::BeatmapDifficultyMask,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapDifficulty> {
        let __cordl_ret: crate::GlobalNamespace::BeatmapDifficulty = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromMask", (mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromMaskMaybe(
        mask: crate::GlobalNamespace::BeatmapDifficultyMask,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::GlobalNamespace::BeatmapDifficulty>,
    > {
        let __cordl_ret: crate::System::Nullable_1<
            crate::GlobalNamespace::BeatmapDifficulty,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromMaskMaybe", (mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn LocalizedKey(
        mask: crate::GlobalNamespace::BeatmapDifficultyMask,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LocalizedKey", (mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShortLocalizedKey(
        mask: crate::GlobalNamespace::BeatmapDifficultyMask,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShortLocalizedKey", (mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToHexString(
        mask: crate::GlobalNamespace::BeatmapDifficultyMask,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToHexString", (mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToMask(
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapDifficultyMask> {
        let __cordl_ret: crate::GlobalNamespace::BeatmapDifficultyMask = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToMask", (difficulty))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDifficultyMaskExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDifficultyMaskExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
