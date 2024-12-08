#[cfg(feature = "IServerBeatmapProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IServerBeatmapProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IServerBeatmapProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IServerBeatmapProvider => ""."IServerBeatmapProvider"
);
#[cfg(feature = "IServerBeatmapProvider")]
impl std::ops::Deref for IServerBeatmapProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IServerBeatmapProvider")]
impl std::ops::DerefMut for IServerBeatmapProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IServerBeatmapProvider")]
impl IServerBeatmapProvider {
    pub fn VerifyBeatmapForSelectionMask(
        &mut self,
        beatmapKeySerializable: *mut BeatmapKeyNetSerializable,
        selectionMask: BeatmapLevelSelectionMask,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "VerifyBeatmapForSelectionMask",
                (beatmapKeySerializable, selectionMask),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SelectBeatmapFromSuggestionsWithSelectionMaskAndOwnedSongPacks(
        &mut self,
        playerCount: i32,
        beatmapsSuggestedByPlayers: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut BeatmapKeyNetSerializable,
        >,
        selectionMask: BeatmapLevelSelectionMask,
        playerOwnedSongPacks: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            SongPackMask,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapKeyNetSerializable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapKeyNetSerializable = __cordl_object
            .invoke(
                "SelectBeatmapFromSuggestionsWithSelectionMaskAndOwnedSongPacks",
                (
                    playerCount,
                    beatmapsSuggestedByPlayers,
                    selectionMask,
                    playerOwnedSongPacks,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IServerBeatmapProvider")]
impl quest_hook::libil2cpp::ObjectType for IServerBeatmapProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
