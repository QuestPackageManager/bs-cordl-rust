#[cfg(feature = "HMUI+ViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct ViewController {
    __cordl_parent: crate::HMUI::ViewControllerBase,
    pub _buttonBinder_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::HMUI::ButtonBinder,
    >,
    pub _containerViewController: quest_hook::libil2cpp::Gc<
        crate::HMUI::ContainerViewController,
    >,
    pub _parentViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    pub _childViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    pub _screen: quest_hook::libil2cpp::Gc<crate::HMUI::Screen>,
    pub _rectTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    pub _canvasGroup: quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup>,
    pub _wasActivatedBefore: bool,
    pub _isActivated: bool,
    pub _isInTransition: bool,
    pub _graphicRaycaster: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::BaseRaycaster,
    >,
}
#[cfg(feature = "HMUI+ViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ViewController => "HMUI"."ViewController"
);
#[cfg(feature = "HMUI+ViewController")]
impl std::ops::Deref for crate::HMUI::ViewController {
    type Target = crate::HMUI::ViewControllerBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ViewController")]
impl std::ops::DerefMut for crate::HMUI::ViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ViewController")]
impl crate::HMUI::ViewController {
    pub const kTransitionDuration: f32 = 0.4f32;
    pub const kTransitionMoveOffset: f32 = 2f32;
    #[cfg(feature = "HMUI+ViewController+AnimationDirection")]
    pub type AnimationDirection = crate::HMUI::ViewController_AnimationDirection;
    #[cfg(feature = "HMUI+ViewController+AnimationType")]
    pub type AnimationType = crate::HMUI::ViewController_AnimationType;
    pub fn DeactivateGameObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeactivateGameObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DidActivate",
                (firstActivation, addedToHierarchy, screenSystemEnabling),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DidDeactivate", (removedFromHierarchy, screenSystemDisabling))?;
        Ok(__cordl_ret.into())
    }
    pub fn DismissViewControllerCoroutine(
        &mut self,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        animationDirection: crate::HMUI::ViewController_AnimationDirection,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object
            .invoke(
                "DismissViewControllerCoroutine",
                (finishedCallback, animationDirection, immediately),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsViewControllerInHierarchy(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsViewControllerInHierarchy", (viewController))?;
        Ok(__cordl_ret.into())
    }
    pub fn Log(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Log", (message))?;
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
    pub fn PresentViewControllerCoroutine(
        &mut self,
        newViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        animationDirection: crate::HMUI::ViewController_AnimationDirection,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object
            .invoke(
                "PresentViewControllerCoroutine",
                (newViewController, finishedCallback, animationDirection, immediately),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReplaceViewControllerCoroutine(
        &mut self,
        newViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        animationType: crate::HMUI::ViewController_AnimationType,
        animationDirection: crate::HMUI::ViewController_AnimationDirection,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object
            .invoke(
                "ReplaceViewControllerCoroutine",
                (newViewController, finishedCallback, animationType, animationDirection),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn __Activate(
        &mut self,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("__Activate", (addedToHierarchy, screenSystemEnabling))?;
        Ok(__cordl_ret.into())
    }
    pub fn __Deactivate(
        &mut self,
        removedFromHierarchy: bool,
        deactivateGameObject: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "__Deactivate",
                (removedFromHierarchy, deactivateGameObject, screenSystemDisabling),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn __DismissViewController(
        &mut self,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        animationDirection: crate::HMUI::ViewController_AnimationDirection,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "__DismissViewController",
                (finishedCallback, animationDirection, immediately),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn __Init(
        &mut self,
        screen: quest_hook::libil2cpp::Gc<crate::HMUI::Screen>,
        parentViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        containerViewController: quest_hook::libil2cpp::Gc<
            crate::HMUI::ContainerViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("__Init", (screen, parentViewController, containerViewController))?;
        Ok(__cordl_ret.into())
    }
    pub fn __PresentViewController(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        animationDirection: crate::HMUI::ViewController_AnimationDirection,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "__PresentViewController",
                (viewController, finishedCallback, animationDirection, immediately),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn __ReplaceViewController(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        animationType: crate::HMUI::ViewController_AnimationType,
        animationDirection: crate::HMUI::ViewController_AnimationDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "__ReplaceViewController",
                (viewController, finishedCallback, animationType, animationDirection),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn __ResetViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("__ResetViewController", ())?;
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
    pub fn get_buttonBinder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::ButtonBinder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::ButtonBinder> = __cordl_object
            .invoke("get_buttonBinder", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_canvasGroup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup> = __cordl_object
            .invoke("get_canvasGroup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_childViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController> = __cordl_object
            .invoke("get_childViewController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_containerViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::ContainerViewController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HMUI::ContainerViewController,
        > = __cordl_object.invoke("get_containerViewController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enableUserInteractions(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enableUserInteractions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_graphicRaycaster(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::BaseRaycaster>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseRaycaster,
        > = __cordl_object.invoke("get_graphicRaycaster", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isActivated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isActivated", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isInTransition(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isInTransition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isInViewControllerHierarchy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_isInViewControllerHierarchy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_parentViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController> = __cordl_object
            .invoke("get_parentViewController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rectTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform> = __cordl_object
            .invoke("get_rectTransform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_screen(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::HMUI::Screen>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::Screen> = __cordl_object
            .invoke("get_screen", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wasActivatedBefore(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_wasActivatedBefore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_buttonBinder(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::HMUI::ButtonBinder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_buttonBinder", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_enableUserInteractions(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_enableUserInteractions", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isInTransition(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isInTransition", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+ViewController")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::ViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+ViewController+AnimationDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewController_AnimationDirection {
    Horizontal = 0i32,
    Vertical = 1i32,
}
#[cfg(feature = "HMUI+ViewController+AnimationDirection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ViewController_AnimationDirection =>
    "HMUI"."ViewController/AnimationDirection"
);
#[cfg(feature = "HMUI+ViewController+AnimationType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewController_AnimationType {
    In = 1i32,
    None = 0i32,
    Out = 2i32,
}
#[cfg(feature = "HMUI+ViewController+AnimationType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ViewController_AnimationType => "HMUI"
    ."ViewController/AnimationType"
);
