#[cfg(feature = "BTSCharacterSpawnAnimationController")]
#[repr(C)]
#[derive(Debug)]
pub struct BTSCharacterSpawnAnimationController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _spawnCharacterPlayableDirector: *mut crate::UnityEngine::Playables::PlayableDirector,
    pub _jumpReceiver: *mut crate::GlobalNamespace::JumpReceiver,
    pub _appearAnimationEndTime: f32,
    pub _disappearAnimationStartTime: f32,
    pub _rimLightColorSetter: *mut crate::GlobalNamespace::MaterialPropertyBlockColorSetter,
    pub _rimLightIntensityAnimator: *mut crate::GlobalNamespace::MaterialPropertyBlockFloatAnimator,
    pub _rimLightEdgeStartAnimator: *mut crate::GlobalNamespace::MaterialPropertyBlockFloatAnimator,
    pub _songSpeedData: *mut crate::GlobalNamespace::SongSpeedData,
    pub animationFinishedEvent: *mut crate::System::Action,
    pub _characterActivationTrack: *mut crate::UnityEngine::Timeline::ActivationTrack,
    pub _currentBtsCharacter: *mut crate::GlobalNamespace::BTSCharacter,
    pub _defaultSpawnCharacterDuration: f32,
    pub _playableDirectorTimeBeforePause: f64,
    pub _animatorNormalizedTimeBeforePause: f32,
}
#[cfg(feature = "BTSCharacterSpawnAnimationController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BTSCharacterSpawnAnimationController => ""
    ."BTSCharacterSpawnAnimationController"
);
#[cfg(feature = "BTSCharacterSpawnAnimationController")]
impl std::ops::Deref for crate::GlobalNamespace::BTSCharacterSpawnAnimationController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BTSCharacterSpawnAnimationController")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BTSCharacterSpawnAnimationController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BTSCharacterSpawnAnimationController")]
impl crate::GlobalNamespace::BTSCharacterSpawnAnimationController {
    pub const kCharacterActivationStreamName: &'static str = "CharacterActivationTrack";
    pub fn EndEarlyAnimation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndEarlyAnimation", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleSpawnCharacterPlayableDirectorStopped(
        &mut self,
        playableDirector: *mut crate::UnityEngine::Playables::PlayableDirector,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSpawnCharacterPlayableDirectorStopped", (playableDirector))?;
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
    pub fn PauseAnimation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PauseAnimation", ())?;
        Ok(__cordl_ret)
    }
    pub fn PlayAnimation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayAnimation", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResumeAnimation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResumeAnimation", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetCharacter(
        &mut self,
        btsCharacter: *mut crate::GlobalNamespace::BTSCharacter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCharacter", (btsCharacter))?;
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
    pub fn StopAnimation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopAnimation", ())?;
        Ok(__cordl_ret)
    }
    pub fn WillResumeAnimation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WillResumeAnimation", ())?;
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
    pub fn add_animationFinishedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_animationFinishedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_characterActivationTrack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::Timeline::ActivationTrack,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Timeline::ActivationTrack = __cordl_object
            .invoke("get_characterActivationTrack", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_duration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_duration", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isCharacterVisible(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isCharacterVisible", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_animationFinishedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_animationFinishedEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BTSCharacterSpawnAnimationController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BTSCharacterSpawnAnimationController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
