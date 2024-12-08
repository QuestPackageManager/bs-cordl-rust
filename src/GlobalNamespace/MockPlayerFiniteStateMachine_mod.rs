#[cfg(feature = "MockPlayerFiniteStateMachine")]
#[repr(C)]
#[derive(Debug)]
pub struct MockPlayerFiniteStateMachine {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "MockPlayerFiniteStateMachine")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MockPlayerFiniteStateMachine => ""
    ."MockPlayerFiniteStateMachine"
);
#[cfg(feature = "MockPlayerFiniteStateMachine")]
impl std::ops::Deref for MockPlayerFiniteStateMachine {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerFiniteStateMachine")]
impl std::ops::DerefMut for MockPlayerFiniteStateMachine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerFiniteStateMachine")]
impl MockPlayerFiniteStateMachine {
    pub fn set_obstaclesColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_obstaclesColor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_gamePoseGenerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut MockPlayerGamePoseGenerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MockPlayerGamePoseGenerator = __cordl_object
            .invoke("get_gamePoseGenerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_leftHanded(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_leftHanded", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_saberBColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_saberBColor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_saberAColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_saberAColor", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetIsReady(
        &mut self,
        isReady: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIsReady", (isReady))?;
        Ok(__cordl_ret)
    }
    pub fn set_saberAColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_saberAColor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_saberBColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_saberBColor", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_inactiveByDefault(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_inactiveByDefault", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        multiplayerSessionManager: *mut IMultiplayerSessionManager,
        gameplayRpcManager: *mut IGameplayRpcManager,
        menuRpcManager: *mut IMenuRpcManager,
        beatmapDataProvider: *mut IMockBeatmapDataProvider,
        lobbyPoseGenerator: *mut MockPlayerLobbyPoseGenerator,
        gamePoseGenerator: *mut MockPlayerGamePoseGenerator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    multiplayerSessionManager,
                    gameplayRpcManager,
                    menuRpcManager,
                    beatmapDataProvider,
                    lobbyPoseGenerator,
                    gamePoseGenerator,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn set_leftHanded(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_leftHanded", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_obstaclesColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_obstaclesColor", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_inactiveByDefault(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_inactiveByDefault", ())?;
        Ok(__cordl_ret)
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tick", ())?;
        Ok(__cordl_ret)
    }
    pub fn RecommendBeatmap(
        &mut self,
        beatmapDifficulty: BeatmapDifficulty,
        beatmapLevelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecommendBeatmap", (beatmapDifficulty, beatmapLevelId))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        multiplayerSessionManager: *mut IMultiplayerSessionManager,
        gameplayRpcManager: *mut IGameplayRpcManager,
        menuRpcManager: *mut IMenuRpcManager,
        beatmapDataProvider: *mut IMockBeatmapDataProvider,
        lobbyPoseGenerator: *mut MockPlayerLobbyPoseGenerator,
        gamePoseGenerator: *mut MockPlayerGamePoseGenerator,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    multiplayerSessionManager,
                    gameplayRpcManager,
                    menuRpcManager,
                    beatmapDataProvider,
                    lobbyPoseGenerator,
                    gamePoseGenerator,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MockPlayerFiniteStateMachine")]
impl quest_hook::libil2cpp::ObjectType for MockPlayerFiniteStateMachine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
