#[cfg(feature = "HMUI+ModalView")]
#[repr(C)]
#[derive(Debug)]
pub struct ModalView {
    __cordl_parent: crate::HMUI::ModalViewBase,
    pub _presentPanelAnimations: quest_hook::libil2cpp::Gc<
        crate::HMUI::PanelAnimationSO,
    >,
    pub _dismissPanelAnimation: quest_hook::libil2cpp::Gc<crate::HMUI::PanelAnimationSO>,
    pub _animateParentCanvas: bool,
    pub _container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    pub blockerClickedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _previousParent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _isShown: bool,
    pub _viewIsValid: bool,
    pub _mainCanvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
    pub _parentCanvasGroup: quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup>,
    pub _blockerGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _test: i32,
}
#[cfg(feature = "HMUI+ModalView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ModalView => "HMUI"."ModalView"
);
#[cfg(feature = "HMUI+ModalView")]
impl std::ops::Deref for crate::HMUI::ModalView {
    type Target = crate::HMUI::ModalViewBase;
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
    pub fn CreateBlocker(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("CreateBlocker", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetModalRootTransform(
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        canvas: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
        >,
        viewController: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::HMUI::ViewControllerBase>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetModalRootTransform", (transform, canvas, viewController))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleBlockerButtonClicked(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBlockerButtonClicked", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn Hide(
        &mut self,
        animated: bool,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Hide", (animated, finishedCallback))?;
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
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetupView(
        &mut self,
        screenTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupView", (screenTransform))?;
        Ok(__cordl_ret.into())
    }
    pub fn Show(
        &mut self,
        animated: bool,
        moveToCenter: bool,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Show", (animated, moveToCenter, finishedCallback))?;
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
    pub fn add_blockerClickedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_blockerClickedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isShown(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isShown", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_blockerClickedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_blockerClickedEvent", (value))?;
        Ok(__cordl_ret.into())
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
