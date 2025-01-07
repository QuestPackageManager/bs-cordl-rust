#[cfg(feature = "BTSCharacterSpawnAnimationController")]
#[repr(C)]
#[derive(Debug)]
pub struct BTSCharacterSpawnAnimationController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _spawnCharacterPlayableDirector: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Playables::PlayableDirector,
    >,
    pub _jumpReceiver: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::JumpReceiver>,
    pub _appearAnimationEndTime: f32,
    pub _disappearAnimationStartTime: f32,
    pub _rimLightColorSetter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MaterialPropertyBlockColorSetter,
    >,
    pub _rimLightIntensityAnimator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MaterialPropertyBlockFloatAnimator,
    >,
    pub _rimLightEdgeStartAnimator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MaterialPropertyBlockFloatAnimator,
    >,
    pub _songSpeedData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SongSpeedData>,
    pub animationFinishedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _characterActivationTrack: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Timeline::ActivationTrack,
    >,
    pub _currentBtsCharacter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BTSCharacter,
    >,
    pub _defaultSpawnCharacterDuration: f32,
    pub _playableDirectorTimeBeforePause: f64,
    pub _animatorNormalizedTimeBeforePause: f32,
}
#[cfg(feature = "BTSCharacterSpawnAnimationController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BTSCharacterSpawnAnimationController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BTSCharacterSpawnAnimationController";
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleSpawnCharacterPlayableDirectorStopped(
        &mut self,
        playableDirector: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Playables::PlayableDirector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSpawnCharacterPlayableDirectorStopped", (playableDirector))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PauseAnimation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PauseAnimation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PlayAnimation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayAnimation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ResumeAnimation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResumeAnimation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCharacter(
        &mut self,
        btsCharacter: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BTSCharacter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCharacter", (btsCharacter))?;
        Ok(__cordl_ret.into())
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
    pub fn StopAnimation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopAnimation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WillResumeAnimation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WillResumeAnimation", ())?;
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
    pub fn add_animationFinishedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_animationFinishedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_characterActivationTrack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Timeline::ActivationTrack>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Timeline::ActivationTrack,
        > = __cordl_object.invoke("get_characterActivationTrack", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_duration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_duration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isCharacterVisible(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isCharacterVisible", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_animationFinishedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_animationFinishedEvent", (value))?;
        Ok(__cordl_ret.into())
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
