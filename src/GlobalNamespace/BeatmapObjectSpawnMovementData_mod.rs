#[cfg(feature = "BeatmapObjectSpawnMovementData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectSpawnMovementData {
    __cordl_parent: crate::System::Object,
    pub _centerPos: crate::UnityEngine::Vector3,
    pub _maxHalfJumpDistance: f32,
    pub _startHalfJumpDurationInBeats: f32,
    pub _baseLinesHighestJumpPosY: f32,
    pub _upperLinesHighestJumpPosY: f32,
    pub _topLinesHighestJumpPosY: f32,
    pub _moveSpeed: f32,
    pub _moveDuration: f32,
    pub _verticalObstaclePosY: f32,
    pub _obstacleTopPosY: f32,
    pub _spawnAheadTime: f32,
    pub _jumpDuration: f32,
    pub _noteJumpStartBeatOffset: f32,
    pub _noteJumpMovementSpeed: f32,
    pub _jumpDistance: f32,
    pub _moveDistance: f32,
    pub _moveStartPos: crate::UnityEngine::Vector3,
    pub _moveEndPos: crate::UnityEngine::Vector3,
    pub _jumpEndPos: crate::UnityEngine::Vector3,
    pub _noteLinesCount: i32,
    pub _jumpOffsetYProvider: *mut IJumpOffsetYProvider,
    pub _rightVec: crate::UnityEngine::Vector3,
    pub _forwardVec: crate::UnityEngine::Vector3,
}
#[cfg(feature = "BeatmapObjectSpawnMovementData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapObjectSpawnMovementData => ""
    ."BeatmapObjectSpawnMovementData"
);
#[cfg(feature = "BeatmapObjectSpawnMovementData")]
impl std::ops::Deref for BeatmapObjectSpawnMovementData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectSpawnMovementData")]
impl std::ops::DerefMut for BeatmapObjectSpawnMovementData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectSpawnMovementData")]
impl BeatmapObjectSpawnMovementData {
    pub const kDefaultMaxHalfJumpDistance: f32 = 18f32;
    pub const kDefaultStartHalfJumpDurationInBeats: f32 = 4f32;
    #[cfg(feature = "BeatmapObjectSpawnMovementData+NoteJumpValueType")]
    pub type NoteJumpValueType = crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteJumpValueType;
    #[cfg(feature = "BeatmapObjectSpawnMovementData+NoteSpawnData")]
    pub type NoteSpawnData = crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteSpawnData;
    #[cfg(feature = "BeatmapObjectSpawnMovementData+SliderSpawnData")]
    pub type SliderSpawnData = crate::GlobalNamespace::BeatmapObjectSpawnMovementData_SliderSpawnData;
    #[cfg(feature = "BeatmapObjectSpawnMovementData+ObstacleSpawnData")]
    pub type ObstacleSpawnData = crate::GlobalNamespace::BeatmapObjectSpawnMovementData_ObstacleSpawnData;
    pub fn get_moveDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_moveDuration", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_jumpDistance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_jumpDistance", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_noteJumpMovementSpeed(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_noteJumpMovementSpeed", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetNoteOffset(
        &mut self,
        noteLineIndex: i32,
        noteLineLayer: NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("GetNoteOffset", (noteLineIndex, noteLineLayer))?;
        Ok(__cordl_ret)
    }
    pub fn get_noteLinesDistance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_noteLinesDistance", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_verticalLayersDistance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_verticalLayersDistance", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_jumpDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_jumpDuration", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        noteLinesCount: i32,
        startNoteJumpMovementSpeed: f32,
        startBpm: f32,
        noteJumpValueType: crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteJumpValueType,
        noteJumpValue: f32,
        jumpOffsetYProvider: *mut IJumpOffsetYProvider,
        rightVec: crate::UnityEngine::Vector3,
        forwardVec: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    noteLinesCount,
                    startNoteJumpMovementSpeed,
                    startBpm,
                    noteJumpValueType,
                    noteJumpValue,
                    jumpOffsetYProvider,
                    rightVec,
                    forwardVec,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetObstacleOffset(
        &mut self,
        noteLineIndex: i32,
        noteLineLayer: NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("GetObstacleOffset", (noteLineIndex, noteLineLayer))?;
        Ok(__cordl_ret)
    }
    pub fn Get2DNoteOffset(
        &mut self,
        noteLineIndex: i32,
        noteLineLayer: NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("Get2DNoteOffset", (noteLineIndex, noteLineLayer))?;
        Ok(__cordl_ret)
    }
    pub fn get_spawnAheadTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_spawnAheadTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetJumpingNoteSpawnData(
        &mut self,
        noteData: *mut NoteData,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteSpawnData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteSpawnData = __cordl_object
            .invoke("GetJumpingNoteSpawnData", (noteData))?;
        Ok(__cordl_ret)
    }
    pub fn GetObstacleSpawnData(
        &mut self,
        obstacleData: *mut ObstacleData,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::BeatmapObjectSpawnMovementData_ObstacleSpawnData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapObjectSpawnMovementData_ObstacleSpawnData = __cordl_object
            .invoke("GetObstacleSpawnData", (obstacleData))?;
        Ok(__cordl_ret)
    }
    pub fn get_centerPos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_centerPos", ())?;
        Ok(__cordl_ret)
    }
    pub fn JumpPosYForLineLayerAtDistanceFromPlayerWithoutJumpOffset(
        &mut self,
        lineLayer: NoteLineLayer,
        distanceFromPlayer: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "JumpPosYForLineLayerAtDistanceFromPlayerWithoutJumpOffset",
                (lineLayer, distanceFromPlayer),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_noteLinesCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_noteLinesCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn HighestJumpPosYForLineLayer(
        &mut self,
        lineLayer: NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("HighestJumpPosYForLineLayer", (lineLayer))?;
        Ok(__cordl_ret)
    }
    pub fn HighestJumpPosYForLineLayerWithoutJumpOffset(
        &mut self,
        lineLayer: NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("HighestJumpPosYForLineLayerWithoutJumpOffset", (lineLayer))?;
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
    pub fn GetSliderSpawnData(
        &mut self,
        sliderData: *mut SliderData,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::BeatmapObjectSpawnMovementData_SliderSpawnData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapObjectSpawnMovementData_SliderSpawnData = __cordl_object
            .invoke("GetSliderSpawnData", (sliderData))?;
        Ok(__cordl_ret)
    }
    pub fn NoteJumpGravityForLineLayerWithoutJumpOffset(
        &mut self,
        lineLayer: NoteLineLayer,
        beforeJumpLineLayer: NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "NoteJumpGravityForLineLayerWithoutJumpOffset",
                (lineLayer, beforeJumpLineLayer),
            )?;
        Ok(__cordl_ret)
    }
    pub fn NoteJumpGravityForLineLayer(
        &mut self,
        lineLayer: NoteLineLayer,
        beforeJumpLineLayer: NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("NoteJumpGravityForLineLayer", (lineLayer, beforeJumpLineLayer))?;
        Ok(__cordl_ret)
    }
    pub fn get_jumpOffsetY(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_jumpOffsetY", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapObjectSpawnMovementData")]
