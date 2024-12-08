#[cfg(feature = "StaticBeatmapObjectSpawnMovementData")]
#[repr(C)]
#[derive(Debug)]
pub struct StaticBeatmapObjectSpawnMovementData {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "StaticBeatmapObjectSpawnMovementData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::StaticBeatmapObjectSpawnMovementData => ""
    ."StaticBeatmapObjectSpawnMovementData"
);
#[cfg(feature = "StaticBeatmapObjectSpawnMovementData")]
impl std::ops::Deref for crate::GlobalNamespace::StaticBeatmapObjectSpawnMovementData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StaticBeatmapObjectSpawnMovementData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::StaticBeatmapObjectSpawnMovementData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StaticBeatmapObjectSpawnMovementData")]
impl crate::GlobalNamespace::StaticBeatmapObjectSpawnMovementData {
    pub const kBaseLinesYPos: f32 = 0.25f32;
    pub const kNoteLinesDistance: f32 = 0.6f32;
    pub const kObstacleVerticalOffset: f32 = -0.15f32;
    pub const kTopLinesYPos: f32 = 1.45f32;
    pub const kUpperLinesYPos: f32 = 0.85f32;
}
#[cfg(feature = "StaticBeatmapObjectSpawnMovementData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StaticBeatmapObjectSpawnMovementData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
