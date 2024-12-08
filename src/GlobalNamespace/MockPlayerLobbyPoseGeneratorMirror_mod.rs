#[cfg(feature = "MockPlayerLobbyPoseGeneratorMirror")]
#[repr(C)]
#[derive(Debug)]
pub struct MockPlayerLobbyPoseGeneratorMirror {
    __cordl_parent: MockPlayerLobbyPoseGenerator,
    pub _nodePoseSyncStateManager: *mut NodePoseSyncStateManager,
    pub _mirroredPlayer: *mut IConnectedPlayer,
}
#[cfg(feature = "MockPlayerLobbyPoseGeneratorMirror")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MockPlayerLobbyPoseGeneratorMirror => ""
    ."MockPlayerLobbyPoseGeneratorMirror"
);
#[cfg(feature = "MockPlayerLobbyPoseGeneratorMirror")]
impl std::ops::Deref for MockPlayerLobbyPoseGeneratorMirror {
    type Target = MockPlayerLobbyPoseGenerator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerLobbyPoseGeneratorMirror")]
impl std::ops::DerefMut for MockPlayerLobbyPoseGeneratorMirror {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerLobbyPoseGeneratorMirror")]
impl MockPlayerLobbyPoseGeneratorMirror {
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
    pub fn FindPlayerToMirror(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FindPlayerToMirror", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleOptionalAvatarDataReceived(
        &mut self,
        optionalAvatarDataPacket: *mut crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket,
        player: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleOptionalAvatarDataReceived",
                (optionalAvatarDataPacket, player),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        multiplayerSessionManager: *mut IMultiplayerSessionManager,
        nodePoseSyncStateManager: *mut NodePoseSyncStateManager,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (multiplayerSessionManager, nodePoseSyncStateManager),
            )?;
        Ok(__cordl_object)
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
    pub fn _ctor(
        &mut self,
        multiplayerSessionManager: *mut IMultiplayerSessionManager,
        nodePoseSyncStateManager: *mut NodePoseSyncStateManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (multiplayerSessionManager, nodePoseSyncStateManager))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MockPlayerLobbyPoseGeneratorMirror")]
impl quest_hook::libil2cpp::ObjectType for MockPlayerLobbyPoseGeneratorMirror {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
