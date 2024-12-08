#[cfg(feature = "HMUI+ContainerViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct ContainerViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _controllersContainer: *mut crate::UnityEngine::RectTransform,
    pub _viewControllers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HMUI::ViewController,
    >,
}
#[cfg(feature = "HMUI+ContainerViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ContainerViewController => "HMUI"
    ."ContainerViewController"
);
#[cfg(feature = "HMUI+ContainerViewController")]
impl std::ops::Deref for crate::HMUI::ContainerViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ContainerViewController")]
impl std::ops::DerefMut for crate::HMUI::ContainerViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ContainerViewController")]
impl crate::HMUI::ContainerViewController {
    #[cfg(feature = "HMUI+ContainerViewController+_AddViewControllerCoroutine_d__15")]
    pub type _AddViewControllerCoroutine_d__15 = crate::HMUI::ContainerViewController__AddViewControllerCoroutine_d__15;
    #[cfg(
        feature = "HMUI+ContainerViewController+_RemoveViewControllersCoroutine_d__17"
    )]
    pub type _RemoveViewControllersCoroutine_d__17 = crate::HMUI::ContainerViewController__RemoveViewControllersCoroutine_d__17;
    pub fn AddViewController(
        &mut self,
        viewController: *mut crate::HMUI::ViewController,
        finishedCallback: *mut crate::System::Action,
        animationLayouter: *mut crate::System::Action_2<
            f32,
            *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::HMUI::ViewController>,
        >,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddViewController",
                (viewController, finishedCallback, animationLayouter, immediately),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddViewControllerCoroutine(
        &mut self,
        newViewController: *mut crate::HMUI::ViewController,
        finishedCallback: *mut crate::System::Action,
        animationLayouter: *mut crate::System::Action_2<
            f32,
            *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::HMUI::ViewController>,
        >,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke(
                "AddViewControllerCoroutine",
                (newViewController, finishedCallback, animationLayouter, immediately),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ClearChildViewControllers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearChildViewControllers", ())?;
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
    pub fn GetNewXPositionsForViewControllers(
        &mut self,
        viewControllers: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HMUI::ViewController,
        >,
        exludeFromEndCount: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<f32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<f32> = __cordl_object
            .invoke(
                "GetNewXPositionsForViewControllers",
                (viewControllers, exludeFromEndCount),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LayoutViewControllers(
        &mut self,
        viewControllers: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HMUI::ViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LayoutViewControllers", (viewControllers))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RemoveViewControllers(
        &mut self,
        viewControllers: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::HMUI::ViewController,
        >,
        finishedCallback: *mut crate::System::Action,
        animationLayouter: *mut crate::System::Action_3<
            f32,
            *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::HMUI::ViewController>,
            *mut crate::System::Collections::Generic::HashSet_1<
                *mut crate::HMUI::ViewController,
            >,
        >,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RemoveViewControllers",
                (viewControllers, finishedCallback, animationLayouter, immediately),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RemoveViewControllersCoroutine(
        &mut self,
        viewControllersToRemove: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::HMUI::ViewController,
        >,
        finishedCallback: *mut crate::System::Action,
        animationLayouter: *mut crate::System::Action_3<
            f32,
            *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::HMUI::ViewController>,
            *mut crate::System::Collections::Generic::HashSet_1<
                *mut crate::HMUI::ViewController,
            >,
        >,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke(
                "RemoveViewControllersCoroutine",
                (
                    viewControllersToRemove,
                    finishedCallback,
                    animationLayouter,
                    immediately,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetChildViewController(
        &mut self,
        viewController: *mut crate::HMUI::ViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetChildViewController", (viewController))?;
        Ok(__cordl_ret)
    }
    pub fn SetChildViewControllers(
        &mut self,
        viewControllers: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::HMUI::ViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetChildViewControllers", (viewControllers))?;
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
    pub fn get_controllersContainer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::RectTransform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::RectTransform = __cordl_object
            .invoke("get_controllersContainer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_viewControllers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::HMUI::ViewController,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HMUI::ViewController,
        > = __cordl_object.invoke("get_viewControllers", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HMUI+ContainerViewController")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::ContainerViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
