#[cfg(feature = "BeatmapLevelExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapLevelExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapLevelExtensions => ""
    ."BeatmapLevelExtensions"
);
#[cfg(feature = "BeatmapLevelExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLevelExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapLevelExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelExtensions")]
impl crate::GlobalNamespace::BeatmapLevelExtensions {
    pub fn CreateColorScheme(
        idx: i32,
        colorSchemes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapLevelSO_ColorScheme,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateColorScheme", (idx, colorSchemes))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToCensoredRuntime(
        beatmapLevelSo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelSO,
        >,
        coverSprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        censoredLocalizedSongName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevel,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ToCensoredRuntime",
                (beatmapLevelSo, coverSprite, censoredLocalizedSongName),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToRuntime(
        beatmapLevelSo: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelSO>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevel,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToRuntime", (beatmapLevelSo))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapLevelExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapLevelExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
