#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerAvatarPoseDataProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerAvatarPoseDataProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub poseDidChangeEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<crate::BeatSaber::AvatarCore::AvatarPoseData>,
    >,
    pub _connectedPlayer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IConnectedPlayer,
    >,
    pub _nodePoseSyncStateManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::INodePoseSyncStateManager,
    >,
    pub _avatarPoseRestriction: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::AvatarCore::IAvatarPoseRestriction,
    >,
    pub _currentPose: crate::BeatSaber::AvatarCore::AvatarPoseData,
}
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerAvatarPoseDataProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::AvatarCore::ConnectedPlayerAvatarPoseDataProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.AvatarCore";
    const CLASS_NAME: &'static str = "ConnectedPlayerAvatarPoseDataProvider";
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
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerAvatarPoseDataProvider")]
impl std::ops::Deref
for crate::BeatSaber::AvatarCore::ConnectedPlayerAvatarPoseDataProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
        nodePoseSyncStateManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::INodePoseSyncStateManager,
        >,
        avatarPoseRestriction: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::IAvatarPoseRestriction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (connectedPlayer, nodePoseSyncStateManager, avatarPoseRestriction),
            )?;
        Ok(__cordl_object.into())
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
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
        nodePoseSyncStateManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::INodePoseSyncStateManager,
        >,
        avatarPoseRestriction: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::IAvatarPoseRestriction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (connectedPlayer, nodePoseSyncStateManager, avatarPoseRestriction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn add_poseDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::BeatSaber::AvatarCore::AvatarPoseData>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_poseDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::AvatarCore::AvatarPoseData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::AvatarCore::AvatarPoseData = __cordl_object
            .invoke("get_currentPose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_poseDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::BeatSaber::AvatarCore::AvatarPoseData>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_poseDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerAvatarPoseDataProvider")]
impl AsRef<crate::BeatSaber::AvatarCore::IAvatarPoseDataProvider>
for crate::BeatSaber::AvatarCore::ConnectedPlayerAvatarPoseDataProvider {
    fn as_ref(&self) -> &crate::BeatSaber::AvatarCore::IAvatarPoseDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerAvatarPoseDataProvider")]
impl AsMut<crate::BeatSaber::AvatarCore::IAvatarPoseDataProvider>
for crate::BeatSaber::AvatarCore::ConnectedPlayerAvatarPoseDataProvider {
    fn as_mut(&mut self) -> &mut crate::BeatSaber::AvatarCore::IAvatarPoseDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerAvatarPoseDataProvider")]
impl AsRef<crate::Zenject::ITickable>
for crate::BeatSaber::AvatarCore::ConnectedPlayerAvatarPoseDataProvider {
    fn as_ref(&self) -> &crate::Zenject::ITickable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerAvatarPoseDataProvider")]
impl AsMut<crate::Zenject::ITickable>
for crate::BeatSaber::AvatarCore::ConnectedPlayerAvatarPoseDataProvider {
    fn as_mut(&mut self) -> &mut crate::Zenject::ITickable {
        unsafe { std::mem::transmute(self) }
    }
}
