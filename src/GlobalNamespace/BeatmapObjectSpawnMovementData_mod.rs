#[cfg(feature = "BeatmapObjectSpawnMovementData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectSpawnMovementData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _centerPos: crate::UnityEngine::Vector3,
    pub _maxHalfJumpDistance: f32,
    pub _startHalfJumpDurationInBeats: f32,
    pub _baseLinesHighestJumpPosY: f32,
    pub _upperLinesHighestJumpPosY: f32,
    pub _topLinesHighestJumpPosY: f32,
    pub _verticalObstaclePosY: f32,
    pub _obstacleTopPosY: f32,
    pub _noteLinesCount: i32,
    pub _jumpOffsetYProvider: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IJumpOffsetYProvider,
    >,
    pub _rightVec: crate::UnityEngine::Vector3,
}
#[cfg(feature = "BeatmapObjectSpawnMovementData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapObjectSpawnMovementData
    => ""."BeatmapObjectSpawnMovementData"
);
#[cfg(feature = "BeatmapObjectSpawnMovementData")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapObjectSpawnMovementData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectSpawnMovementData")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapObjectSpawnMovementData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectSpawnMovementData")]
impl crate::GlobalNamespace::BeatmapObjectSpawnMovementData {
    pub const kDefaultMaxHalfJumpDistance: f32 = 18f32;
    pub const kDefaultStartHalfJumpDurationInBeats: f32 = 4f32;
    #[cfg(feature = "BeatmapObjectSpawnMovementData+NoteJumpValueType")]
    pub type NoteJumpValueType = crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteJumpValueType;
    pub fn Get2DNoteOffset(
        &mut self,
        noteLineIndex: i32,
        noteLineLayer: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("Get2DNoteOffset", (noteLineIndex, noteLineLayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGravityBase(
        &mut self,
        noteLineLayer: crate::GlobalNamespace::NoteLineLayer,
        beforeJumpLineLayer: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetGravityBase", (noteLineLayer, beforeJumpLineLayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetJumpingNoteSpawnData(
        &mut self,
        noteData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteSpawnData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::NoteSpawnData = __cordl_object
            .invoke("GetJumpingNoteSpawnData", (noteData))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNoteOffset(
        &mut self,
        noteLineIndex: i32,
        noteLineLayer: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("GetNoteOffset", (noteLineIndex, noteLineLayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObstacleOffset(
        &mut self,
        noteLineIndex: i32,
        noteLineLayer: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("GetObstacleOffset", (noteLineIndex, noteLineLayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObstacleSpawnData(
        &mut self,
        obstacleData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObstacleData>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::ObstacleSpawnData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::ObstacleSpawnData = __cordl_object
            .invoke("GetObstacleSpawnData", (obstacleData))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSliderSpawnData(
        &mut self,
        sliderData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SliderData>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SliderSpawnData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::SliderSpawnData = __cordl_object
            .invoke("GetSliderSpawnData", (sliderData))?;
        Ok(__cordl_ret.into())
    }
    pub fn HighestJumpPosYForLineLayer(
        &mut self,
        lineLayer: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("HighestJumpPosYForLineLayer", (lineLayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn HighestJumpPosYForLineLayerWithoutJumpOffset(
        &mut self,
        lineLayer: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("HighestJumpPosYForLineLayerWithoutJumpOffset", (lineLayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        noteLinesCount: i32,
        jumpOffsetYProvider: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IJumpOffsetYProvider,
        >,
        rightVec: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (noteLinesCount, jumpOffsetYProvider, rightVec))?;
        Ok(__cordl_ret.into())
    }
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
    pub fn get_centerPos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_centerPos", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxHalfJumpDistance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_maxHalfJumpDistance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noteLinesCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_noteLinesCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noteLinesDistance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_noteLinesDistance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startHalfJumpDurationInBeats(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_startHalfJumpDurationInBeats", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_verticalLayersDistance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_verticalLayersDistance", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapObjectSpawnMovementData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapObjectSpawnMovementData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapObjectSpawnMovementData+NoteJumpValueType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BeatmapObjectSpawnMovementData_NoteJumpValueType {
    #[default]
    BeatOffset = 1i32,
    JumpDuration = 2i32,
}
#[cfg(feature = "BeatmapObjectSpawnMovementData+NoteJumpValueType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteJumpValueType => ""
    ."BeatmapObjectSpawnMovementData/NoteJumpValueType"
);
