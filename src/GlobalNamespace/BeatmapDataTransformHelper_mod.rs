#[cfg(feature = "BeatmapDataTransformHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataTransformHelper {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "BeatmapDataTransformHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapDataTransformHelper =>
    ""."BeatmapDataTransformHelper"
);
#[cfg(feature = "BeatmapDataTransformHelper")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDataTransformHelper {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataTransformHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDataTransformHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataTransformHelper")]
impl crate::GlobalNamespace::BeatmapDataTransformHelper {
    pub fn AddTestBurstSlider(
        _cordl_time: f32,
        beat: f32,
        duration: f32,
        headRotation: i32,
        headLineIndex: i32,
        headNoteLineLayer: crate::GlobalNamespace::NoteLineLayer,
        headCutDirection: crate::GlobalNamespace::NoteCutDirection,
        tailRotation: i32,
        tailLineIndex: i32,
        tailNoteLineLayer: crate::GlobalNamespace::NoteLineLayer,
        tailCutDirection: crate::GlobalNamespace::NoteCutDirection,
        sliceCount: i32,
        squishAmount: f32,
        beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AddTestBurstSlider",
                (
                    _cordl_time,
                    beat,
                    duration,
                    headRotation,
                    headLineIndex,
                    headNoteLineLayer,
                    headCutDirection,
                    tailRotation,
                    tailLineIndex,
                    tailNoteLineLayer,
                    tailCutDirection,
                    sliceCount,
                    squishAmount,
                    beatmapData,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddTestSlider(
        _cordl_time: f32,
        beat: f32,
        duration: f32,
        headRotation: i32,
        headLineIndex: i32,
        headNoteLineLayer: crate::GlobalNamespace::NoteLineLayer,
        headCutDirection: crate::GlobalNamespace::NoteCutDirection,
        headControlPointLength: f32,
        tailRotation: i32,
        tailLineIndex: i32,
        tailNoteLineLayer: crate::GlobalNamespace::NoteLineLayer,
        tailCutDirection: crate::GlobalNamespace::NoteCutDirection,
        tailControlPointLength: f32,
        hasHeadNote: bool,
        hasTailNote: bool,
        beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AddTestSlider",
                (
                    _cordl_time,
                    beat,
                    duration,
                    headRotation,
                    headLineIndex,
                    headNoteLineLayer,
                    headCutDirection,
                    headControlPointLength,
                    tailRotation,
                    tailLineIndex,
                    tailNoteLineLayer,
                    tailCutDirection,
                    tailControlPointLength,
                    hasHeadNote,
                    hasTailNote,
                    beatmapData,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateTransformedBeatmapData(
        beatmapData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        leftHanded: bool,
        environmentEffectsFilterPreset: crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
        environmentIntensityReductionOptions: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentIntensityReductionOptions,
        >,
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IReadonlyBeatmapData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateTransformedBeatmapData",
                (
                    beatmapData,
                    beatmapLevel,
                    gameplayModifiers,
                    leftHanded,
                    environmentEffectsFilterPreset,
                    environmentIntensityReductionOptions,
                    settings,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsObstaclesMergingNeeded(
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        screenDisplacementEffectsEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "IsObstaclesMergingNeeded",
                (beatmapLevel, screenDisplacementEffectsEnabled),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataTransformHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDataTransformHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
