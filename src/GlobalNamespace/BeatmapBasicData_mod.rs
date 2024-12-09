#[cfg(feature = "BeatmapBasicData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapBasicData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub noteJumpMovementSpeed: f32,
    pub noteJumpStartBeatOffset: f32,
    pub environmentName: crate::GlobalNamespace::EnvironmentName,
    pub beatmapColorScheme: *mut crate::GlobalNamespace::ColorScheme,
    pub notesCount: i32,
    pub obstaclesCount: i32,
    pub bombsCount: i32,
    pub mappers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub lighters: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "BeatmapBasicData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapBasicData => ""
    ."BeatmapBasicData"
);
#[cfg(feature = "BeatmapBasicData")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapBasicData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapBasicData")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapBasicData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapBasicData")]
impl crate::GlobalNamespace::BeatmapBasicData {
    pub fn New(
        noteJumpMovementSpeed: f32,
        noteJumpStartBeatOffset: f32,
        environmentName: crate::GlobalNamespace::EnvironmentName,
        beatmapColorScheme: *mut crate::GlobalNamespace::ColorScheme,
        notesCount: i32,
        obstaclesCount: i32,
        bombsCount: i32,
        mappers: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        lighters: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    noteJumpMovementSpeed,
                    noteJumpStartBeatOffset,
                    environmentName,
                    beatmapColorScheme,
                    notesCount,
                    obstaclesCount,
                    bombsCount,
                    mappers,
                    lighters,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        noteJumpMovementSpeed: f32,
        noteJumpStartBeatOffset: f32,
        environmentName: crate::GlobalNamespace::EnvironmentName,
        beatmapColorScheme: *mut crate::GlobalNamespace::ColorScheme,
        notesCount: i32,
        obstaclesCount: i32,
        bombsCount: i32,
        mappers: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        lighters: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    noteJumpMovementSpeed,
                    noteJumpStartBeatOffset,
                    environmentName,
                    beatmapColorScheme,
                    notesCount,
                    obstaclesCount,
                    bombsCount,
                    mappers,
                    lighters,
                ),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapBasicData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatmapBasicData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
