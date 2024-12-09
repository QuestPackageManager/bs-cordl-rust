#[cfg(feature = "MultiplayerOutroAnimationController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerOutroAnimationController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _outroPlayableDirector: *mut crate::UnityEngine::Playables::PlayableDirector,
    pub _playerTimelineTrackNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _ringTimelineTrackNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _resultsTimelineTrackNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _playerTimelinePropertyNames: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::PropertyName,
    >,
    pub _resultsTimelinePropertyNames: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::PropertyName,
    >,
    pub _localPlayerTrackName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _localPlayerTimelinePropertyName: crate::UnityEngine::PropertyName,
    pub _badgeTimelineTrackNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _badgeTimelinePropertyNames: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::PropertyName,
    >,
    pub _badgeStartTransform: *mut crate::UnityEngine::Transform,
    pub _badgeMidTransform: *mut crate::UnityEngine::Transform,
    pub _songPreviewTrackName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _resultsMocksActivationTrack: *mut quest_hook::libil2cpp::Il2CppString,
    pub _multiplayerScoreRingManager: *mut crate::GlobalNamespace::MultiplayerScoreRingManager,
    pub _multiplayerResultsPyramidView: *mut crate::GlobalNamespace::MultiplayerResultsPyramidView,
    pub _multiplayerPlayersManager: *mut crate::GlobalNamespace::MultiplayerPlayersManager,
    pub _multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
    pub _sceneSetupData: *mut crate::GlobalNamespace::GameplayCoreSceneSetupData,
    pub _layoutProvider: *mut crate::GlobalNamespace::MultiplayerLayoutProvider,
    pub _onCompleted: *mut crate::System::Action,
}
#[cfg(feature = "MultiplayerOutroAnimationController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerOutroAnimationController => ""
    ."MultiplayerOutroAnimationController"
);
#[cfg(feature = "MultiplayerOutroAnimationController")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerOutroAnimationController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerOutroAnimationController")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerOutroAnimationController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerOutroAnimationController")]
impl crate::GlobalNamespace::MultiplayerOutroAnimationController {
    pub fn AnimateOutro(
        &mut self,
        multiplayerResultsData: *mut crate::GlobalNamespace::MultiplayerResultsData,
        onCompleted: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AnimateOutro", (multiplayerResultsData, onCompleted))?;
        Ok(__cordl_ret)
    }
    pub fn BindOutroTimeline(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindOutroTimeline", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindRingsAndAudio(
        &mut self,
        rings: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::GameObject,
        >,
        isMock: bool,
        isDuel: bool,
        resultsMocks: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindRingsAndAudio", (rings, isMock, isDuel, resultsMocks))?;
        Ok(__cordl_ret)
    }
    pub fn Completed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Completed", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerSpawningDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerSpawningDidFinish", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnValidate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnValidate", ())?;
        Ok(__cordl_ret)
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
}
#[cfg(feature = "MultiplayerOutroAnimationController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerOutroAnimationController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
