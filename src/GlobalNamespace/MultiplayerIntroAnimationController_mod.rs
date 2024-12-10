#[cfg(feature = "MultiplayerIntroAnimationController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerIntroAnimationController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _introPlayableDirector: *mut crate::UnityEngine::Playables::PlayableDirector,
    pub _playerTimelineTrackNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _ringTimelineTrackNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _playerTimelinePropertyNames: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::PropertyName,
    >,
    pub _localPlayerTrackName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _localPlayerRingTrackName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _localPlayerTimelinePropertyName: crate::UnityEngine::PropertyName,
    pub _firstConnectedPlayerStart: f32,
    pub _spawnDuration: f32,
    pub _endMarkerName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _scoreRingManager: *mut crate::GlobalNamespace::MultiplayerScoreRingManager,
    pub _multiplayerPlayersManager: *mut crate::GlobalNamespace::MultiplayerPlayersManager,
    pub _multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
    pub _layoutProvider: *mut crate::GlobalNamespace::MultiplayerLayoutProvider,
    pub _onCompleted: *mut crate::System::Action,
    pub _introDuration: i64,
    pub _bindingFinished: bool,
}
#[cfg(feature = "MultiplayerIntroAnimationController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerIntroAnimationController => ""
    ."MultiplayerIntroAnimationController"
);
#[cfg(feature = "MultiplayerIntroAnimationController")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerIntroAnimationController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerIntroAnimationController")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerIntroAnimationController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerIntroAnimationController")]
impl crate::GlobalNamespace::MultiplayerIntroAnimationController {
    pub fn BindRingsAndSetTiming(
        &mut self,
        connectedPlayersCount: i32,
        connectedRings: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::GameObject>,
        >,
        localRing: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "BindRingsAndSetTiming",
                (connectedPlayersCount, connectedRings, localRing),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn BindTimeline(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindTimeline", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculatePlayerIndexSequence(
        &mut self,
        allActivePlayer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::GlobalNamespace::IConnectedPlayer,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::Queue_1<i32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Queue_1<i32>,
        > = __cordl_object.invoke("CalculatePlayerIndexSequence", (allActivePlayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Completed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Completed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFullIntroAnimationTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("GetFullIntroAnimationTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnValidate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnValidate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PlayIntroAnimation(
        &mut self,
        maxDesiredIntroAnimationDuration: f32,
        onCompleted: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "PlayIntroAnimation",
                (maxDesiredIntroAnimationDuration, onCompleted),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBeforeIntroValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBeforeIntroValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TransitionToAfterIntroAnimationState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TransitionToAfterIntroAnimationState", ())?;
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
}
#[cfg(feature = "MultiplayerIntroAnimationController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerIntroAnimationController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
