#[cfg(feature = "HMUI+FlowCoordinator")]
#[repr(C)]
#[derive(Debug)]
pub struct FlowCoordinator {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    pub _baseInputModule: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::BaseInputModule,
    >,
    pub _screenSystem: quest_hook::libil2cpp::Gc<crate::HMUI::ScreenSystem>,
    pub _parentFlowCoordinator: quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
    pub _childFlowCoordinator: quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
    pub _mainScreenViewControllers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    >,
    pub _leftScreenViewController: quest_hook::libil2cpp::Gc<
        crate::HMUI::ViewController,
    >,
    pub _rightScreenViewController: quest_hook::libil2cpp::Gc<
        crate::HMUI::ViewController,
    >,
    pub _bottomScreenViewController: quest_hook::libil2cpp::Gc<
        crate::HMUI::ViewController,
    >,
    pub _topScreenViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    pub _wasActivatedBefore: bool,
    pub _title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _providedMainViewController: quest_hook::libil2cpp::Gc<
        crate::HMUI::ViewController,
    >,
    pub _providedLeftScreenViewController: quest_hook::libil2cpp::Gc<
        crate::HMUI::ViewController,
    >,
    pub _providedRightScreenViewController: quest_hook::libil2cpp::Gc<
        crate::HMUI::ViewController,
    >,
    pub _providedBottomScreenViewController: quest_hook::libil2cpp::Gc<
        crate::HMUI::ViewController,
    >,
    pub _providedTopScreenViewController: quest_hook::libil2cpp::Gc<
        crate::HMUI::ViewController,
    >,
    pub _viewControllersWereProvided: bool,
    pub _isInDidActivatePhase: bool,
    pub _isActivated: bool,
    pub _isInTransition: bool,
    pub _showBackButton: bool,
    pub _prevEventSystem: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::EventSystem,
    >,
}
#[cfg(feature = "HMUI+FlowCoordinator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::FlowCoordinator => "HMUI"
    ."FlowCoordinator"
);
#[cfg(feature = "HMUI+FlowCoordinator")]
impl std::ops::Deref for crate::HMUI::FlowCoordinator {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>;
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
        Ok(__cordl_ret.into())
    }
    pub fn BackButtonWasPressed(
        &mut self,
        topViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BackButtonWasPressed", (topViewController))?;
        Ok(__cordl_ret.into())
    }
    pub fn CanPressBackButton(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanPressBackButton", ())?;
        Ok(__cordl_ret.into())
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
    pub fn DismissChildFlowCoordinatorsRecursively(
        &mut self,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object
            .invoke("DismissChildFlowCoordinatorsRecursively", (immediately))?;
        Ok(__cordl_ret.into())
    }
    pub fn DismissChildViewControllersRecursively(
        &mut self,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object
            .invoke("DismissChildViewControllersRecursively", (immediately))?;
        Ok(__cordl_ret.into())
    }
    pub fn DismissFlowCoordinator(
        &mut self,
        flowCoordinator: quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
        animationDirection: crate::HMUI::ViewController_AnimationDirection,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
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
        Ok(__cordl_ret.into())
    }
    pub fn DismissViewController(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        animationDirection: crate::HMUI::ViewController_AnimationDirection,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object
            .invoke(
                "DismissViewController",
                (viewController, animationDirection, finishedCallback, immediately),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleScreenSystemBackButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleScreenSystemBackButtonWasPressed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitialViewControllerWasPresented(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitialViewControllerWasPresented", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsFlowCoordinatorInHierarchy(
        &mut self,
        flowCoordinator: quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsFlowCoordinatorInHierarchy", (flowCoordinator))?;
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
    pub fn PopViewControllerFromNavigationController(
        &mut self,
        navigationController: quest_hook::libil2cpp::Gc<
            crate::HMUI::NavigationController,
        >,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
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
        Ok(__cordl_ret.into())
    }
    pub fn PopViewControllersFromNavigationController(
        &mut self,
        navigationController: quest_hook::libil2cpp::Gc<
            crate::HMUI::NavigationController,
        >,
        numberOfControllers: i32,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
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
        Ok(__cordl_ret.into())
    }
    pub fn PresentFlowCoordinator(
        &mut self,
        flowCoordinator: quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
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
        Ok(__cordl_ret.into())
    }
    pub fn PresentTitle(
        &mut self,
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PresentTitle", (title, animationType))?;
        Ok(__cordl_ret.into())
    }
    pub fn PresentViewController(
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
                "PresentViewController",
                (viewController, finishedCallback, animationDirection, immediately),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ProvideInitialViewControllers(
        &mut self,
        mainViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        leftScreenViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        rightScreenViewController: quest_hook::libil2cpp::Gc<
            crate::HMUI::ViewController,
        >,
        bottomScreenViewController: quest_hook::libil2cpp::Gc<
            crate::HMUI::ViewController,
        >,
        topScreenViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
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
        Ok(__cordl_ret.into())
    }
    pub fn PushViewControllerToNavigationController(
        &mut self,
        navigationController: quest_hook::libil2cpp::Gc<
            crate::HMUI::NavigationController,
        >,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
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
        Ok(__cordl_ret.into())
    }
    pub fn ReplaceChildFlowCoordinator(
        &mut self,
        flowCoordinator: quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
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
        Ok(__cordl_ret.into())
    }
    pub fn ReplaceTopViewController_Gc_Gc_ViewController_AnimationType_ViewController_AnimationDirection1(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        originalOwnerFlowCoordinator: quest_hook::libil2cpp::Gc<
            crate::HMUI::FlowCoordinator,
        >,
        newOwnerFlowCoordinator: quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
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
        Ok(__cordl_ret.into())
    }
    pub fn ReplaceTopViewController_ViewController_AnimationType_ViewController_AnimationDirection0(
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
                "ReplaceTopViewController",
                (viewController, finishedCallback, animationType, animationDirection),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBottomScreenViewController(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBottomScreenViewController", (viewController, animationType))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn SetLeftScreenViewController(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLeftScreenViewController", (viewController, animationType))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRightScreenViewController(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRightScreenViewController", (viewController, animationType))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTitle(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTitle", (value, animationType))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTopScreenViewController(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTopScreenViewController", (viewController, animationType))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetViewControllerToNavigationController(
        &mut self,
        navigationController: quest_hook::libil2cpp::Gc<
            crate::HMUI::NavigationController,
        >,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetViewControllerToNavigationController",
                (navigationController, viewController),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetViewControllersToNavigationController(
        &mut self,
        navigationController: quest_hook::libil2cpp::Gc<
            crate::HMUI::NavigationController,
        >,
        viewControllers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn TopViewControllerWillChange(
        &mut self,
        oldViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        newViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
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
        Ok(__cordl_ret.into())
    }
    pub fn TransitionDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TransitionDidFinish", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TransitionDidStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TransitionDidStart", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn YoungestChildFlowCoordinatorOrSelf(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator> = __cordl_object
            .invoke("YoungestChildFlowCoordinatorOrSelf", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn __ExternalActivate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("__ExternalActivate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn __ExternalDeactivate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("__ExternalDeactivate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn __StartOnScreenSystem(
        &mut self,
        screenSystem: quest_hook::libil2cpp::Gc<crate::HMUI::ScreenSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("__StartOnScreenSystem", (screenSystem))?;
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
    pub fn get_childFlowCoordinator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator> = __cordl_object
            .invoke("get_childFlowCoordinator", ())?;
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
    pub fn get_rightScreenViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController> = __cordl_object
            .invoke("get_rightScreenViewController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_showBackButton(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_showBackButton", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_title(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_title", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_topViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController> = __cordl_object
            .invoke("get_topViewController", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
