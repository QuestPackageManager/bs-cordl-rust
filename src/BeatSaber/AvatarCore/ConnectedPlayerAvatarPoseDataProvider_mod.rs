#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerAvatarPoseDataProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerAvatarPoseDataProvider {
    __cordl_parent: crate::System::Object,
    pub poseDidChangeEvent: *mut crate::System::Action_1<
        crate::BeatSaber::AvatarCore::AvatarPoseData,
    >,
    pub _connectedPlayer: *mut crate::GlobalNamespace::IConnectedPlayer,
    pub _nodePoseSyncStateManager: *mut crate::GlobalNamespace::INodePoseSyncStateManager,
    pub _avatarPoseRestriction: *mut crate::BeatSaber::AvatarCore::IAvatarPoseRestriction,
    pub _currentPose: crate::BeatSaber::AvatarCore::AvatarPoseData,
}
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerAvatarPoseDataProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::AvatarCore::ConnectedPlayerAvatarPoseDataProvider =>
    "BeatSaber.AvatarCore"."ConnectedPlayerAvatarPoseDataProvider"
);
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerAvatarPoseDataProvider")]
impl std::ops::Deref
for crate::BeatSaber::AvatarCore::ConnectedPlayerAvatarPoseDataProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerAvatarPoseDataProvider")]
impl std::ops::DerefMut
for crate::BeatSaber::AvatarCore::ConnectedPlayerAvatarPoseDataProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerAvatarPoseDataProvider")]
impl crate::BeatSaber::AvatarCore::ConnectedPlayerAvatarPoseDataProvider {
    pub fn New(
        connectedPlayer: *mut crate::GlobalNamespace::IConnectedPlayer,
        nodePoseSyncStateManager: *mut crate::GlobalNamespace::INodePoseSyncStateManager,
        avatarPoseRestriction: *mut crate::BeatSaber::AvatarCore::IAvatarPoseRestriction,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (connectedPlayer, nodePoseSyncStateManager, avatarPoseRestriction),
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
        connectedPlayer: *mut crate::GlobalNamespace::IConnectedPlayer,
        nodePoseSyncStateManager: *mut crate::GlobalNamespace::INodePoseSyncStateManager,
        avatarPoseRestriction: *mut crate::BeatSaber::AvatarCore::IAvatarPoseRestriction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (connectedPlayer, nodePoseSyncStateManager, avatarPoseRestriction),
            )?;
        Ok(__cordl_ret)
    }
    pub fn add_poseDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<crate::BeatSaber::AvatarCore::AvatarPoseData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_poseDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_currentPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::AvatarCore::AvatarPoseData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::AvatarCore::AvatarPoseData = __cordl_object
            .invoke("get_currentPose", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_poseDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<crate::BeatSaber::AvatarCore::AvatarPoseData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_poseDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerAvatarPoseDataProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::AvatarCore::ConnectedPlayerAvatarPoseDataProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
