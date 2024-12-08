#[cfg(feature = "MockPlayerLobbyPoseGeneratorAI")]
#[repr(C)]
#[derive(Debug)]
pub struct MockPlayerLobbyPoseGeneratorAI {
    __cordl_parent: MockPlayerLobbyPoseGenerator,
    pub _random: *mut crate::System::Random,
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
    in quest_hook::libil2cpp for MockPlayerLobbyPoseGeneratorAI => ""
    ."MockPlayerLobbyPoseGeneratorAI"
);
#[cfg(feature = "MockPlayerLobbyPoseGeneratorAI")]
impl std::ops::Deref for MockPlayerLobbyPoseGeneratorAI {
    type Target = MockPlayerLobbyPoseGenerator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerLobbyPoseGeneratorAI")]
impl std::ops::DerefMut for MockPlayerLobbyPoseGeneratorAI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerLobbyPoseGeneratorAI")]
impl MockPlayerLobbyPoseGeneratorAI {
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (multiplayerSessionManager))?;
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
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (multiplayerSessionManager))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MockPlayerLobbyPoseGeneratorAI")]
impl quest_hook::libil2cpp::ObjectType for MockPlayerLobbyPoseGeneratorAI {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
