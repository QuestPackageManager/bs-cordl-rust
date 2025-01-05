#[cfg(feature = "MockBeatmapProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct MockBeatmapProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "MockBeatmapProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MockBeatmapProvider => ""
    ."MockBeatmapProvider"
);
#[cfg(feature = "MockBeatmapProvider")]
impl std::ops::Deref for crate::GlobalNamespace::MockBeatmapProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockBeatmapProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockBeatmapProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockBeatmapProvider")]
impl crate::GlobalNamespace::MockBeatmapProvider {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SelectBeatmapFromSuggestionsWithSelectionMaskAndOwnedSongPacks(
        &mut self,
        playerCount: i32,
        suggestedBeatmaps: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapKeyNetSerializable,
                >,
            >,
        >,
        selectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
        ownedSongPacks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
                (playerCount, suggestedBeatmaps, selectionMask, ownedSongPacks),
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
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MockBeatmapProvider")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MockBeatmapProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MockBeatmapProvider")]
impl AsRef<crate::GlobalNamespace::IServerBeatmapProvider>
for crate::GlobalNamespace::MockBeatmapProvider {
    fn as_ref(&self) -> &crate::GlobalNamespace::IServerBeatmapProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MockBeatmapProvider")]
impl AsMut<crate::GlobalNamespace::IServerBeatmapProvider>
for crate::GlobalNamespace::MockBeatmapProvider {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IServerBeatmapProvider {
        unsafe { std::mem::transmute(self) }
    }
}
