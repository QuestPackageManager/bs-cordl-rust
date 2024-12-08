#[cfg(feature = "MockBeatmapProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct MockBeatmapProvider {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "MockBeatmapProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MockBeatmapProvider => ""."MockBeatmapProvider"
);
#[cfg(feature = "MockBeatmapProvider")]
impl std::ops::Deref for MockBeatmapProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockBeatmapProvider")]
impl std::ops::DerefMut for MockBeatmapProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockBeatmapProvider")]
impl MockBeatmapProvider {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SelectBeatmapFromSuggestionsWithSelectionMaskAndOwnedSongPacks(
        &mut self,
        playerCount: i32,
        suggestedBeatmaps: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut BeatmapKeyNetSerializable,
        >,
        selectionMask: BeatmapLevelSelectionMask,
        ownedSongPacks: *mut crate::System::Collections::Generic::Dictionary_2<
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
                (playerCount, suggestedBeatmaps, selectionMask, ownedSongPacks),
            )?;
        Ok(__cordl_ret)
    }
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
}
#[cfg(feature = "MockBeatmapProvider")]
impl quest_hook::libil2cpp::ObjectType for MockBeatmapProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
