#[cfg(feature = "IntroTutorialRing")]
#[repr(C)]
#[derive(Debug)]
pub struct IntroTutorialRing {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _progressImages: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::UI::Image>,
    >,
    pub _saberType: crate::GlobalNamespace::SaberType,
    pub _particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    pub _canvasGroup: quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup>,
    pub _activationDuration: f32,
    pub _ringGLowImages: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::UI::Image>,
    >,
    pub _colorManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorManager>,
    pub _highlighted: bool,
    pub _emitNextParticleTimer: f32,
    pub _activationProgress: f32,
    pub _sabersInside: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<crate::GlobalNamespace::SaberType>,
    >,
    pub _sabersInsideAfterOnEnable: bool,
}
#[cfg(feature = "IntroTutorialRing")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IntroTutorialRing => ""
    ."IntroTutorialRing"
);
#[cfg(feature = "IntroTutorialRing")]
impl std::ops::Deref for crate::GlobalNamespace::IntroTutorialRing {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IntroTutorialRing")]
impl std::ops::DerefMut for crate::GlobalNamespace::IntroTutorialRing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IntroTutorialRing")]
impl crate::GlobalNamespace::IntroTutorialRing {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnTriggerEnter(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnTriggerEnter", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnTriggerExit(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnTriggerExit", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnTriggerStay(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnTriggerStay", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetProgressImagesfillAmount(
        &mut self,
        fillAmount: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetProgressImagesfillAmount", (fillAmount))?;
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
    pub fn get_fullyActivated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_fullyActivated", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_saberType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SaberType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::SaberType = __cordl_object
            .invoke("get_saberType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_alpha(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_alpha", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_saberType(
        &mut self,
        value: crate::GlobalNamespace::SaberType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_saberType", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IntroTutorialRing")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IntroTutorialRing {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
