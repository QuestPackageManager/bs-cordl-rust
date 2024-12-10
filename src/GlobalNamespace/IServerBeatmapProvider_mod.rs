#[cfg(feature = "IServerBeatmapProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IServerBeatmapProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IServerBeatmapProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IServerBeatmapProvider => ""
    ."IServerBeatmapProvider"
);
#[cfg(feature = "IServerBeatmapProvider")]
impl std::ops::Deref for crate::GlobalNamespace::IServerBeatmapProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IServerBeatmapProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::IServerBeatmapProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IServerBeatmapProvider")]
impl crate::GlobalNamespace::IServerBeatmapProvider {
    pub fn SelectBeatmapFromSuggestionsWithSelectionMaskAndOwnedSongPacks(
        &mut self,
        playerCount: i32,
        beatmapsSuggestedByPlayers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
            >,
        >,
        selectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
        playerOwnedSongPacks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                crate::GlobalNamespace::SongPackMask,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapKeyNetSerializable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapKeyNetSerializable,
        > = __cordl_object
            .invoke(
                "SelectBeatmapFromSuggestionsWithSelectionMaskAndOwnedSongPacks",
                (
                    playerCount,
                    beatmapsSuggestedByPlayers,
                    selectionMask,
                    playerOwnedSongPacks,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifyBeatmapForSelectionMask(
        &mut self,
        beatmapKeySerializable: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapKeyNetSerializable,
        >,
        selectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "VerifyBeatmapForSelectionMask",
                (beatmapKeySerializable, selectionMask),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IServerBeatmapProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IServerBeatmapProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
