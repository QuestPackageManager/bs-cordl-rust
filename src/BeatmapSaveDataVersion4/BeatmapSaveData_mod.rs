#[cfg(feature = "BeatmapSaveDataVersion4+BeatmapSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapSaveData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub colorNotes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatmapSaveDataVersion4::BeatmapBeatIndex,
        >,
    >,
    pub bombNotes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatmapSaveDataVersion4::BeatmapBeatIndex,
        >,
    >,
    pub obstacles: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatmapSaveDataVersion4::BeatmapBeatIndex,
        >,
    >,
    pub chains: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatmapSaveDataVersion4::ChainBeatIndex,
        >,
    >,
    pub arcs: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatmapSaveDataVersion4::ArcBeatIndex,
        >,
    >,
    pub colorNotesData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::BeatmapSaveDataVersion4::ColorNote>,
    >,
    pub bombNotesData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::BeatmapSaveDataVersion4::BombNote>,
    >,
    pub obstaclesData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::BeatmapSaveDataVersion4::Obstacle>,
    >,
    pub chainsData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::BeatmapSaveDataVersion4::Chain>,
    >,
    pub arcsData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::BeatmapSaveDataVersion4::Arc>,
    >,
    pub njsEvents: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatmapSaveDataVersion4::BeatIndex,
        >,
    >,
    pub njsEventData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::BeatmapSaveDataVersion4::NoteJumpMovementSpeedEvent,
        >,
    >,
}
#[cfg(feature = "BeatmapSaveDataVersion4+BeatmapSaveData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion4::BeatmapSaveData =>
    "BeatmapSaveDataVersion4"."BeatmapSaveData"
);
#[cfg(feature = "BeatmapSaveDataVersion4+BeatmapSaveData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion4::BeatmapSaveData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+BeatmapSaveData")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion4::BeatmapSaveData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+BeatmapSaveData")]
impl crate::BeatmapSaveDataVersion4::BeatmapSaveData {
    pub const kCurrentVersion: &'static str = "4.1.0";
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "BeatmapSaveDataVersion4+BeatmapSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion4::BeatmapSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
