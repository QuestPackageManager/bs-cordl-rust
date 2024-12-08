#[cfg(feature = "FadeInOutController")]
#[repr(C)]
#[derive(Debug)]
pub struct FadeInOutController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _easeValue: *mut FloatSO,
    pub _fadeInCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _fadeOutCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _fadeInStartDelay: f32,
    pub _defaultFadeOutDuration: f32,
    pub _defaultFadeInDuration: f32,
}
#[cfg(feature = "FadeInOutController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for FadeInOutController => ""."FadeInOutController"
);
#[cfg(feature = "FadeInOutController")]
impl std::ops::Deref for FadeInOutController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FadeInOutController")]
impl std::ops::DerefMut for FadeInOutController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FadeInOutController")]
impl FadeInOutController {
    #[cfg(feature = "FadeInOutController+_Fade_d__15")]
    pub type _Fade_d__15 = crate::GlobalNamespace::FadeInOutController__Fade_d__15;
    pub fn Fade(
        &mut self,
        fromValue: f32,
        toValue: f32,
        duration: f32,
        startDelay: f32,
        curve: *mut crate::UnityEngine::AnimationCurve,
        fadeFinishedCallback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke(
                "Fade",
                (fromValue, toValue, duration, startDelay, curve, fadeFinishedCallback),
            )?;
        Ok(__cordl_ret)
    }
    pub fn FadeIn_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FadeIn", ())?;
        Ok(__cordl_ret)
    }
    pub fn FadeIn_Action2(
        &mut self,
        fadeInCallback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FadeIn", (fadeInCallback))?;
        Ok(__cordl_ret)
    }
    pub fn FadeIn_f32_1(
        &mut self,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FadeIn", (duration))?;
        Ok(__cordl_ret)
    }
    pub fn FadeIn_f32_Action3(
        &mut self,
        duration: f32,
        fadeInFinishedCallback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FadeIn", (duration, fadeInFinishedCallback))?;
        Ok(__cordl_ret)
    }
    pub fn FadeOutInstant(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FadeOutInstant", ())?;
        Ok(__cordl_ret)
    }
    pub fn FadeOut_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FadeOut", ())?;
        Ok(__cordl_ret)
    }
    pub fn FadeOut_Action2(
        &mut self,
        fadeOutCallback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FadeOut", (fadeOutCallback))?;
        Ok(__cordl_ret)
    }
    pub fn FadeOut_f32_1(
        &mut self,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FadeOut", (duration))?;
        Ok(__cordl_ret)
    }
    pub fn FadeOut_f32_Action3(
        &mut self,
        duration: f32,
        fadeOutFinishedCallback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FadeOut", (duration, fadeOutFinishedCallback))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "FadeInOutController")]
impl quest_hook::libil2cpp::ObjectType for FadeInOutController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
