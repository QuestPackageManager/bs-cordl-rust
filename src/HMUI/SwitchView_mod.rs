#[cfg(feature = "HMUI+SwitchView")]
#[repr(C)]
#[derive(Debug)]
pub struct SwitchView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _animationType: crate::HMUI::SwitchView_AnimationType,
    pub _normalAnimationClip: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AnimationClip,
    >,
    pub _highlightedAnimationClip: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AnimationClip,
    >,
    pub _pressedAnimationClip: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AnimationClip,
    >,
    pub _disabledAnimationClip: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AnimationClip,
    >,
    pub _onAnimationClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
    pub _offAnimationClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationClip>,
    pub _selectedAnimationClip: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AnimationClip,
    >,
    pub _selectedAndHighlightedAnimationClip: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AnimationClip,
    >,
    pub _toggle: quest_hook::libil2cpp::Gc<crate::HMUI::ToggleWithCallbacks>,
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
    pub fn RefreshVisuals(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshVisuals", ())?;
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
