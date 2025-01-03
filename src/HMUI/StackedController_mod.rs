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
        viewControllers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<*mut crate::HMUI::ViewController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LayoutViewControllers", (viewControllers))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PopViewController(
        &mut self,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopViewController", (finishedCallback, immediately))?;
        Ok(__cordl_ret.into())
    }
    pub fn PopViewControllers(
        &mut self,
        numberOfViewControllersToPop: i32,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
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
        Ok(__cordl_ret.into())
    }
    pub fn PushViewController(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
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
        Ok(__cordl_ret.into())
    }
    pub fn SetupViewControllerRect(
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetupViewControllerRect", (viewController, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn _PushViewController_g__AnimationLayouter_3_0(
        t: f32,
        viewControllers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::HMUI::ViewController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "<PushViewController>g__AnimationLayouter|3_0",
                (t, viewControllers),
            )?;
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
    pub fn get_topStackedViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController> = __cordl_object
            .invoke("get_topStackedViewController", ())?;
        Ok(__cordl_ret.into())
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
