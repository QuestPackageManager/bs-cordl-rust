#[cfg(feature = "HMUI+SwitchView")]
#[repr(C)]
#[derive(Debug)]
pub struct SwitchView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _animationType: crate::HMUI::SwitchView_AnimationType,
    pub _normalAnimationClip: *mut crate::UnityEngine::AnimationClip,
    pub _highlightedAnimationClip: *mut crate::UnityEngine::AnimationClip,
    pub _pressedAnimationClip: *mut crate::UnityEngine::AnimationClip,
    pub _disabledAnimationClip: *mut crate::UnityEngine::AnimationClip,
    pub _onAnimationClip: *mut crate::UnityEngine::AnimationClip,
    pub _offAnimationClip: *mut crate::UnityEngine::AnimationClip,
    pub _selectedAnimationClip: *mut crate::UnityEngine::AnimationClip,
    pub _selectedAndHighlightedAnimationClip: *mut crate::UnityEngine::AnimationClip,
    pub _toggle: *mut crate::HMUI::ToggleWithCallbacks,
}
#[cfg(feature = "HMUI+SwitchView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::SwitchView => "HMUI"."SwitchView"
);
#[cfg(feature = "HMUI+SwitchView")]
impl std::ops::Deref for crate::HMUI::SwitchView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+SwitchView")]
impl std::ops::DerefMut for crate::HMUI::SwitchView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+SwitchView")]
impl crate::HMUI::SwitchView {
    #[cfg(feature = "HMUI+SwitchView+AnimationType")]
    pub type AnimationType = crate::HMUI::SwitchView_AnimationType;
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
    pub fn HandleOnValueChanged(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleOnValueChanged", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleStateDidChange(
        &mut self,
        value: crate::HMUI::ToggleWithCallbacks_SelectionState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleStateDidChange", (value))?;
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
    pub fn RefreshVisuals(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshVisuals", ())?;
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
#[cfg(feature = "HMUI+SwitchView")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::SwitchView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+SwitchView+AnimationType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchView_AnimationType {
    OnOff = 0i32,
    SelectedState = 1i32,
}
#[cfg(feature = "HMUI+SwitchView+AnimationType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::SwitchView_AnimationType => "HMUI"
    ."SwitchView/AnimationType"
);
