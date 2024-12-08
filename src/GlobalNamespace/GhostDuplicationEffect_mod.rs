#[cfg(feature = "GhostDuplicationEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct GhostDuplicationEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _canvases: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::CanvasGroup,
    >,
    pub isInitialized: bool,
    pub _tweeningManager: *mut crate::Tweening::TimeTweeningManager,
}
#[cfg(feature = "GhostDuplicationEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GhostDuplicationEffect => ""
    ."GhostDuplicationEffect"
);
#[cfg(feature = "GhostDuplicationEffect")]
impl std::ops::Deref for crate::GlobalNamespace::GhostDuplicationEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GhostDuplicationEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::GhostDuplicationEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GhostDuplicationEffect")]
impl crate::GlobalNamespace::GhostDuplicationEffect {
    #[cfg(feature = "GhostDuplicationEffect+GhostEffectParams")]
    pub type GhostEffectParams = crate::GlobalNamespace::GhostDuplicationEffect_GhostEffectParams;
    #[cfg(feature = "GhostDuplicationEffect+_HideRedundantWithDelay_d__8")]
    pub type _HideRedundantWithDelay_d__8 = crate::GlobalNamespace::GhostDuplicationEffect__HideRedundantWithDelay_d__8;
    #[cfg(feature = "GhostDuplicationEffect+__c__DisplayClass7_0")]
    pub type __c__DisplayClass7_0 = crate::GlobalNamespace::GhostDuplicationEffect___c__DisplayClass7_0;
    pub fn Animate(
        &mut self,
        ghostEffectParams: crate::GlobalNamespace::GhostDuplicationEffect_GhostEffectParams,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Animate", (ghostEffectParams))?;
        Ok(__cordl_ret)
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn HideRedundantWithDelay(
        &mut self,
        delay: f32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("HideRedundantWithDelay", (delay))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
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
    pub fn SetDistances(
        &mut self,
        distance: f32,
        direction: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDistances", (distance, direction))?;
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
    pub fn get_alpha(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_alpha", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_size(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_size", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn set_hide(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hide", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_size(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_size", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "GhostDuplicationEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GhostDuplicationEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GhostDuplicationEffect+GhostEffectParams")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GhostDuplicationEffect_GhostEffectParams {
    pub startAlpha: f32,
    pub startPosition: crate::UnityEngine::Vector3,
    pub startSize: f32,
    pub endAlpha: f32,
    pub endPosition: crate::UnityEngine::Vector3,
    pub endSize: f32,
    pub duration: f32,
    pub delay: f32,
    pub easeType: crate::GlobalNamespace::EaseType,
    pub distanceCurve: *mut crate::UnityEngine::AnimationCurve,
    pub peakDistance: f32,
    pub lastPhase: bool,
}
#[cfg(feature = "GhostDuplicationEffect+GhostEffectParams")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GhostDuplicationEffect_GhostEffectParams => ""
    ."GhostDuplicationEffect/GhostEffectParams"
);
#[cfg(feature = "GhostDuplicationEffect+GhostEffectParams")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::GhostDuplicationEffect_GhostEffectParams {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "GhostDuplicationEffect+GhostEffectParams")]
impl crate::GlobalNamespace::GhostDuplicationEffect_GhostEffectParams {}
