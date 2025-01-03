#[cfg(feature = "MockBeatmapDataConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct MockBeatmapDataConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "MockBeatmapDataConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MockBeatmapDataConverter => ""
    ."MockBeatmapDataConverter"
);
#[cfg(feature = "MockBeatmapDataConverter")]
impl std::ops::Deref for crate::GlobalNamespace::MockBeatmapDataConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockBeatmapDataConverter")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockBeatmapDataConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockBeatmapDataConverter")]
impl crate::GlobalNamespace::MockBeatmapDataConverter {
    pub fn ToMockBeatmapData(
        beatmapData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockBeatmapData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MockBeatmapData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToMockBeatmapData", (beatmapData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToMockNoteData(
        noteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockNoteData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MockNoteData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToMockNoteData", (noteData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToMockObstacleData(
        obstacleData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObstacleData>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockObstacleData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MockObstacleData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToMockObstacleData", (obstacleData))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MockBeatmapDataConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MockBeatmapDataConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
