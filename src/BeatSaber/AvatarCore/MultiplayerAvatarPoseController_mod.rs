#[cfg(feature = "BeatSaber+AvatarCore+MultiplayerAvatarPoseController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerAvatarPoseController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _leftSaberTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _rightSaberTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _headTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _nodePoseSyncStateManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::INodePoseSyncStateManager,
    >,
    pub _avatarPoseRestriction: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::AvatarCore::IAvatarPoseRestriction,
    >,
    pub _connectedPlayer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IConnectedPlayer,
    >,
    pub didUpdatePoseEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<crate::UnityEngine::Vector3>,
    >,
}
#[cfg(feature = "BeatSaber+AvatarCore+MultiplayerAvatarPoseController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::AvatarCore::MultiplayerAvatarPoseController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.AvatarCore";
    const CLASS_NAME: &'static str = "MultiplayerAvatarPoseController";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
    pub fn add_didUpdatePoseEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didUpdatePoseEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didUpdatePoseEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didUpdatePoseEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_connectedPlayer(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_connectedPlayer", (value))?;
        Ok(__cordl_ret.into())
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
