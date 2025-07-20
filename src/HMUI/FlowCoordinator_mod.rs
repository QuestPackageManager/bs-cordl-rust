#[cfg(feature = "HMUI+FlowCoordinator")]
#[repr(C)]
#[derive(Debug)]
pub struct FlowCoordinator {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _baseInputModule: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::BaseInputModule,
    >,
    pub _screenSystem: quest_hook::libil2cpp::Gc<crate::HMUI::ScreenSystem>,
    pub _parentFlowCoordinator: quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
    pub _childFlowCoordinator: quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
    pub _mainScreenViewControllers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        >,
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
unsafe impl quest_hook::libil2cpp::Type for crate::HMUI::FlowCoordinator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HMUI";
    const CLASS_NAME: &'static str = "FlowCoordinator";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
    pub fn Activate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool, bool, bool),
                quest_hook::libil2cpp::Void,
                3usize,
            >("Activate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "Activate", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (firstActivation, addedToHierarchy, screenSystemEnabling),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BackButtonWasPressed(
        &mut self,
        topViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("BackButtonWasPressed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "BackButtonWasPressed", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (topViewController))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CanPressBackButton(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("CanPressBackButton")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "CanPressBackButton", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Deactivate(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Deactivate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "Deactivate", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (removedFromHierarchy, screenSystemDisabling))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool, bool, bool),
                quest_hook::libil2cpp::Void,
                3usize,
            >("DidActivate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "DidActivate", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (firstActivation, addedToHierarchy, screenSystemEnabling),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("DidDeactivate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "DidDeactivate", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (removedFromHierarchy, screenSystemDisabling))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DismissChildFlowCoordinatorsRecursively(
        &mut self,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                1usize,
            >("DismissChildFlowCoordinatorsRecursively")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "DismissChildFlowCoordinatorsRecursively", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe { method.invoke_unchecked(self, (immediately))? };
        Ok(__cordl_ret.into())
    }
    pub fn DismissChildViewControllersRecursively(
        &mut self,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                1usize,
            >("DismissChildViewControllersRecursively")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "DismissChildViewControllersRecursively", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe { method.invoke_unchecked(self, (immediately))? };
        Ok(__cordl_ret.into())
    }
    pub fn DismissFlowCoordinator(
        &mut self,
        flowCoordinator: quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
        animationDirection: crate::HMUI::ViewController_AnimationDirection,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
                    crate::HMUI::ViewController_AnimationDirection,
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("DismissFlowCoordinator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "DismissFlowCoordinator", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (flowCoordinator, animationDirection, finishedCallback, immediately),
                )?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    crate::HMUI::ViewController_AnimationDirection,
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                4usize,
            >("DismissViewController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "DismissViewController", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (viewController, animationDirection, finishedCallback, immediately),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleScreenSystemBackButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleScreenSystemBackButtonWasPressed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "HandleScreenSystemBackButtonWasPressed", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitialViewControllerWasPresented(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("InitialViewControllerWasPresented")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "InitialViewControllerWasPresented", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsFlowCoordinatorInHierarchy(
        &mut self,
        flowCoordinator: quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>),
                bool,
                1usize,
            >("IsFlowCoordinatorInHierarchy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "IsFlowCoordinatorInHierarchy", 1usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (flowCoordinator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Log(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Log")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "Log", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (message))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::NavigationController>,
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("PopViewControllerFromNavigationController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "PopViewControllerFromNavigationController", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (navigationController, finishedCallback, immediately),
                )?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::NavigationController>,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("PopViewControllersFromNavigationController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "PopViewControllersFromNavigationController", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        navigationController,
                        numberOfControllers,
                        finishedCallback,
                        immediately,
                    ),
                )?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                    crate::HMUI::ViewController_AnimationDirection,
                    bool,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("PresentFlowCoordinator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "PresentFlowCoordinator", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        flowCoordinator,
                        finishedCallback,
                        animationDirection,
                        immediately,
                        replaceTopViewController,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PresentTitle(
        &mut self,
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::HMUI::ViewController_AnimationType,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("PresentTitle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "PresentTitle", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (title, animationType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PresentViewController(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        animationDirection: crate::HMUI::ViewController_AnimationDirection,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                    crate::HMUI::ViewController_AnimationDirection,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("PresentViewController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "PresentViewController", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (viewController, finishedCallback, animationDirection, immediately),
                )?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("ProvideInitialViewControllers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "ProvideInitialViewControllers", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        mainViewController,
                        leftScreenViewController,
                        rightScreenViewController,
                        bottomScreenViewController,
                        topScreenViewController,
                    ),
                )?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::NavigationController>,
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("PushViewControllerToNavigationController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "PushViewControllerToNavigationController", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (navigationController, viewController, finishedCallback, immediately),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReplaceChildFlowCoordinator(
        &mut self,
        flowCoordinator: quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        animationDirection: crate::HMUI::ViewController_AnimationDirection,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                    crate::HMUI::ViewController_AnimationDirection,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("ReplaceChildFlowCoordinator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "ReplaceChildFlowCoordinator", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (flowCoordinator, finishedCallback, animationDirection, immediately),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReplaceTopViewController_Action_ViewController_AnimationType_ViewController_AnimationDirection0(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        animationType: crate::HMUI::ViewController_AnimationType,
        animationDirection: crate::HMUI::ViewController_AnimationDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                    crate::HMUI::ViewController_AnimationType,
                    crate::HMUI::ViewController_AnimationDirection,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("ReplaceTopViewController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "ReplaceTopViewController", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (viewController, finishedCallback, animationType, animationDirection),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReplaceTopViewController_FlowCoordinator_FlowCoordinator_Action_ViewController_AnimationType_ViewController_AnimationDirection1(
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
                    quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                    crate::HMUI::ViewController_AnimationType,
                    crate::HMUI::ViewController_AnimationDirection,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("ReplaceTopViewController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "ReplaceTopViewController", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        viewController,
                        originalOwnerFlowCoordinator,
                        newOwnerFlowCoordinator,
                        finishedCallback,
                        animationType,
                        animationDirection,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBottomScreenViewController(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    crate::HMUI::ViewController_AnimationType,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetBottomScreenViewController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "SetBottomScreenViewController", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (viewController, animationType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalUserInteraction(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetGlobalUserInteraction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "SetGlobalUserInteraction", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetLeftScreenViewController(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    crate::HMUI::ViewController_AnimationType,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetLeftScreenViewController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "SetLeftScreenViewController", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (viewController, animationType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRightScreenViewController(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    crate::HMUI::ViewController_AnimationType,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetRightScreenViewController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "SetRightScreenViewController", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (viewController, animationType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTitle(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::HMUI::ViewController_AnimationType,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetTitle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "SetTitle", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value, animationType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTopScreenViewController(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    crate::HMUI::ViewController_AnimationType,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetTopScreenViewController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "SetTopScreenViewController", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (viewController, animationType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetViewControllerToNavigationController(
        &mut self,
        navigationController: quest_hook::libil2cpp::Gc<
            crate::HMUI::NavigationController,
        >,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::NavigationController>,
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetViewControllerToNavigationController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "SetViewControllerToNavigationController", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (navigationController, viewController))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::NavigationController>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetViewControllersToNavigationController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "SetViewControllersToNavigationController", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (navigationController, viewControllers))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TopViewControllerWillChange(
        &mut self,
        oldViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        newViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    crate::HMUI::ViewController_AnimationType,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("TopViewControllerWillChange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "TopViewControllerWillChange", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (oldViewController, newViewController, animationType),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TransitionDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("TransitionDidFinish")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "TransitionDidFinish", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TransitionDidStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("TransitionDidStart")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "TransitionDidStart", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn YoungestChildFlowCoordinatorOrSelf(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
                0usize,
            >("YoungestChildFlowCoordinatorOrSelf")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "YoungestChildFlowCoordinatorOrSelf", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn __ExternalActivate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("__ExternalActivate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "__ExternalActivate", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn __ExternalDeactivate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("__ExternalDeactivate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "__ExternalDeactivate", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn __StartOnScreenSystem(
        &mut self,
        screenSystem: quest_hook::libil2cpp::Gc<crate::HMUI::ScreenSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::HMUI::ScreenSystem>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("__StartOnScreenSystem")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "__StartOnScreenSystem", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (screenSystem))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_childFlowCoordinator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
                0usize,
            >("get_childFlowCoordinator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "get_childFlowCoordinator", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_isActivated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isActivated")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "get_isActivated", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isInTransition(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isInTransition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "get_isInTransition", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_rightScreenViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                0usize,
            >("get_rightScreenViewController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "get_rightScreenViewController", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_showBackButton(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_showBackButton")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "get_showBackButton", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_title(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_title")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "get_title", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_topViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                0usize,
            >("get_topViewController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "get_topViewController", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_showBackButton(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_showBackButton")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::HMUI::FlowCoordinator as quest_hook::libil2cpp::Type >
                    ::class(), "set_showBackButton", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
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
