#[cfg(feature = "MockPlayerGamePoseGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct MockPlayerGamePoseGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
    pub gameplayRpcManager: *mut crate::GlobalNamespace::IGameplayRpcManager,
    pub leftHanded: bool,
    pub mockNodePoseSyncStateSender: *mut crate::GlobalNamespace::MockNodePoseSyncStateSender,
    pub mockScoreSyncStateSender: *mut crate::GlobalNamespace::MockScoreSyncStateSender,
}
#[cfg(feature = "MockPlayerGamePoseGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MockPlayerGamePoseGenerator =>
    ""."MockPlayerGamePoseGenerator"
);
#[cfg(feature = "MockPlayerGamePoseGenerator")]
impl std::ops::Deref for crate::GlobalNamespace::MockPlayerGamePoseGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerGamePoseGenerator")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockPlayerGamePoseGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerGamePoseGenerator")]
impl crate::GlobalNamespace::MockPlayerGamePoseGenerator {
    pub fn CreateEmptyLevelCompletionResults(
        levelEndStateType: crate::GlobalNamespace::LevelCompletionResults_LevelEndStateType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LevelCompletionResults>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateEmptyLevelCompletionResults", (levelEndStateType))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        introStartTime: i64,
        beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockBeatmapData>,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        onSongFinished: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (introStartTime, beatmapData, gameplayModifiers, onSongFinished),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        multiplayerSessionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
        gameplayRpcManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IGameplayRpcManager,
        >,
        leftHanded: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (multiplayerSessionManager, gameplayRpcManager, leftHanded),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn SimulateFail(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SimulateFail", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SimulateGiveUp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SimulateGiveUp", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tick", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        multiplayerSessionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
        gameplayRpcManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IGameplayRpcManager,
        >,
        leftHanded: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (multiplayerSessionManager, gameplayRpcManager, leftHanded),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MockPlayerGamePoseGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MockPlayerGamePoseGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MockPlayerGamePoseGenerator")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::MockPlayerGamePoseGenerator {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MockPlayerGamePoseGenerator")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::MockPlayerGamePoseGenerator {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
