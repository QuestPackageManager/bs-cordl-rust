#[cfg(feature = "MissionMapAnimationController")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionMapAnimationController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _missionNodesManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MissionNodesManager,
    >,
    pub _mapScrollView: quest_hook::libil2cpp::Gc<crate::HMUI::ScrollView>,
    pub _startDelay: f32,
    pub _stageAnimationStartDelay: f32,
    pub _missionConnectionAnimationStartDelay: f32,
    pub _missionConnectionAnimationSeparationTime: f32,
    pub _stageAnimationDuration: f32,
    pub _shockwaveEffect: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MenuShockwave,
    >,
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ScrollToTopMostNotClearedMission(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScrollToTopMostNotClearedMission", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateClearedNodeStateCoroutine(
        &mut self,
        lastClearedMissionNode: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionNode,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object
            .invoke("UpdateClearedNodeStateCoroutine", (lastClearedMissionNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateMissionMapAfterMissionWasCleared(
        &mut self,
        animated: bool,
        finishCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UpdateMissionMapAfterMissionWasCleared",
                (animated, finishCallback),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateMissionMapCoroutine(
        &mut self,
        lastClearedMissionNode: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionNode,
        >,
        finishCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object
            .invoke(
                "UpdateMissionMapCoroutine",
                (lastClearedMissionNode, finishCallback),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateNodesAndConnectionCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("UpdateNodesAndConnectionCoroutine", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateStageCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("UpdateStageCoroutine", ())?;
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
    pub fn get_animatedUpdateIsRequired(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_animatedUpdateIsRequired", ())?;
        Ok(__cordl_ret.into())
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
