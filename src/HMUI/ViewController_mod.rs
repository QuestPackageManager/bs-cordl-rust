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
#[cfg(feature = "HMUI+ViewController+DidActivateDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct ViewController_DidActivateDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "HMUI+ViewController+DidActivateDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ViewController_DidActivateDelegate =>
    "HMUI"."ViewController/DidActivateDelegate"
);
#[cfg(feature = "HMUI+ViewController+DidActivateDelegate")]
impl std::ops::Deref for crate::HMUI::ViewController_DidActivateDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ViewController+DidActivateDelegate")]
impl std::ops::DerefMut for crate::HMUI::ViewController_DidActivateDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ViewController+DidActivateDelegate")]
impl crate::HMUI::ViewController_DidActivateDelegate {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn BeginInvoke(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (
                    firstActivation,
                    addedToHierarchy,
                    screenSystemEnabling,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
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
                "Invoke",
                (firstActivation, addedToHierarchy, screenSystemEnabling),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "HMUI+ViewController+DidActivateDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::HMUI::ViewController_DidActivateDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+ViewController+DidDeactivateDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct ViewController_DidDeactivateDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "HMUI+ViewController+DidDeactivateDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ViewController_DidDeactivateDelegate =>
    "HMUI"."ViewController/DidDeactivateDelegate"
);
#[cfg(feature = "HMUI+ViewController+DidDeactivateDelegate")]
impl std::ops::Deref for crate::HMUI::ViewController_DidDeactivateDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ViewController+DidDeactivateDelegate")]
impl std::ops::DerefMut for crate::HMUI::ViewController_DidDeactivateDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ViewController+DidDeactivateDelegate")]
impl crate::HMUI::ViewController_DidDeactivateDelegate {
    pub fn BeginInvoke(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (removedFromHierarchy, screenSystemDisabling, callback, object),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (removedFromHierarchy, screenSystemDisabling))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "HMUI+ViewController+DidDeactivateDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::HMUI::ViewController_DidDeactivateDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+ViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct ViewController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub didActivateEvent: *mut crate::HMUI::ViewController_DidActivateDelegate,
    pub didDeactivateEvent: *mut crate::HMUI::ViewController_DidDeactivateDelegate,
    pub _buttonBinder_k__BackingField: *mut crate::HMUI::ButtonBinder,
    pub _containerViewController: *mut crate::HMUI::ContainerViewController,
    pub _parentViewController: *mut crate::HMUI::ViewController,
    pub _childViewController: *mut crate::HMUI::ViewController,
    pub _screen: *mut crate::HMUI::Screen,
    pub _rectTransform: *mut crate::UnityEngine::RectTransform,
    pub _canvasGroup: *mut crate::UnityEngine::CanvasGroup,
    pub _wasActivatedBefore: bool,
    pub _isActivated: bool,
    pub _isInTransition: bool,
    pub _graphicRaycaster: *mut crate::UnityEngine::EventSystems::BaseRaycaster,
}
#[cfg(feature = "HMUI+ViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ViewController => "HMUI"."ViewController"
);
#[cfg(feature = "HMUI+ViewController")]
impl std::ops::Deref for crate::HMUI::ViewController {
    type Target = crate::UnityEngine::MonoBehaviour;
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
    #[cfg(feature = "HMUI+ViewController+_ReplaceViewControllerCoroutine_d__60")]
    pub type _ReplaceViewControllerCoroutine_d__60 = crate::HMUI::ViewController__ReplaceViewControllerCoroutine_d__60;
    #[cfg(feature = "HMUI+ViewController+DidDeactivateDelegate")]
    pub type DidDeactivateDelegate = crate::HMUI::ViewController_DidDeactivateDelegate;
    #[cfg(feature = "HMUI+ViewController+AnimationDirection")]
    pub type AnimationDirection = crate::HMUI::ViewController_AnimationDirection;
    #[cfg(feature = "HMUI+ViewController+_DismissViewControllerCoroutine_d__62")]
    pub type _DismissViewControllerCoroutine_d__62 = crate::HMUI::ViewController__DismissViewControllerCoroutine_d__62;
    #[cfg(feature = "HMUI+ViewController+AnimationType")]
    pub type AnimationType = crate::HMUI::ViewController_AnimationType;
    #[cfg(feature = "HMUI+ViewController+DidActivateDelegate")]
    pub type DidActivateDelegate = crate::HMUI::ViewController_DidActivateDelegate;
    #[cfg(feature = "HMUI+ViewController+_PresentViewControllerCoroutine_d__58")]
    pub type _PresentViewControllerCoroutine_d__58 = crate::HMUI::ViewController__PresentViewControllerCoroutine_d__58;
    pub fn get_graphicRaycaster(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::EventSystems::BaseRaycaster,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::EventSystems::BaseRaycaster = __cordl_object
            .invoke("get_graphicRaycaster", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn add_didActivateEvent(
        &mut self,
        value: *mut crate::HMUI::ViewController_DidActivateDelegate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didActivateEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn __DismissViewController(
        &mut self,
        finishedCallback: *mut crate::System::Action,
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
        Ok(__cordl_ret)
    }
    pub fn set_buttonBinder(
        &mut self,
        value: *mut crate::HMUI::ButtonBinder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_buttonBinder", (value))?;
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
    pub fn remove_didActivateEvent(
        &mut self,
        value: *mut crate::HMUI::ViewController_DidActivateDelegate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didActivateEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn __PresentViewController(
        &mut self,
        viewController: *mut crate::HMUI::ViewController,
        finishedCallback: *mut crate::System::Action,
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
        Ok(__cordl_ret)
    }
    pub fn PresentViewControllerCoroutine(
        &mut self,
        newViewController: *mut crate::HMUI::ViewController,
        finishedCallback: *mut crate::System::Action,
        animationDirection: crate::HMUI::ViewController_AnimationDirection,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke(
                "PresentViewControllerCoroutine",
                (newViewController, finishedCallback, animationDirection, immediately),
            )?;
        Ok(__cordl_ret)
    }
    pub fn remove_didDeactivateEvent(
        &mut self,
        value: *mut crate::HMUI::ViewController_DidDeactivateDelegate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didDeactivateEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_isActivated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isActivated", ())?;
        Ok(__cordl_ret)
    }
    pub fn __ResetViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("__ResetViewController", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_childViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::ViewController> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::ViewController = __cordl_object
            .invoke("get_childViewController", ())?;
        Ok(__cordl_ret)
    }
    pub fn __ReplaceViewController(
        &mut self,
        viewController: *mut crate::HMUI::ViewController,
        finishedCallback: *mut crate::System::Action,
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
        Ok(__cordl_ret)
    }
    pub fn add_didDeactivateEvent(
        &mut self,
        value: *mut crate::HMUI::ViewController_DidDeactivateDelegate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didDeactivateEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn DeactivateGameObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeactivateGameObject", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_canvasGroup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::CanvasGroup> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::CanvasGroup = __cordl_object
            .invoke("get_canvasGroup", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_isInViewControllerHierarchy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_isInViewControllerHierarchy", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isInTransition(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isInTransition", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReplaceViewControllerCoroutine(
        &mut self,
        newViewController: *mut crate::HMUI::ViewController,
        finishedCallback: *mut crate::System::Action,
        animationType: crate::HMUI::ViewController_AnimationType,
        animationDirection: crate::HMUI::ViewController_AnimationDirection,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke(
                "ReplaceViewControllerCoroutine",
                (newViewController, finishedCallback, animationType, animationDirection),
            )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn DismissViewControllerCoroutine(
        &mut self,
        finishedCallback: *mut crate::System::Action,
        animationDirection: crate::HMUI::ViewController_AnimationDirection,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke(
                "DismissViewControllerCoroutine",
                (finishedCallback, animationDirection, immediately),
            )?;
        Ok(__cordl_ret)
    }
    pub fn IsViewControllerInHierarchy(
        &mut self,
        viewController: *mut crate::HMUI::ViewController,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsViewControllerInHierarchy", (viewController))?;
        Ok(__cordl_ret)
    }
    pub fn get_containerViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::ContainerViewController> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::ContainerViewController = __cordl_object
            .invoke("get_containerViewController", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rectTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::RectTransform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::RectTransform = __cordl_object
            .invoke("get_rectTransform", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_screen(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::Screen> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::Screen = __cordl_object
            .invoke("get_screen", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_wasActivatedBefore(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_wasActivatedBefore", ())?;
        Ok(__cordl_ret)
    }
    pub fn __Init(
        &mut self,
        screen: *mut crate::HMUI::Screen,
        parentViewController: *mut crate::HMUI::ViewController,
        containerViewController: *mut crate::HMUI::ContainerViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("__Init", (screen, parentViewController, containerViewController))?;
        Ok(__cordl_ret)
    }
    pub fn get_buttonBinder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::ButtonBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::ButtonBinder = __cordl_object
            .invoke("get_buttonBinder", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_enableUserInteractions(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enableUserInteractions", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_parentViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::ViewController> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::ViewController = __cordl_object
            .invoke("get_parentViewController", ())?;
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
#[cfg(feature = "HMUI+ViewController")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::ViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
