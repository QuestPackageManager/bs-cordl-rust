#[cfg(feature = "HMUI+FlowCoordinator")]
#[repr(C)]
#[derive(Debug)]
pub struct FlowCoordinator {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _baseInputModule: *mut crate::UnityEngine::EventSystems::BaseInputModule,
    pub _screenSystem: *mut crate::HMUI::ScreenSystem,
    pub _parentFlowCoordinator: *mut crate::HMUI::FlowCoordinator,
    pub _childFlowCoordinator: *mut crate::HMUI::FlowCoordinator,
    pub _mainScreenViewControllers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HMUI::ViewController,
    >,
    pub _leftScreenViewController: *mut crate::HMUI::ViewController,
    pub _rightScreenViewController: *mut crate::HMUI::ViewController,
    pub _bottomScreenViewController: *mut crate::HMUI::ViewController,
    pub _topScreenViewController: *mut crate::HMUI::ViewController,
    pub _wasActivatedBefore: bool,
    pub _title: *mut crate::System::String,
    pub _providedMainViewController: *mut crate::HMUI::ViewController,
    pub _providedLeftScreenViewController: *mut crate::HMUI::ViewController,
    pub _providedRightScreenViewController: *mut crate::HMUI::ViewController,
    pub _providedBottomScreenViewController: *mut crate::HMUI::ViewController,
    pub _providedTopScreenViewController: *mut crate::HMUI::ViewController,
    pub _viewControllersWereProvided: bool,
    pub _isInDidActivatePhase: bool,
    pub _isActivated: bool,
    pub _isInTransition: bool,
    pub _showBackButton: bool,
    pub _prevEventSystem: *mut crate::UnityEngine::EventSystems::EventSystem,
}
#[cfg(feature = "HMUI+FlowCoordinator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::FlowCoordinator => "HMUI"
    ."FlowCoordinator"
);
#[cfg(feature = "HMUI+FlowCoordinator")]
impl std::ops::Deref for crate::HMUI::FlowCoordinator {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+FlowCoordinator")]
impl std::ops::DerefMut for crate::HMUI::FlowCoordinator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+FlowCoordinator")]
impl crate::HMUI::FlowCoordinator {
    #[cfg(feature = "HMUI+FlowCoordinator+__c__DisplayClass39_0")]
    pub type __c__DisplayClass39_0 = crate::HMUI::FlowCoordinator___c__DisplayClass39_0;
    #[cfg(feature = "HMUI+FlowCoordinator+__c__DisplayClass48_0")]
    pub type __c__DisplayClass48_0 = crate::HMUI::FlowCoordinator___c__DisplayClass48_0;
    #[cfg(
        feature = "HMUI+FlowCoordinator+_DismissChildFlowCoordinatorsRecursively_d__41"
    )]
    pub type _DismissChildFlowCoordinatorsRecursively_d__41 = crate::HMUI::FlowCoordinator__DismissChildFlowCoordinatorsRecursively_d__41;
    #[cfg(
        feature = "HMUI+FlowCoordinator+_DismissChildViewControllersRecursively_d__42"
    )]
    pub type _DismissChildViewControllersRecursively_d__42 = crate::HMUI::FlowCoordinator__DismissChildViewControllersRecursively_d__42;
    #[cfg(feature = "HMUI+FlowCoordinator+__c__DisplayClass43_0")]
    pub type __c__DisplayClass43_0 = crate::HMUI::FlowCoordinator___c__DisplayClass43_0;
    #[cfg(feature = "HMUI+FlowCoordinator+__c__DisplayClass47_0")]
    pub type __c__DisplayClass47_0 = crate::HMUI::FlowCoordinator___c__DisplayClass47_0;
    #[cfg(feature = "HMUI+FlowCoordinator+__c__DisplayClass44_0")]
    pub type __c__DisplayClass44_0 = crate::HMUI::FlowCoordinator___c__DisplayClass44_0;
    #[cfg(feature = "HMUI+FlowCoordinator+__c__DisplayClass51_0")]
    pub type __c__DisplayClass51_0 = crate::HMUI::FlowCoordinator___c__DisplayClass51_0;
    #[cfg(feature = "HMUI+FlowCoordinator+__c__DisplayClass52_0")]
    pub type __c__DisplayClass52_0 = crate::HMUI::FlowCoordinator___c__DisplayClass52_0;
    #[cfg(feature = "HMUI+FlowCoordinator+__c__DisplayClass45_0")]
    pub type __c__DisplayClass45_0 = crate::HMUI::FlowCoordinator___c__DisplayClass45_0;
    #[cfg(feature = "HMUI+FlowCoordinator+__c__DisplayClass40_0")]
    pub type __c__DisplayClass40_0 = crate::HMUI::FlowCoordinator___c__DisplayClass40_0;
    pub fn YoungestChildFlowCoordinatorOrSelf(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::FlowCoordinator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::FlowCoordinator = __cordl_object
            .invoke("YoungestChildFlowCoordinatorOrSelf", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetBottomScreenViewController(
        &mut self,
        viewController: *mut crate::HMUI::ViewController,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBottomScreenViewController", (viewController, animationType))?;
        Ok(__cordl_ret)
    }
    pub fn SetRightScreenViewController(
        &mut self,
        viewController: *mut crate::HMUI::ViewController,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRightScreenViewController", (viewController, animationType))?;
        Ok(__cordl_ret)
    }
    pub fn SetGlobalUserInteraction(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGlobalUserInteraction", (value))?;
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
    pub fn PresentTitle(
        &mut self,
        title: *mut crate::System::String,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PresentTitle", (title, animationType))?;
        Ok(__cordl_ret)
    }
    pub fn DismissFlowCoordinator(
        &mut self,
        flowCoordinator: *mut crate::HMUI::FlowCoordinator,
        animationDirection: crate::HMUI::ViewController_AnimationDirection,
        finishedCallback: *mut crate::System::Action,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DismissFlowCoordinator",
                (flowCoordinator, animationDirection, finishedCallback, immediately),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ReplaceTopViewController_Action_ViewController_AnimationType_ViewController_AnimationDirection0(
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
                "ReplaceTopViewController",
                (viewController, finishedCallback, animationType, animationDirection),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ReplaceTopViewController_FlowCoordinator_FlowCoordinator_Action_ViewController_AnimationType_ViewController_AnimationDirection1(
        &mut self,
        viewController: *mut crate::HMUI::ViewController,
        originalOwnerFlowCoordinator: *mut crate::HMUI::FlowCoordinator,
        newOwnerFlowCoordinator: *mut crate::HMUI::FlowCoordinator,
        finishedCallback: *mut crate::System::Action,
        animationType: crate::HMUI::ViewController_AnimationType,
        animationDirection: crate::HMUI::ViewController_AnimationDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ReplaceTopViewController",
                (
                    viewController,
                    originalOwnerFlowCoordinator,
                    newOwnerFlowCoordinator,
                    finishedCallback,
                    animationType,
                    animationDirection,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn set_showBackButton(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_showBackButton", (value))?;
        Ok(__cordl_ret)
    }
    pub fn PopViewControllersFromNavigationController(
        &mut self,
        navigationController: *mut crate::HMUI::NavigationController,
        numberOfControllers: i32,
        finishedCallback: *mut crate::System::Action,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "PopViewControllersFromNavigationController",
                (
                    navigationController,
                    numberOfControllers,
                    finishedCallback,
                    immediately,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn __ExternalActivate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("__ExternalActivate", ())?;
        Ok(__cordl_ret)
    }
    pub fn TopViewControllerWillChange(
        &mut self,
        oldViewController: *mut crate::HMUI::ViewController,
        newViewController: *mut crate::HMUI::ViewController,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "TopViewControllerWillChange",
                (oldViewController, newViewController, animationType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_topViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::ViewController> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::ViewController = __cordl_object
            .invoke("get_topViewController", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetViewControllersToNavigationController(
        &mut self,
        navigationController: *mut crate::HMUI::NavigationController,
        viewControllers: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::HMUI::ViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetViewControllersToNavigationController",
                (navigationController, viewControllers),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_isInTransition(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isInTransition", ())?;
        Ok(__cordl_ret)
    }
    pub fn DismissViewController(
        &mut self,
        viewController: *mut crate::HMUI::ViewController,
        animationDirection: crate::HMUI::ViewController_AnimationDirection,
        finishedCallback: *mut crate::System::Action,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke(
                "DismissViewController",
                (viewController, animationDirection, finishedCallback, immediately),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetTopScreenViewController(
        &mut self,
        viewController: *mut crate::HMUI::ViewController,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTopScreenViewController", (viewController, animationType))?;
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
    pub fn PopViewControllerFromNavigationController(
        &mut self,
        navigationController: *mut crate::HMUI::NavigationController,
        finishedCallback: *mut crate::System::Action,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "PopViewControllerFromNavigationController",
                (navigationController, finishedCallback, immediately),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_rightScreenViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::ViewController> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::ViewController = __cordl_object
            .invoke("get_rightScreenViewController", ())?;
        Ok(__cordl_ret)
    }
    pub fn __StartOnScreenSystem(
        &mut self,
        screenSystem: *mut crate::HMUI::ScreenSystem,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("__StartOnScreenSystem", (screenSystem))?;
        Ok(__cordl_ret)
    }
    pub fn SetViewControllerToNavigationController(
        &mut self,
        navigationController: *mut crate::HMUI::NavigationController,
        viewController: *mut crate::HMUI::ViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetViewControllerToNavigationController",
                (navigationController, viewController),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_isActivated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isActivated", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetLeftScreenViewController(
        &mut self,
        viewController: *mut crate::HMUI::ViewController,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLeftScreenViewController", (viewController, animationType))?;
        Ok(__cordl_ret)
    }
    pub fn PresentFlowCoordinator(
        &mut self,
        flowCoordinator: *mut crate::HMUI::FlowCoordinator,
        finishedCallback: *mut crate::System::Action,
        animationDirection: crate::HMUI::ViewController_AnimationDirection,
        immediately: bool,
        replaceTopViewController: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "PresentFlowCoordinator",
                (
                    flowCoordinator,
                    finishedCallback,
                    animationDirection,
                    immediately,
                    replaceTopViewController,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn BackButtonWasPressed(
        &mut self,
        topViewController: *mut crate::HMUI::ViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BackButtonWasPressed", (topViewController))?;
        Ok(__cordl_ret)
    }
    pub fn get_showBackButton(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_showBackButton", ())?;
        Ok(__cordl_ret)
    }
    pub fn DismissChildFlowCoordinatorsRecursively(
        &mut self,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("DismissChildFlowCoordinatorsRecursively", (immediately))?;
        Ok(__cordl_ret)
    }
    pub fn ReplaceChildFlowCoordinator(
        &mut self,
        flowCoordinator: *mut crate::HMUI::FlowCoordinator,
        finishedCallback: *mut crate::System::Action,
        animationDirection: crate::HMUI::ViewController_AnimationDirection,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ReplaceChildFlowCoordinator",
                (flowCoordinator, finishedCallback, animationDirection, immediately),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_title(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_title", ())?;
        Ok(__cordl_ret)
    }
    pub fn PushViewControllerToNavigationController(
        &mut self,
        navigationController: *mut crate::HMUI::NavigationController,
        viewController: *mut crate::HMUI::ViewController,
        finishedCallback: *mut crate::System::Action,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "PushViewControllerToNavigationController",
                (navigationController, viewController, finishedCallback, immediately),
            )?;
        Ok(__cordl_ret)
    }
    pub fn IsFlowCoordinatorInHierarchy(
        &mut self,
        flowCoordinator: *mut crate::HMUI::FlowCoordinator,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsFlowCoordinatorInHierarchy", (flowCoordinator))?;
        Ok(__cordl_ret)
    }
    pub fn Activate(
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
                "Activate",
                (firstActivation, addedToHierarchy, screenSystemEnabling),
            )?;
        Ok(__cordl_ret)
    }
    pub fn __ExternalDeactivate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("__ExternalDeactivate", ())?;
        Ok(__cordl_ret)
    }
    pub fn TransitionDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TransitionDidFinish", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_childFlowCoordinator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::FlowCoordinator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::FlowCoordinator = __cordl_object
            .invoke("get_childFlowCoordinator", ())?;
        Ok(__cordl_ret)
    }
    pub fn Deactivate(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deactivate", (removedFromHierarchy, screenSystemDisabling))?;
        Ok(__cordl_ret)
    }
    pub fn HandleScreenSystemBackButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleScreenSystemBackButtonWasPressed", ())?;
        Ok(__cordl_ret)
    }
    pub fn PresentViewController(
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
                "PresentViewController",
                (viewController, finishedCallback, animationDirection, immediately),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InitialViewControllerWasPresented(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitialViewControllerWasPresented", ())?;
        Ok(__cordl_ret)
    }
    pub fn DismissChildViewControllersRecursively(
        &mut self,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("DismissChildViewControllersRecursively", (immediately))?;
        Ok(__cordl_ret)
    }
    pub fn SetTitle(
        &mut self,
        value: *mut crate::System::String,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTitle", (value, animationType))?;
        Ok(__cordl_ret)
    }
    pub fn ProvideInitialViewControllers(
        &mut self,
        mainViewController: *mut crate::HMUI::ViewController,
        leftScreenViewController: *mut crate::HMUI::ViewController,
        rightScreenViewController: *mut crate::HMUI::ViewController,
        bottomScreenViewController: *mut crate::HMUI::ViewController,
        topScreenViewController: *mut crate::HMUI::ViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ProvideInitialViewControllers",
                (
                    mainViewController,
                    leftScreenViewController,
                    rightScreenViewController,
                    bottomScreenViewController,
                    topScreenViewController,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn TransitionDidStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TransitionDidStart", ())?;
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
#[cfg(feature = "HMUI+FlowCoordinator")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::FlowCoordinator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
