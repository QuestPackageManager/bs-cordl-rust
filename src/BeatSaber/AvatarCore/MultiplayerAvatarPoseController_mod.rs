#[cfg(feature = "BeatSaber+AvatarCore+MultiplayerAvatarPoseController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerAvatarPoseController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _leftSaberTransform: *mut crate::UnityEngine::Transform,
    pub _rightSaberTransform: *mut crate::UnityEngine::Transform,
    pub _headTransform: *mut crate::UnityEngine::Transform,
    pub _nodePoseSyncStateManager: *mut crate::GlobalNamespace::INodePoseSyncStateManager,
    pub _avatarPoseRestriction: *mut crate::BeatSaber::AvatarCore::IAvatarPoseRestriction,
    pub _connectedPlayer: *mut crate::GlobalNamespace::IConnectedPlayer,
    pub didUpdatePoseEvent: *mut crate::System::Action_1<crate::UnityEngine::Vector3>,
}
#[cfg(feature = "BeatSaber+AvatarCore+MultiplayerAvatarPoseController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::AvatarCore::MultiplayerAvatarPoseController => "BeatSaber.AvatarCore"
    ."MultiplayerAvatarPoseController"
);
#[cfg(feature = "BeatSaber+AvatarCore+MultiplayerAvatarPoseController")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::MultiplayerAvatarPoseController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+MultiplayerAvatarPoseController")]
impl std::ops::DerefMut
for crate::BeatSaber::AvatarCore::MultiplayerAvatarPoseController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+MultiplayerAvatarPoseController")]
impl crate::BeatSaber::AvatarCore::MultiplayerAvatarPoseController {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
    pub fn add_didUpdatePoseEvent(
        &mut self,
        value: *mut crate::System::Action_1<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didUpdatePoseEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didUpdatePoseEvent(
        &mut self,
        value: *mut crate::System::Action_1<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didUpdatePoseEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_connectedPlayer(
        &mut self,
        value: *mut crate::GlobalNamespace::IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_connectedPlayer", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+MultiplayerAvatarPoseController")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::AvatarCore::MultiplayerAvatarPoseController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
