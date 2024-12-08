#[cfg(feature = "HMUI+StackedController")]
#[repr(C)]
#[derive(Debug)]
pub struct StackedController {
    __cordl_parent: crate::HMUI::ContainerViewController,
}
#[cfg(feature = "HMUI+StackedController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::StackedController => "HMUI"
    ."StackedController"
);
#[cfg(feature = "HMUI+StackedController")]
impl std::ops::Deref for crate::HMUI::StackedController {
    type Target = crate::HMUI::ContainerViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+StackedController")]
impl std::ops::DerefMut for crate::HMUI::StackedController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+StackedController")]
impl crate::HMUI::StackedController {
    #[cfg(feature = "HMUI+StackedController+__c__DisplayClass5_0")]
    pub type __c__DisplayClass5_0 = crate::HMUI::StackedController___c__DisplayClass5_0;
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
    pub fn get_topStackedViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::ViewController> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::ViewController = __cordl_object
            .invoke("get_topStackedViewController", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HMUI+StackedController")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::StackedController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}