impl quest_hook::libil2cpp::ObjectType for BeatmapObjectSpawnMovementData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapObjectSpawnMovementData+NoteJumpValueType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeatmapObjectSpawnMovementData_NoteJumpValueType {
    BeatOffset = 1i32,
    JumpDuration = 2i32,
}
#[cfg(feature = "BeatmapObjectSpawnMovementData+NoteJumpValueType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteJumpValueType => ""
    ."BeatmapObjectSpawnMovementData/NoteJumpValueType"
);
#[cfg(feature = "BeatmapObjectSpawnMovementData+NoteSpawnData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BeatmapObjectSpawnMovementData_NoteSpawnData {
    pub moveStartPos: crate::UnityEngine::Vector3,
    pub moveEndPos: crate::UnityEngine::Vector3,
    pub jumpEndPos: crate::UnityEngine::Vector3,
    pub jumpGravity: f32,
    pub moveDuration: f32,
    pub jumpDuration: f32,
}
#[cfg(feature = "BeatmapObjectSpawnMovementData+NoteSpawnData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteSpawnData => ""
    ."BeatmapObjectSpawnMovementData/NoteSpawnData"
);
#[cfg(feature = "BeatmapObjectSpawnMovementData+NoteSpawnData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteSpawnData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapObjectSpawnMovementData+NoteSpawnData")]
impl crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteSpawnData {
    pub fn _ctor(
        &mut self,
        moveStartPos: crate::UnityEngine::Vector3,
        moveEndPos: crate::UnityEngine::Vector3,
        jumpEndPos: crate::UnityEngine::Vector3,
        jumpGravity: f32,
        moveDuration: f32,
        jumpDuration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (
                moveStartPos,
                moveEndPos,
                jumpEndPos,
                jumpGravity,
                moveDuration,
                jumpDuration,
            ),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapObjectSpawnMovementData+ObstacleSpawnData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BeatmapObjectSpawnMovementData_ObstacleSpawnData {
    pub moveStartPos: crate::UnityEngine::Vector3,
    pub moveEndPos: crate::UnityEngine::Vector3,
    pub jumpEndPos: crate::UnityEngine::Vector3,
    pub obstacleHeight: f32,
    pub moveDuration: f32,
    pub jumpDuration: f32,
    pub noteLinesDistance: f32,
}
#[cfg(feature = "BeatmapObjectSpawnMovementData+ObstacleSpawnData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapObjectSpawnMovementData_ObstacleSpawnData => ""
    ."BeatmapObjectSpawnMovementData/ObstacleSpawnData"
);
#[cfg(feature = "BeatmapObjectSpawnMovementData+ObstacleSpawnData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::BeatmapObjectSpawnMovementData_ObstacleSpawnData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapObjectSpawnMovementData+ObstacleSpawnData")]
impl crate::GlobalNamespace::BeatmapObjectSpawnMovementData_ObstacleSpawnData {
    pub fn _ctor(
        &mut self,
        moveStartPos: crate::UnityEngine::Vector3,
        moveEndPos: crate::UnityEngine::Vector3,
        jumpEndPos: crate::UnityEngine::Vector3,
        obstacleHeight: f32,
        moveDuration: f32,
        jumpDuration: f32,
        noteLinesDistance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (
                moveStartPos,
                moveEndPos,
                jumpEndPos,
                obstacleHeight,
                moveDuration,
                jumpDuration,
                noteLinesDistance,
            ),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapObjectSpawnMovementData+SliderSpawnData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BeatmapObjectSpawnMovementData_SliderSpawnData {
    pub headMoveStartPos: crate::UnityEngine::Vector3,
    pub headJumpStartPos: crate::UnityEngine::Vector3,
    pub headJumpEndPos: crate::UnityEngine::Vector3,
    pub headJumpGravity: f32,
    pub tailMoveStartPos: crate::UnityEngine::Vector3,
    pub tailJumpStartPos: crate::UnityEngine::Vector3,
    pub tailJumpEndPos: crate::UnityEngine::Vector3,
    pub tailJumpGravity: f32,
    pub moveDuration: f32,
    pub jumpDuration: f32,
}
#[cfg(feature = "BeatmapObjectSpawnMovementData+SliderSpawnData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapObjectSpawnMovementData_SliderSpawnData => ""
    ."BeatmapObjectSpawnMovementData/SliderSpawnData"
);
#[cfg(feature = "BeatmapObjectSpawnMovementData+SliderSpawnData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::BeatmapObjectSpawnMovementData_SliderSpawnData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapObjectSpawnMovementData+SliderSpawnData")]
impl crate::GlobalNamespace::BeatmapObjectSpawnMovementData_SliderSpawnData {
    pub fn _ctor(
        &mut self,
        headMoveStartPos: crate::UnityEngine::Vector3,
        headJumpStartPos: crate::UnityEngine::Vector3,
        headJumpEndPos: crate::UnityEngine::Vector3,
        headJumpGravity: f32,
        tailMoveStartPos: crate::UnityEngine::Vector3,
        tailJumpStartPos: crate::UnityEngine::Vector3,
        tailJumpEndPos: crate::UnityEngine::Vector3,
        tailJumpGravity: f32,
        moveDuration: f32,
        jumpDuration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (
                headMoveStartPos,
                headJumpStartPos,
                headJumpEndPos,
                headJumpGravity,
                tailMoveStartPos,
                tailJumpStartPos,
                tailJumpEndPos,
                tailJumpGravity,
                moveDuration,
                jumpDuration,
            ),
        )?;
        Ok(__cordl_ret)
    }
}
