#[cfg(feature = "NoteData")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteData {
    __cordl_parent: crate::GlobalNamespace::BeatmapObjectData,
    pub _gameplayType_k__BackingField: crate::GlobalNamespace::NoteData_GameplayType,
    pub _scoringType_k__BackingField: crate::GlobalNamespace::NoteData_ScoringType,
    pub _colorType_k__BackingField: crate::GlobalNamespace::ColorType,
    pub _cutDirection_k__BackingField: crate::GlobalNamespace::NoteCutDirection,
    pub _timeToNextColorNote_k__BackingField: f32,
    pub _timeToPrevColorNote_k__BackingField: f32,
    pub _lineIndex_k__BackingField: i32,
    pub _noteLineLayer_k__BackingField: crate::GlobalNamespace::NoteLineLayer,
    pub _beforeJumpNoteLineLayer_k__BackingField: crate::GlobalNamespace::NoteLineLayer,
    pub _flipLineIndex_k__BackingField: i32,
    pub _flipYSide_k__BackingField: f32,
    pub _cutDirectionAngleOffset_k__BackingField: f32,
    pub _cutSfxVolumeMultiplier_k__BackingField: f32,
    pub _isArcHead_k__BackingField: bool,
    pub _isArcTail_k__BackingField: bool,
}
#[cfg(feature = "NoteData")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::NoteData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NoteData";
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
#[cfg(feature = "NoteData")]
impl std::ops::Deref for crate::GlobalNamespace::NoteData {
    type Target = crate::GlobalNamespace::BeatmapObjectData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteData")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteData")]
