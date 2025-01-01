#[cfg(feature = "MockPlayerFiniteStateMachine")]
#[repr(C)]
#[derive(Debug)]
pub struct MockPlayerFiniteStateMachine {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "MockPlayerFiniteStateMachine")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MockPlayerFiniteStateMachine =>
    ""."MockPlayerFiniteStateMachine"
);
#[cfg(feature = "MockPlayerFiniteStateMachine")]
impl std::ops::Deref for crate::GlobalNamespace::MockPlayerFiniteStateMachine {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerFiniteStateMachine")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockPlayerFiniteStateMachine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerFiniteStateMachine")]
impl crate::GlobalNamespace::MockPlayerFiniteStateMachine {
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
    pub fn New(
        taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
        multiplayerSessionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
        gameplayRpcManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IGameplayRpcManager,
        >,
        menuRpcManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMenuRpcManager,
        >,
        beatmapDataProvider: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMockBeatmapDataProvider,
        >,
        lobbyPoseGenerator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MockPlayerLobbyPoseGenerator,
        >,
        gamePoseGenerator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MockPlayerGamePoseGenerator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    taskUtility,
                    multiplayerSessionManager,
                    gameplayRpcManager,
                    menuRpcManager,
                    beatmapDataProvider,
                    lobbyPoseGenerator,
                    gamePoseGenerator,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn RecommendBeatmap(
        &mut self,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
        beatmapLevelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecommendBeatmap", (beatmapDifficulty, beatmapLevelId))?;
        Ok(__cordl_ret.into())
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
        taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
        multiplayerSessionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
        gameplayRpcManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IGameplayRpcManager,
        >,
        menuRpcManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMenuRpcManager,
        >,
        beatmapDataProvider: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMockBeatmapDataProvider,
        >,
        lobbyPoseGenerator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MockPlayerLobbyPoseGenerator,
        >,
        gamePoseGenerator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MockPlayerGamePoseGenerator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    taskUtility,
                    multiplayerSessionManager,
                    gameplayRpcManager,
                    menuRpcManager,
                    beatmapDataProvider,
                    lobbyPoseGenerator,
                    gamePoseGenerator,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_gamePoseGenerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockPlayerGamePoseGenerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MockPlayerGamePoseGenerator,
        > = __cordl_object.invoke("get_gamePoseGenerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_inactiveByDefault(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_inactiveByDefault", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_leftHanded(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_leftHanded", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_obstaclesColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_obstaclesColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_saberAColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_saberAColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_saberBColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_saberBColor", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_obstaclesColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_obstaclesColor", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MockPlayerFiniteStateMachine")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MockPlayerFiniteStateMachine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MockPlayerFiniteStateMachine")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::MockPlayerFiniteStateMachine {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MockPlayerFiniteStateMachine")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::MockPlayerFiniteStateMachine {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
