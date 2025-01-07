#[cfg(feature = "MockBeatmapDataConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct MockBeatmapDataConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "MockBeatmapDataConverter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MockBeatmapDataConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MockBeatmapDataConverter";
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
