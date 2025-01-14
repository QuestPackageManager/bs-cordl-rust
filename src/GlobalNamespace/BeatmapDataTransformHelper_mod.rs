#[cfg(feature = "BeatmapDataTransformHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataTransformHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDataTransformHelper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapDataTransformHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapDataTransformHelper";
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
#[cfg(feature = "BeatmapDataTransformHelper")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDataTransformHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    f32,
                    f32,
                    f32,
                    i32,
                    i32,
                    crate::GlobalNamespace::NoteLineLayer,
                    crate::GlobalNamespace::NoteCutDirection,
                    i32,
                    i32,
                    crate::GlobalNamespace::NoteLineLayer,
                    crate::GlobalNamespace::NoteCutDirection,
                    i32,
                    f32,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
                ),
                quest_hook::libil2cpp::Void,
                14usize,
            >("AddTestBurstSlider")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddTestBurstSlider", 14usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
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
                )
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    f32,
                    f32,
                    f32,
                    i32,
                    i32,
                    crate::GlobalNamespace::NoteLineLayer,
                    crate::GlobalNamespace::NoteCutDirection,
                    f32,
                    i32,
                    i32,
                    crate::GlobalNamespace::NoteLineLayer,
                    crate::GlobalNamespace::NoteCutDirection,
                    f32,
                    bool,
                    bool,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
                ),
                quest_hook::libil2cpp::Void,
                16usize,
            >("AddTestSlider")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddTestSlider", 16usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
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
                )
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IReadonlyBeatmapData,
                    >,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
                    bool,
                    crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::EnvironmentIntensityReductionOptions,
                    >,
                    quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
                ),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IReadonlyBeatmapData>,
                7usize,
            >("CreateTransformedBeatmapData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateTransformedBeatmapData", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        beatmapData,
                        beatmapLevel,
                        gameplayModifiers,
                        leftHanded,
                        environmentEffectsFilterPreset,
                        environmentIntensityReductionOptions,
                        settings,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsObstaclesMergingNeeded(
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        screenDisplacementEffectsEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>, bool),
                bool,
                2usize,
            >("IsObstaclesMergingNeeded")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsObstaclesMergingNeeded", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (beatmapLevel, screenDisplacementEffectsEnabled))
        };
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