impl crate::GlobalNamespace::NoteData {
    #[cfg(feature = "NoteData+GameplayType")]
    pub type GameplayType = crate::GlobalNamespace::NoteData_GameplayType;
    #[cfg(feature = "NoteData+ScoringType")]
    pub type ScoringType = crate::GlobalNamespace::NoteData_ScoringType;
    pub fn ChangeNoteCutDirection(
        &mut self,
        newCutDirection: crate::GlobalNamespace::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChangeNoteCutDirection", (newCutDirection))?;
        Ok(__cordl_ret.into())
    }
    pub fn ChangeToBurstSliderHead(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChangeToBurstSliderHead", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ChangeToGameNote(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChangeToGameNote", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyWith(
        &mut self,
        _cordl_time: crate::System::Nullable_1<f32>,
        beat: crate::System::Nullable_1<i32>,
        rotation: crate::System::Nullable_1<i32>,
        lineIndex: crate::System::Nullable_1<i32>,
        noteLineLayer: crate::System::Nullable_1<crate::GlobalNamespace::NoteLineLayer>,
        beforeJumpNoteLineLayer: crate::System::Nullable_1<
            crate::GlobalNamespace::NoteLineLayer,
        >,
        gameplayType: crate::System::Nullable_1<
            crate::GlobalNamespace::NoteData_GameplayType,
        >,
        scoringType: crate::System::Nullable_1<
            crate::GlobalNamespace::NoteData_ScoringType,
        >,
        colorType: crate::System::Nullable_1<crate::GlobalNamespace::ColorType>,
        cutDirection: crate::System::Nullable_1<
            crate::GlobalNamespace::NoteCutDirection,
        >,
        timeToNextColorNote: crate::System::Nullable_1<f32>,
        timeToPrevColorNote: crate::System::Nullable_1<f32>,
        flipLineIndex: crate::System::Nullable_1<i32>,
        flipYSide: crate::System::Nullable_1<f32>,
        cutDirectionAngleOffset: crate::System::Nullable_1<f32>,
        cutSfxVolumeMultiplier: crate::System::Nullable_1<f32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData> = __cordl_object
            .invoke(
                "CopyWith",
                (
                    _cordl_time,
                    beat,
                    rotation,
                    lineIndex,
                    noteLineLayer,
                    beforeJumpNoteLineLayer,
                    gameplayType,
                    scoringType,
                    colorType,
                    cutDirection,
                    timeToNextColorNote,
                    timeToPrevColorNote,
                    flipLineIndex,
                    flipYSide,
                    cutDirectionAngleOffset,
                    cutSfxVolumeMultiplier,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateBasicNoteData(
        _cordl_time: f32,
        beat: f32,
        rotation: i32,
        lineIndex: i32,
        noteLineLayer: crate::GlobalNamespace::NoteLineLayer,
        colorType: crate::GlobalNamespace::ColorType,
        cutDirection: crate::GlobalNamespace::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateBasicNoteData",
                (
                    _cordl_time,
                    beat,
                    rotation,
                    lineIndex,
                    noteLineLayer,
                    colorType,
                    cutDirection,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateBombNoteData(
        _cordl_time: f32,
        beat: f32,
        rotation: i32,
        lineIndex: i32,
        noteLineLayer: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateBombNoteData",
                (_cordl_time, beat, rotation, lineIndex, noteLineLayer),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateBurstSliderNoteData(
        _cordl_time: f32,
        beat: f32,
        rotation: i32,
        lineIndex: i32,
        noteLineLayer: crate::GlobalNamespace::NoteLineLayer,
        beforeJumpNoteLineLayer: crate::GlobalNamespace::NoteLineLayer,
        colorType: crate::GlobalNamespace::ColorType,
        cutDirection: crate::GlobalNamespace::NoteCutDirection,
        cutSfxVolumeMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateBurstSliderNoteData",
                (
                    _cordl_time,
                    beat,
                    rotation,
                    lineIndex,
                    noteLineLayer,
                    beforeJumpNoteLineLayer,
                    colorType,
                    cutDirection,
                    cutSfxVolumeMultiplier,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCopy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataItem,
        > = __cordl_object.invoke("GetCopy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkAsSliderHead(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkAsSliderHead", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkAsSliderTail(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkAsSliderTail", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Mirror(
        &mut self,
        lineCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Mirror", (lineCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        _cordl_time: f32,
        beat: f32,
        rotation: i32,
        lineIndex: i32,
        noteLineLayer: crate::GlobalNamespace::NoteLineLayer,
        beforeJumpNoteLineLayer: crate::GlobalNamespace::NoteLineLayer,
        gameplayType: crate::GlobalNamespace::NoteData_GameplayType,
        scoringType: crate::GlobalNamespace::NoteData_ScoringType,
        colorType: crate::GlobalNamespace::ColorType,
        cutDirection: crate::GlobalNamespace::NoteCutDirection,
        timeToNextColorNote: f32,
        timeToPrevColorNote: f32,
        flipLineIndex: i32,
        flipYSide: f32,
        cutDirectionAngleOffset: f32,
        cutSfxVolumeMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    _cordl_time,
                    beat,
                    rotation,
                    lineIndex,
                    noteLineLayer,
                    beforeJumpNoteLineLayer,
                    gameplayType,
                    scoringType,
                    colorType,
                    cutDirection,
                    timeToNextColorNote,
                    timeToPrevColorNote,
                    flipLineIndex,
                    flipYSide,
                    cutDirectionAngleOffset,
                    cutSfxVolumeMultiplier,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ResetNoteFlip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetNoteFlip", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBeforeJumpNoteLineLayer(
        &mut self,
        lineLayer: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBeforeJumpNoteLineLayer", (lineLayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCutDirectionAngleOffset(
        &mut self,
        cutDirectionAngleOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCutDirectionAngleOffset", (cutDirectionAngleOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNoteFlipToNote(
        &mut self,
        targetNote: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNoteFlipToNote", (targetNote))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNoteToAnyCutDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNoteToAnyCutDirection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SubtypeIdentifier(
        colorType: crate::GlobalNamespace::ColorType,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SubtypeIdentifier", (colorType))?;
        Ok(__cordl_ret.into())
    }
    pub fn TransformNoteAOrBToRandomType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TransformNoteAOrBToRandomType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        beat: f32,
        rotation: i32,
        lineIndex: i32,
        noteLineLayer: crate::GlobalNamespace::NoteLineLayer,
        beforeJumpNoteLineLayer: crate::GlobalNamespace::NoteLineLayer,
        gameplayType: crate::GlobalNamespace::NoteData_GameplayType,
        scoringType: crate::GlobalNamespace::NoteData_ScoringType,
        colorType: crate::GlobalNamespace::ColorType,
        cutDirection: crate::GlobalNamespace::NoteCutDirection,
        timeToNextColorNote: f32,
        timeToPrevColorNote: f32,
        flipLineIndex: i32,
        flipYSide: f32,
        cutDirectionAngleOffset: f32,
        cutSfxVolumeMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    _cordl_time,
                    beat,
                    rotation,
                    lineIndex,
                    noteLineLayer,
                    beforeJumpNoteLineLayer,
                    gameplayType,
                    scoringType,
                    colorType,
                    cutDirection,
                    timeToNextColorNote,
                    timeToPrevColorNote,
                    flipLineIndex,
                    flipYSide,
                    cutDirectionAngleOffset,
                    cutSfxVolumeMultiplier,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beforeJumpNoteLineLayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteLineLayer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::NoteLineLayer = __cordl_object
            .invoke("get_beforeJumpNoteLineLayer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_colorType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::ColorType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::ColorType = __cordl_object
            .invoke("get_colorType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cutDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteCutDirection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::NoteCutDirection = __cordl_object
            .invoke("get_cutDirection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cutDirectionAngleOffset(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_cutDirectionAngleOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cutSfxVolumeMultiplier(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_cutSfxVolumeMultiplier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flipLineIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_flipLineIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flipYSide(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_flipYSide", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_gameplayType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteData_GameplayType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::NoteData_GameplayType = __cordl_object
            .invoke("get_gameplayType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isArcHead(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isArcHead", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isArcTail(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isArcTail", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lineIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_lineIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noteLineLayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteLineLayer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::NoteLineLayer = __cordl_object
            .invoke("get_noteLineLayer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scoringType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteData_ScoringType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::NoteData_ScoringType = __cordl_object
            .invoke("get_scoringType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_subtypeGroupIdentifier(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_subtypeGroupIdentifier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_timeToNextColorNote(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_timeToNextColorNote", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_timeToPrevColorNote(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_timeToPrevColorNote", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_beforeJumpNoteLineLayer(
        &mut self,
        value: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_beforeJumpNoteLineLayer", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_colorType(
        &mut self,
        value: crate::GlobalNamespace::ColorType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_colorType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_cutDirection(
        &mut self,
        value: crate::GlobalNamespace::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cutDirection", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_cutDirectionAngleOffset(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cutDirectionAngleOffset", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_cutSfxVolumeMultiplier(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cutSfxVolumeMultiplier", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_flipLineIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_flipLineIndex", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_flipYSide(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_flipYSide", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_gameplayType(
        &mut self,
        value: crate::GlobalNamespace::NoteData_GameplayType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_gameplayType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isArcHead(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isArcHead", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isArcTail(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isArcTail", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_lineIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lineIndex", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_noteLineLayer(
        &mut self,
        value: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_noteLineLayer", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_scoringType(
        &mut self,
        value: crate::GlobalNamespace::NoteData_ScoringType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_scoringType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_timeToNextColorNote(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_timeToNextColorNote", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_timeToPrevColorNote(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_timeToPrevColorNote", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NoteData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::NoteData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NoteData+GameplayType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoteData_GameplayType {
    #[default]
    Bomb = 1i32,
    BurstSliderElement = 3i32,
    BurstSliderHead = 2i32,
    Normal = 0i32,
}
#[cfg(feature = "NoteData+GameplayType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NoteData_GameplayType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GameplayType";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::NoteData_GameplayType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::NoteData_GameplayType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::NoteData_GameplayType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::NoteData_GameplayType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "NoteData+ScoringType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoteData_ScoringType {
    #[default]
    ArcHead = 2i32,
    ArcHeadArcTail = 6i32,
    ArcTail = 3i32,
    ChainHead = 4i32,
    ChainHeadArcTail = 7i32,
    ChainLink = 5i32,
    ChainLinkArcHead = 8i32,
    Ignore = -1i32,
    NoScore = 0i32,
    Normal = 1i32,
}
#[cfg(feature = "NoteData+ScoringType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NoteData_ScoringType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ScoringType";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::NoteData_ScoringType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::NoteData_ScoringType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::NoteData_ScoringType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::NoteData_ScoringType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
