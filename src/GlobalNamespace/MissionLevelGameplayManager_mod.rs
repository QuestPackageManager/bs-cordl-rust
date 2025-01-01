#[cfg(feature = "MissionLevelGameplayManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionLevelGameplayManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _missionObjectiveCheckersManager: *mut crate::GlobalNamespace::MissionObjectiveCheckersManager,
    pub _gameScenesManager: *mut crate::GlobalNamespace::GameScenesManager,
    pub _gameSongController: *mut crate::GlobalNamespace::GameSongController,
    pub _gameEnergyCounter: *mut crate::GlobalNamespace::GameEnergyCounter,
    pub _pauseController: *mut crate::GlobalNamespace::PauseController,
    pub _initData: *mut crate::GlobalNamespace::MissionLevelGameplayManager_InitData,
    pub levelWillStartIntroEvent: *mut crate::System::Action,
    pub levelDidStartEvent: *mut crate::System::Action,
    pub levelFinishedEvent: *mut crate::System::Action,
    pub levelFailedEvent: *mut crate::System::Action,
    pub _gameState: crate::GlobalNamespace::MissionLevelGameplayManager_GameState,
    pub _prePauseGameState: crate::GlobalNamespace::MissionLevelGameplayManager_GameState,
    pub _levelEndStateType: crate::GlobalNamespace::LevelCompletionResults_LevelEndStateType,
}
#[cfg(feature = "MissionLevelGameplayManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MissionLevelGameplayManager =>
    ""."MissionLevelGameplayManager"
);
#[cfg(feature = "MissionLevelGameplayManager")]
impl std::ops::Deref for crate::GlobalNamespace::MissionLevelGameplayManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelGameplayManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionLevelGameplayManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelGameplayManager")]
impl crate::GlobalNamespace::MissionLevelGameplayManager {
    #[cfg(feature = "MissionLevelGameplayManager+GameState")]
    pub type GameState = crate::GlobalNamespace::MissionLevelGameplayManager_GameState;
    #[cfg(feature = "MissionLevelGameplayManager+InitData")]
    pub type InitData = crate::GlobalNamespace::MissionLevelGameplayManager_InitData;
    #[cfg(feature = "MissionLevelGameplayManager+_Start_d__26")]
    pub type _Start_d__26 = crate::GlobalNamespace::MissionLevelGameplayManager__Start_d__26;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleGameEnergyDidReach0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleGameEnergyDidReach0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMissionObjectiveCheckersManagerObjectiveDidFail(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMissionObjectiveCheckersManagerObjectiveDidFail", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlePauseControllerCanPause(
        &mut self,
        canPause: quest_hook::libil2cpp::Gc<crate::System::Action_1<bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePauseControllerCanPause", (canPause))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlePauseControllerDidPause(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePauseControllerDidPause", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlePauseControllerDidResume(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePauseControllerDidResume", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlePauseControllerDidStartResume(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePauseControllerDidStartResume", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSongDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSongDidFinish", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("Start", ())?;
        Ok(__cordl_ret.into())
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
    pub fn add_levelDidStartEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_levelDidStartEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_levelFailedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_levelFailedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_levelFinishedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_levelFinishedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_levelWillStartIntroEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_levelWillStartIntroEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_levelEndStateType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::LevelCompletionResults_LevelEndStateType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::LevelCompletionResults_LevelEndStateType = __cordl_object
            .invoke("get_levelEndStateType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_levelDidStartEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_levelDidStartEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_levelFailedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_levelFailedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_levelFinishedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_levelFinishedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_levelWillStartIntroEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_levelWillStartIntroEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MissionLevelGameplayManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionLevelGameplayManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MissionLevelGameplayManager")]
impl AsRef<crate::GlobalNamespace::ILevelEndActions>
for crate::GlobalNamespace::MissionLevelGameplayManager {
    fn as_ref(&self) -> &crate::GlobalNamespace::ILevelEndActions {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MissionLevelGameplayManager")]
impl AsMut<crate::GlobalNamespace::ILevelEndActions>
for crate::GlobalNamespace::MissionLevelGameplayManager {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::ILevelEndActions {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MissionLevelGameplayManager")]
impl AsRef<crate::GlobalNamespace::ILevelStartController>
for crate::GlobalNamespace::MissionLevelGameplayManager {
    fn as_ref(&self) -> &crate::GlobalNamespace::ILevelStartController {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MissionLevelGameplayManager")]
impl AsMut<crate::GlobalNamespace::ILevelStartController>
for crate::GlobalNamespace::MissionLevelGameplayManager {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::ILevelStartController {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MissionLevelGameplayManager+GameState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissionLevelGameplayManager_GameState {
    Failed = 5i32,
    Finished = 4i32,
    Intro = 0i32,
    Paused = 2i32,
    Playing = 1i32,
    Resuming = 3i32,
}
#[cfg(feature = "MissionLevelGameplayManager+GameState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MissionLevelGameplayManager_GameState => ""
    ."MissionLevelGameplayManager/GameState"
);
#[cfg(feature = "MissionLevelGameplayManager+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionLevelGameplayManager_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub failOn0Energy: bool,
}
#[cfg(feature = "MissionLevelGameplayManager+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MissionLevelGameplayManager_InitData => ""
    ."MissionLevelGameplayManager/InitData"
);
#[cfg(feature = "MissionLevelGameplayManager+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::MissionLevelGameplayManager_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelGameplayManager+InitData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MissionLevelGameplayManager_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelGameplayManager+InitData")]
impl crate::GlobalNamespace::MissionLevelGameplayManager_InitData {
    pub fn New(
        failOn0Energy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (failOn0Energy))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        failOn0Energy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (failOn0Energy))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MissionLevelGameplayManager+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionLevelGameplayManager_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
