#[cfg(feature = "NoteData+GameplayType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteData_GameplayType {
    Bomb = 1i32,
    BurstSliderElement = 3i32,
    BurstSliderHead = 2i32,
    Normal = 0i32,
}
#[cfg(feature = "NoteData+GameplayType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteData_GameplayType => ""
    ."NoteData/GameplayType"
);
#[cfg(feature = "NoteData")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteData {
    __cordl_parent: BeatmapObjectData,
    pub _gameplayType_k__BackingField: crate::GlobalNamespace::NoteData_GameplayType,
    pub _scoringType_k__BackingField: crate::GlobalNamespace::NoteData_ScoringType,
    pub _colorType_k__BackingField: ColorType,
    pub _cutDirection_k__BackingField: NoteCutDirection,
    pub _timeToNextColorNote_k__BackingField: f32,
    pub _timeToPrevColorNote_k__BackingField: f32,
    pub _lineIndex_k__BackingField: i32,
    pub _noteLineLayer_k__BackingField: NoteLineLayer,
    pub _beforeJumpNoteLineLayer_k__BackingField: NoteLineLayer,
    pub _flipLineIndex_k__BackingField: i32,
    pub _flipYSide_k__BackingField: f32,
    pub _cutDirectionAngleOffset_k__BackingField: f32,
    pub _cutSfxVolumeMultiplier_k__BackingField: f32,
    pub _isArcHead_k__BackingField: bool,
    pub _isArcTail_k__BackingField: bool,
}
#[cfg(feature = "NoteData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for NoteData => ""."NoteData"
);
#[cfg(feature = "NoteData")]
impl std::ops::Deref for NoteData {
    type Target = BeatmapObjectData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteData")]
impl std::ops::DerefMut for NoteData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteData")]
impl NoteData {
    #[cfg(feature = "NoteData+GameplayType")]
    pub type GameplayType = crate::GlobalNamespace::NoteData_GameplayType;
    #[cfg(feature = "NoteData+ScoringType")]
    pub type ScoringType = crate::GlobalNamespace::NoteData_ScoringType;
    pub fn ChangeNoteCutDirection(
        &mut self,
        newCutDirection: NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChangeNoteCutDirection", (newCutDirection))?;
        Ok(__cordl_ret)
    }
    pub fn ChangeToBurstSliderHead(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChangeToBurstSliderHead", ())?;
        Ok(__cordl_ret)
    }
    pub fn ChangeToGameNote(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChangeToGameNote", ())?;
        Ok(__cordl_ret)
    }
    pub fn CopyWith(
        &mut self,
        _cordl_time: crate::System::Nullable_1<f32>,
        lineIndex: crate::System::Nullable_1<i32>,
        noteLineLayer: crate::System::Nullable_1<NoteLineLayer>,
        beforeJumpNoteLineLayer: crate::System::Nullable_1<NoteLineLayer>,
        gameplayType: crate::System::Nullable_1<
            crate::GlobalNamespace::NoteData_GameplayType,
        >,
        scoringType: crate::System::Nullable_1<
            crate::GlobalNamespace::NoteData_ScoringType,
        >,
        colorType: crate::System::Nullable_1<ColorType>,
        cutDirection: crate::System::Nullable_1<NoteCutDirection>,
        timeToNextColorNote: crate::System::Nullable_1<f32>,
        timeToPrevColorNote: crate::System::Nullable_1<f32>,
        flipLineIndex: crate::System::Nullable_1<i32>,
        flipYSide: crate::System::Nullable_1<f32>,
        cutDirectionAngleOffset: crate::System::Nullable_1<f32>,
        cutSfxVolumeMultiplier: crate::System::Nullable_1<f32>,
    ) -> quest_hook::libil2cpp::Result<*mut NoteData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut NoteData = __cordl_object
            .invoke(
                "CopyWith",
                (
                    _cordl_time,
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
        Ok(__cordl_ret)
    }
    pub fn GetCopy(&mut self) -> quest_hook::libil2cpp::Result<*mut BeatmapDataItem> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapDataItem = __cordl_object.invoke("GetCopy", ())?;
        Ok(__cordl_ret)
    }
    pub fn MarkAsSliderHead(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkAsSliderHead", ())?;
        Ok(__cordl_ret)
    }
    pub fn MarkAsSliderTail(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkAsSliderTail", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New(
        _cordl_time: f32,
        lineIndex: i32,
        noteLineLayer: NoteLineLayer,
        beforeJumpNoteLineLayer: NoteLineLayer,
        gameplayType: crate::GlobalNamespace::NoteData_GameplayType,
        scoringType: crate::GlobalNamespace::NoteData_ScoringType,
        colorType: ColorType,
        cutDirection: NoteCutDirection,
        timeToNextColorNote: f32,
        timeToPrevColorNote: f32,
        flipLineIndex: i32,
        flipYSide: f32,
        cutDirectionAngleOffset: f32,
        cutSfxVolumeMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    _cordl_time,
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
        Ok(__cordl_object)
    }
    pub fn ResetNoteFlip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetNoteFlip", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetBeforeJumpNoteLineLayer(
        &mut self,
        lineLayer: NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBeforeJumpNoteLineLayer", (lineLayer))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn SetNoteFlipToNote(
        &mut self,
        targetNote: *mut NoteData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNoteFlipToNote", (targetNote))?;
        Ok(__cordl_ret)
    }
    pub fn SetNoteToAnyCutDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNoteToAnyCutDirection", ())?;
        Ok(__cordl_ret)
    }
    pub fn TransformNoteAOrBToRandomType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TransformNoteAOrBToRandomType", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        lineIndex: i32,
        noteLineLayer: NoteLineLayer,
        beforeJumpNoteLineLayer: NoteLineLayer,
        gameplayType: crate::GlobalNamespace::NoteData_GameplayType,
        scoringType: crate::GlobalNamespace::NoteData_ScoringType,
        colorType: ColorType,
        cutDirection: NoteCutDirection,
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
        Ok(__cordl_ret)
    }
    pub fn get_beforeJumpNoteLineLayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<NoteLineLayer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: NoteLineLayer = __cordl_object
            .invoke("get_beforeJumpNoteLineLayer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_colorType(&mut self) -> quest_hook::libil2cpp::Result<ColorType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: ColorType = __cordl_object.invoke("get_colorType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_cutDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<NoteCutDirection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: NoteCutDirection = __cordl_object
            .invoke("get_cutDirection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_cutDirectionAngleOffset(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_cutDirectionAngleOffset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_cutSfxVolumeMultiplier(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_cutSfxVolumeMultiplier", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_flipLineIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_flipLineIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_flipYSide(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_flipYSide", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_gameplayType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteData_GameplayType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::NoteData_GameplayType = __cordl_object
            .invoke("get_gameplayType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isArcHead(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isArcHead", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isArcTail(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isArcTail", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lineIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_lineIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_noteLineLayer(&mut self) -> quest_hook::libil2cpp::Result<NoteLineLayer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: NoteLineLayer = __cordl_object.invoke("get_noteLineLayer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_scoringType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteData_ScoringType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::NoteData_ScoringType = __cordl_object
            .invoke("get_scoringType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_subtypeGroupIdentifier(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_subtypeGroupIdentifier", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_timeToNextColorNote(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_timeToNextColorNote", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_timeToPrevColorNote(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_timeToPrevColorNote", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_beforeJumpNoteLineLayer(
        &mut self,
        value: NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_beforeJumpNoteLineLayer", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_colorType(
        &mut self,
        value: ColorType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_colorType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_cutDirection(
        &mut self,
        value: NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cutDirection", (value))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn set_noteLineLayer(
        &mut self,
        value: NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_noteLineLayer", (value))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "NoteData")]
impl quest_hook::libil2cpp::ObjectType for NoteData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NoteData+ScoringType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteData_ScoringType {
    BurstSliderElement = 5i32,
    BurstSliderHead = 4i32,
    Ignore = -1i32,
    NoScore = 0i32,
    Normal = 1i32,
    SliderHead = 2i32,
    SliderTail = 3i32,
}
#[cfg(feature = "NoteData+ScoringType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteData_ScoringType => ""
    ."NoteData/ScoringType"
);
