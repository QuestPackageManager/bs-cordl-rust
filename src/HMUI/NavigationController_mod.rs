#[cfg(feature = "HMUI+NavigationController+Alignment")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationController_Alignment {
    Beginning = 0i32,
    End = 2i32,
    Middle = 1i32,
}
#[cfg(feature = "HMUI+NavigationController+Alignment")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::NavigationController_Alignment => "HMUI"
    ."NavigationController/Alignment"
);
#[cfg(feature = "HMUI+NavigationController")]
#[repr(C)]
#[derive(Debug)]
pub struct NavigationController {
    __cordl_parent: crate::HMUI::ContainerViewController,
    pub _orientation: crate::HMUI::NavigationController_Orientation,
    pub _reversedStacking: bool,
    pub _alignment: crate::HMUI::NavigationController_Alignment,
    pub _edgeSize: f32,
    pub _viewControllersSeparator: f32,
}
#[cfg(feature = "HMUI+NavigationController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::NavigationController => "HMUI"
    ."NavigationController"
);
#[cfg(feature = "HMUI+NavigationController")]
impl std::ops::Deref for crate::HMUI::NavigationController {
    type Target = crate::HMUI::ContainerViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+NavigationController")]
impl std::ops::DerefMut for crate::HMUI::NavigationController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+NavigationController")]
impl crate::HMUI::NavigationController {
    #[cfg(feature = "HMUI+NavigationController+Orientation")]
    pub type Orientation = crate::HMUI::NavigationController_Orientation;
    #[cfg(feature = "HMUI+NavigationController+__c__DisplayClass10_0")]
    pub type __c__DisplayClass10_0 = crate::HMUI::NavigationController___c__DisplayClass10_0;
    #[cfg(feature = "HMUI+NavigationController+__c__DisplayClass8_0")]
    pub type __c__DisplayClass8_0 = crate::HMUI::NavigationController___c__DisplayClass8_0;
    #[cfg(feature = "HMUI+NavigationController+Alignment")]
    pub type Alignment = crate::HMUI::NavigationController_Alignment;
    pub fn GetNewPositionsForViewControllers(
        &mut self,
        viewControllers: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HMUI::ViewController,
        >,
        fixedViewControllers: *mut crate::System::Collections::Generic::HashSet_1<
            *mut crate::HMUI::ViewController,
        >,
        fixedEndPos: f32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<f32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<f32> = __cordl_object
            .invoke(
                "GetNewPositionsForViewControllers",
                (viewControllers, fixedViewControllers, fixedEndPos),
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
    pub fn PopViewController(
        &mut self,
        finishedCallback: *mut crate::System::Action,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopViewController", (finishedCallback, immediately))?;
        Ok(__cordl_ret)
    }
    pub fn PopViewControllers(
        &mut self,
        numberOfViewControllersToPop: i32,
        finishedCallback: *mut crate::System::Action,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "PopViewControllers",
                (numberOfViewControllersToPop, finishedCallback, immediately),
            )?;
        Ok(__cordl_ret)
    }
    pub fn PositionVector(
        &mut self,
        pos: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("PositionVector", (pos))?;
        Ok(__cordl_ret)
    }
    pub fn PushViewController(
        &mut self,
        viewController: *mut crate::HMUI::ViewController,
        finishedCallback: *mut crate::System::Action,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "PushViewController",
                (viewController, finishedCallback, immediately),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetupViewControllerRect(
        &mut self,
        viewController: *mut crate::HMUI::ViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupViewControllerRect", (viewController))?;
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
#[cfg(feature = "HMUI+NavigationController")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::NavigationController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+NavigationController+Orientation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationController_Orientation {
    Horizontal = 0i32,
    Vertical = 1i32,
}
#[cfg(feature = "HMUI+NavigationController+Orientation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::NavigationController_Orientation => "HMUI"
    ."NavigationController/Orientation"
);
