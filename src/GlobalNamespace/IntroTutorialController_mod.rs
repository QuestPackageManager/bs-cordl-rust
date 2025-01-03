#[cfg(feature = "IntroTutorialController")]
#[repr(C)]
#[derive(Debug)]
pub struct IntroTutorialController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _redRing: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IntroTutorialRing>,
    pub _blueRing: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IntroTutorialRing>,
    pub _redRingWrapper: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _blueRingWrapper: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _textCanvasGroup: quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup>,
    pub _shockWavePS: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    pub _finishForParams: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BloomFogEnvironmentParams,
    >,
    pub _bloomFogAnimator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BloomFogParamsAnimator,
    >,
    pub _gamePause: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IGamePause>,
    pub introTutorialDidFinishEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _showingFinishAnimation: bool,
    pub _redRingWrapperActive: bool,
    pub _blueRingWrapperActive: bool,
}
#[cfg(feature = "IntroTutorialController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IntroTutorialController => ""
    ."IntroTutorialController"
);
#[cfg(feature = "IntroTutorialController")]
impl std::ops::Deref for crate::GlobalNamespace::IntroTutorialController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IntroTutorialController")]
impl std::ops::DerefMut for crate::GlobalNamespace::IntroTutorialController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IntroTutorialController")]
impl crate::GlobalNamespace::IntroTutorialController {
    pub fn CleanUp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanUp", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleGameDidPause(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleGameDidPause", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleGameDidResume(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleGameDidResume", ())?;
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
    pub fn SetFinishAnimationParams(
        &mut self,
        progress: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFinishAnimationParams", (progress))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowFinishAnimation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowFinishAnimation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowFinishAnimationCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("ShowFinishAnimationCoroutine", ())?;
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
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
    pub fn add_introTutorialDidFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_introTutorialDidFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_introTutorialDidFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_introTutorialDidFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IntroTutorialController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IntroTutorialController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
