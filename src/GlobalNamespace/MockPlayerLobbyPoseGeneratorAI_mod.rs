#[cfg(feature = "MockPlayerLobbyPoseGeneratorAI")]
#[repr(C)]
#[derive(Debug)]
pub struct MockPlayerLobbyPoseGeneratorAI {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MockPlayerLobbyPoseGenerator,
    >,
    pub _random: quest_hook::libil2cpp::Gc<crate::System::Random>,
    pub _headPose: crate::UnityEngine::Pose,
    pub _leftHandPose: crate::UnityEngine::Pose,
    pub _rightHandPose: crate::UnityEngine::Pose,
    pub _lastHeadPoseTarget: crate::UnityEngine::Pose,
    pub _lastLeftHandPoseTarget: crate::UnityEngine::Pose,
    pub _lastRightHandPoseTarget: crate::UnityEngine::Pose,
    pub _lastTargetTime: i64,
    pub _headPoseTarget: crate::UnityEngine::Pose,
    pub _leftHandPoseTarget: crate::UnityEngine::Pose,
    pub _rightHandPoseTarget: crate::UnityEngine::Pose,
    pub _nextTargetTime: i64,
}
#[cfg(feature = "MockPlayerLobbyPoseGeneratorAI")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MockPlayerLobbyPoseGeneratorAI
    => ""."MockPlayerLobbyPoseGeneratorAI"
);
#[cfg(feature = "MockPlayerLobbyPoseGeneratorAI")]
impl std::ops::Deref for crate::GlobalNamespace::MockPlayerLobbyPoseGeneratorAI {
    type Target = quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MockPlayerLobbyPoseGenerator,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerLobbyPoseGeneratorAI")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockPlayerLobbyPoseGeneratorAI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerLobbyPoseGeneratorAI")]
impl crate::GlobalNamespace::MockPlayerLobbyPoseGeneratorAI {
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        multiplayerSessionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (multiplayerSessionManager))?;
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
        multiplayerSessionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (multiplayerSessionManager))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MockPlayerLobbyPoseGeneratorAI")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MockPlayerLobbyPoseGeneratorAI {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
