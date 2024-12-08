#[cfg(feature = "MultiplayerLocalActiveClient")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLocalActiveClient {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _playerTransforms: *mut crate::GlobalNamespace::PlayerTransforms,
    pub _audioTimeSyncController: *mut crate::GlobalNamespace::AudioTimeSyncController,
    pub _scoreSyncStateManager: *mut crate::GlobalNamespace::IScoreSyncStateManager,
    pub _nodePoseSyncStateManager: *mut crate::GlobalNamespace::INodePoseSyncStateManager,
    pub _rpcManager: *mut crate::GlobalNamespace::IGameplayRpcManager,
    pub _beatmapObjectManager: *mut crate::GlobalNamespace::BeatmapObjectManager,
    pub _scoreController: *mut crate::GlobalNamespace::IScoreController,
    pub _comboController: *mut crate::GlobalNamespace::ComboController,
}
#[cfg(feature = "MultiplayerLocalActiveClient")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerLocalActiveClient =>
    ""."MultiplayerLocalActiveClient"
);
#[cfg(feature = "MultiplayerLocalActiveClient")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerLocalActiveClient {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalActiveClient")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerLocalActiveClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalActiveClient")]
impl crate::GlobalNamespace::MultiplayerLocalActiveClient {
    pub fn HandleComboDidChange(
        &mut self,
        combo: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleComboDidChange", (combo))?;
        Ok(__cordl_ret)
    }
    pub fn HandleMultiplierDidChange(
        &mut self,
        multiplier: i32,
        multiplierProgress: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMultiplierDidChange", (multiplier, multiplierProgress))?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteWasAdded(
        &mut self,
        noteData: *mut crate::GlobalNamespace::NoteData,
        noteSpawnData: crate::GlobalNamespace::BeatmapObjectSpawnMovementData_NoteSpawnData,
        rotation: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasAdded", (noteData, noteSpawnData, rotation))?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteWasCut(
        &mut self,
        noteController: *mut crate::GlobalNamespace::NoteController,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::NoteCutInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasCut", (noteController, noteCutInfo))?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteWasMissed(
        &mut self,
        noteController: *mut crate::GlobalNamespace::NoteController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasMissed", (noteController))?;
        Ok(__cordl_ret)
    }
    pub fn HandleObstacleWasAdded(
        &mut self,
        obstacleData: *mut crate::GlobalNamespace::ObstacleData,
        obstacleSpawnData: crate::GlobalNamespace::BeatmapObjectSpawnMovementData_ObstacleSpawnData,
        rotation: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleObstacleWasAdded",
                (obstacleData, obstacleSpawnData, rotation),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleScoreDidChange(
        &mut self,
        multipliedScore: i32,
        modifiedScore: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleScoreDidChange", (multipliedScore, modifiedScore))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSliderWasAdded(
        &mut self,
        sliderData: *mut crate::GlobalNamespace::SliderData,
        sliderSpawnData: crate::GlobalNamespace::BeatmapObjectSpawnMovementData_SliderSpawnData,
        rotation: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSliderWasAdded", (sliderData, sliderSpawnData, rotation))?;
        Ok(__cordl_ret)
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
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
#[cfg(feature = "MultiplayerLocalActiveClient")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerLocalActiveClient {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
