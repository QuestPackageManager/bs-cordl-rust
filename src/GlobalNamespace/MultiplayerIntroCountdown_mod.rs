#[cfg(feature = "MultiplayerIntroCountdown")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerIntroCountdown {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _textAppearDuration: f32,
    pub _textDisappearDuration: f32,
    pub _goDisappearDuration: f32,
    pub _partsDistance: f32,
    pub _startLocalPosition: crate::UnityEngine::Vector3,
    pub _targetLocalPosition: crate::UnityEngine::Vector3,
    pub _readyClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    pub _setClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    pub _goClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    pub _buildUpClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    pub _textController0: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerIntroCountdownTextController,
    >,
    pub _textController1: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerIntroCountdownTextController,
    >,
    pub _audioSource: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioSource>,
    pub _multiplayerOffsetByLocalPlayerPosition: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerOffsetPositionByLocalPlayerPosition,
    >,
    pub _tweeningManager: quest_hook::libil2cpp::Gc<
        crate::Tweening::TimeTweeningManager,
    >,
    pub _currentTextController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerIntroCountdownTextController,
    >,
    pub _fontSize: f32,
    pub _alpha: f32,
}
#[cfg(feature = "MultiplayerIntroCountdown")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerIntroCountdown {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerIntroCountdown";
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
#[cfg(feature = "MultiplayerIntroCountdown")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerIntroCountdown {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerIntroCountdown")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerIntroCountdown {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerIntroCountdown")]
impl crate::GlobalNamespace::MultiplayerIntroCountdown {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CountdownRoutine(
        &mut self,
        seconds: f32,
        delay: f32,
        durationMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object
            .invoke("CountdownRoutine", (seconds, delay, durationMultiplier))?;
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
    pub fn PhaseRoutine(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        appearDuration: f32,
        disappearDuration: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object
            .invoke("PhaseRoutine", (text, appearDuration, disappearDuration))?;
        Ok(__cordl_ret.into())
    }
    pub fn PlayDelayed(
        &mut self,
        audioClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        delay: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("PlayDelayed", (audioClip, delay))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartCountdown(
        &mut self,
        seconds: f32,
        delay: f32,
        durationMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartCountdown", (seconds, delay, durationMultiplier))?;
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
    pub fn get_textAppearDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_textAppearDuration", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerIntroCountdown")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerIntroCountdown {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
