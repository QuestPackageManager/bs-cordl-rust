#[cfg(feature = "NoteSpawnInfoNetSerializable")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteSpawnInfoNetSerializable {
    __cordl_parent: crate::GlobalNamespace::PoolableSerializable,
    pub _cordl_time: f32,
    pub lineIndex: i32,
    pub noteLineLayer: crate::GlobalNamespace::NoteLineLayer,
    pub beforeJumpNoteLineLayer: crate::GlobalNamespace::NoteLineLayer,
    pub gameplayType: crate::GlobalNamespace::NoteData_GameplayType,
    pub scoringType: crate::GlobalNamespace::NoteData_ScoringType,
    pub colorType: crate::GlobalNamespace::ColorType,
    pub cutDirection: crate::GlobalNamespace::NoteCutDirection,
    pub timeToNextColorNote: f32,
    pub timeToPrevColorNote: f32,
    pub flipLineIndex: i32,
    pub flipYSide: f32,
    pub moveStartPos: crate::GlobalNamespace::Vector3Serializable,
    pub moveEndPos: crate::GlobalNamespace::Vector3Serializable,
    pub jumpEndPos: crate::GlobalNamespace::Vector3Serializable,
    pub jumpGravity: f32,
    pub moveDuration: f32,
    pub jumpDuration: f32,
    pub rotation: f32,
    pub cutDirectionAngleOffset: f32,
    pub cutSfxVolumeMultiplier: f32,
}
#[cfg(feature = "NoteSpawnInfoNetSerializable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteSpawnInfoNetSerializable =>
    ""."NoteSpawnInfoNetSerializable"
);
#[cfg(feature = "NoteSpawnInfoNetSerializable")]
impl std::ops::Deref for crate::GlobalNamespace::NoteSpawnInfoNetSerializable {
    type Target = crate::GlobalNamespace::PoolableSerializable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteSpawnInfoNetSerializable")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteSpawnInfoNetSerializable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteSpawnInfoNetSerializable")]
impl crate::GlobalNamespace::NoteSpawnInfoNetSerializable {
    pub fn Deserialize(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        _cordl_time: f32,
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
        moveStartPos: crate::UnityEngine::Vector3,
        moveEndPos: crate::UnityEngine::Vector3,
        jumpEndPos: crate::UnityEngine::Vector3,
        jumpGravity: f32,
        moveDuration: f32,
        jumpDuration: f32,
        rotation: f32,
        cutDirectionAngleOffset: f32,
        cutSfxVolumeMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::NoteSpawnInfoNetSerializable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::NoteSpawnInfoNetSerializable = __cordl_object
            .invoke(
                "Init",
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
                    moveStartPos,
                    moveEndPos,
                    jumpEndPos,
                    jumpGravity,
                    moveDuration,
                    jumpDuration,
                    rotation,
                    cutDirectionAngleOffset,
                    cutSfxVolumeMultiplier,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "NoteSpawnInfoNetSerializable")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoteSpawnInfoNetSerializable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
