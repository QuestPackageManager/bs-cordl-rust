#[cfg(feature = "BeatmapObjectSpawnController")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectSpawnController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _beatmapObjectSpawnMovementData: *mut BeatmapObjectSpawnMovementData,
    pub _beatmapCallbacksController: *mut BeatmapCallbacksController,
    pub _beatmapObjectSpawner: *mut IBeatmapObjectSpawner,
    pub _jumpOffsetYProvider: *mut IJumpOffsetYProvider,
    pub _initData: *mut crate::GlobalNamespace::BeatmapObjectSpawnController_InitData,
    pub didInitEvent: *mut crate::System::Action,
    pub _disableSpawning: bool,
    pub _isInitialized: bool,
    pub _obstacleDataCallbackWrapper: *mut BeatmapDataCallbackWrapper,
    pub _noteDataCallbackWrapper: *mut BeatmapDataCallbackWrapper,
    pub _sliderDataCallbackWrapper: *mut BeatmapDataCallbackWrapper,
    pub _spawnRotationCallbackWrapper: *mut BeatmapDataCallbackWrapper,
    pub _spawnRotation: f32,
}
#[cfg(feature = "BeatmapObjectSpawnController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapObjectSpawnController => ""
    ."BeatmapObjectSpawnController"
);
#[cfg(feature = "BeatmapObjectSpawnController")]
impl std::ops::Deref for BeatmapObjectSpawnController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectSpawnController")]
impl std::ops::DerefMut for BeatmapObjectSpawnController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectSpawnController")]
impl BeatmapObjectSpawnController {
    #[cfg(feature = "BeatmapObjectSpawnController+InitData")]
    pub type InitData = crate::GlobalNamespace::BeatmapObjectSpawnController_InitData;
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
    pub fn HandleNoteDataCallback(
        &mut self,
        noteData: *mut NoteData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteDataCallback", (noteData))?;
        Ok(__cordl_ret)
    }
    pub fn HandleObstacleDataCallback(
        &mut self,
        obstacleData: *mut ObstacleData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleObstacleDataCallback", (obstacleData))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSliderDataCallback(
        &mut self,
        sliderNoteData: *mut SliderData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSliderDataCallback", (sliderNoteData))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSpawnRotationCallback(
        &mut self,
        beatmapEventData: *mut SpawnRotationBeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSpawnRotationCallback", (beatmapEventData))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn StopSpawning(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopSpawning", ())?;
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
    pub fn add_didInitEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didInitEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapObjectSpawnMovementData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapObjectSpawnMovementData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapObjectSpawnMovementData = __cordl_object
            .invoke("get_beatmapObjectSpawnMovementData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isInitialized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isInitialized", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_jumpDistance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_jumpDistance", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_jumpDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_jumpDuration", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_jumpOffsetY(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_jumpOffsetY", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_moveDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_moveDuration", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_noteJumpMovementSpeed(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_noteJumpMovementSpeed", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_noteLinesCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_noteLinesCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_noteLinesDistance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_noteLinesDistance", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_verticalLayerDistance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_verticalLayerDistance", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didInitEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didInitEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapObjectSpawnController")]
impl quest_hook::libil2cpp::ObjectType for BeatmapObjectSpawnController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapObjectSpawnController+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectSpawnController_InitData {
    __cordl_parent: crate::System::Object,
    pub beatsPerMinute: f32,
    pub noteLinesCount: i32,
    pub noteJumpMovementSpeed: f32,
    pub noteJumpValueType: crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteJumpValueType,
    pub noteJumpValue: f32,
}
#[cfg(feature = "BeatmapObjectSpawnController+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapObjectSpawnController_InitData => ""
    ."BeatmapObjectSpawnController/InitData"
);
#[cfg(feature = "BeatmapObjectSpawnController+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapObjectSpawnController_InitData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectSpawnController+InitData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapObjectSpawnController_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectSpawnController+InitData")]
impl crate::GlobalNamespace::BeatmapObjectSpawnController_InitData {
    pub fn New(
        beatsPerMinute: f32,
        noteLinesCount: i32,
        noteJumpMovementSpeed: f32,
        noteJumpValueType: crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteJumpValueType,
        noteJumpValue: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    beatsPerMinute,
                    noteLinesCount,
                    noteJumpMovementSpeed,
                    noteJumpValueType,
                    noteJumpValue,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        beatsPerMinute: f32,
        noteLinesCount: i32,
        noteJumpMovementSpeed: f32,
        noteJumpValueType: crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteJumpValueType,
        noteJumpValue: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    beatsPerMinute,
                    noteLinesCount,
                    noteJumpMovementSpeed,
                    noteJumpValueType,
                    noteJumpValue,
                ),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapObjectSpawnController+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapObjectSpawnController_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
