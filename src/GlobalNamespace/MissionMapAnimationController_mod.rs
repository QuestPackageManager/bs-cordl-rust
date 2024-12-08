#[cfg(feature = "MissionMapAnimationController")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionMapAnimationController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _missionNodesManager: *mut crate::GlobalNamespace::MissionNodesManager,
    pub _mapScrollView: *mut crate::HMUI::ScrollView,
    pub _startDelay: f32,
    pub _stageAnimationStartDelay: f32,
    pub _missionConnectionAnimationStartDelay: f32,
    pub _missionConnectionAnimationSeparationTime: f32,
    pub _stageAnimationDuration: f32,
    pub _shockwaveEffect: *mut crate::GlobalNamespace::MenuShockwave,
}
#[cfg(feature = "MissionMapAnimationController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MissionMapAnimationController
    => ""."MissionMapAnimationController"
);
#[cfg(feature = "MissionMapAnimationController")]
impl std::ops::Deref for crate::GlobalNamespace::MissionMapAnimationController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionMapAnimationController")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionMapAnimationController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionMapAnimationController")]
impl crate::GlobalNamespace::MissionMapAnimationController {
    #[cfg(
        feature = "MissionMapAnimationController+_UpdateClearedNodeStateCoroutine_d__13"
    )]
    pub type _UpdateClearedNodeStateCoroutine_d__13 = crate::GlobalNamespace::MissionMapAnimationController__UpdateClearedNodeStateCoroutine_d__13;
    #[cfg(feature = "MissionMapAnimationController+_UpdateMissionMapCoroutine_d__12")]
    pub type _UpdateMissionMapCoroutine_d__12 = crate::GlobalNamespace::MissionMapAnimationController__UpdateMissionMapCoroutine_d__12;
    #[cfg(
        feature = "MissionMapAnimationController+_UpdateNodesAndConnectionCoroutine_d__15"
    )]
    pub type _UpdateNodesAndConnectionCoroutine_d__15 = crate::GlobalNamespace::MissionMapAnimationController__UpdateNodesAndConnectionCoroutine_d__15;
    #[cfg(feature = "MissionMapAnimationController+_UpdateStageCoroutine_d__14")]
    pub type _UpdateStageCoroutine_d__14 = crate::GlobalNamespace::MissionMapAnimationController__UpdateStageCoroutine_d__14;
    #[cfg(feature = "MissionMapAnimationController+__c")]
    pub type __c = crate::GlobalNamespace::MissionMapAnimationController___c;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ScrollToTopMostNotClearedMission(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScrollToTopMostNotClearedMission", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateClearedNodeStateCoroutine(
        &mut self,
        lastClearedMissionNode: *mut crate::GlobalNamespace::MissionNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("UpdateClearedNodeStateCoroutine", (lastClearedMissionNode))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateMissionMapAfterMissionWasCleared(
        &mut self,
        animated: bool,
        finishCallback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UpdateMissionMapAfterMissionWasCleared",
                (animated, finishCallback),
            )?;
        Ok(__cordl_ret)
    }
    pub fn UpdateMissionMapCoroutine(
        &mut self,
        lastClearedMissionNode: *mut crate::GlobalNamespace::MissionNode,
        finishCallback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke(
                "UpdateMissionMapCoroutine",
                (lastClearedMissionNode, finishCallback),
            )?;
        Ok(__cordl_ret)
    }
    pub fn UpdateNodesAndConnectionCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("UpdateNodesAndConnectionCoroutine", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateStageCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("UpdateStageCoroutine", ())?;
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
    pub fn get_animatedUpdateIsRequired(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_animatedUpdateIsRequired", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MissionMapAnimationController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionMapAnimationController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
