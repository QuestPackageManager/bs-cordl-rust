#[cfg(feature = "HMUI+ModalView")]
#[repr(C)]
#[derive(Debug)]
pub struct ModalView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _presentPanelAnimations: *mut crate::HMUI::PanelAnimationSO,
    pub _dismissPanelAnimation: *mut crate::HMUI::PanelAnimationSO,
    pub _animateParentCanvas: bool,
    pub _container: *mut crate::Zenject::DiContainer,
    pub blockerClickedEvent: *mut crate::System::Action,
    pub _previousParent: *mut crate::UnityEngine::Transform,
    pub _isShown: bool,
    pub _viewIsValid: bool,
    pub _mainCanvas: *mut crate::UnityEngine::Canvas,
    pub _parentCanvasGroup: *mut crate::UnityEngine::CanvasGroup,
    pub _blockerGO: *mut crate::UnityEngine::GameObject,
    pub _test: i32,
}
#[cfg(feature = "HMUI+ModalView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ModalView => "HMUI"."ModalView"
);
#[cfg(feature = "HMUI+ModalView")]
impl std::ops::Deref for crate::HMUI::ModalView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ModalView")]
impl std::ops::DerefMut for crate::HMUI::ModalView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ModalView")]
impl crate::HMUI::ModalView {
    #[cfg(feature = "HMUI+ModalView+__c__DisplayClass17_0")]
    pub type __c__DisplayClass17_0 = crate::HMUI::ModalView___c__DisplayClass17_0;
    #[cfg(feature = "HMUI+ModalView+__c__DisplayClass21_0")]
    pub type __c__DisplayClass21_0 = crate::HMUI::ModalView___c__DisplayClass21_0;
    pub fn Hide(
        &mut self,
        animated: bool,
        finishedCallback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Hide", (animated, finishedCallback))?;
        Ok(__cordl_ret)
    }
    pub fn SetupView(
        &mut self,
        screenTransform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupView", (screenTransform))?;
        Ok(__cordl_ret)
    }
    pub fn CreateBlocker(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("CreateBlocker", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleParentViewControllerDidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleParentViewControllerDidDeactivate",
                (removedFromHierarchy, screenSystemDisabling),
            )?;
        Ok(__cordl_ret)
    }
    pub fn remove_blockerClickedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_blockerClickedEvent", (value))?;
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
    pub fn Show(
        &mut self,
        animated: bool,
        moveToCenter: bool,
        finishedCallback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Show", (animated, moveToCenter, finishedCallback))?;
        Ok(__cordl_ret)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_blockerClickedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_blockerClickedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleBlockerButtonClicked(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBlockerButtonClicked", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "HMUI+ModalView")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::ModalView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
