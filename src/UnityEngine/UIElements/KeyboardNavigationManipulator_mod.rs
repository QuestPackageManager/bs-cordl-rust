#[cfg(feature = "UnityEngine+UIElements+KeyboardNavigationManipulator")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyboardNavigationManipulator {
    __cordl_parent: crate::UnityEngine::UIElements::Manipulator,
    pub m_Action: *mut crate::System::Action_2<
        crate::UnityEngine::UIElements::KeyboardNavigationOperation,
        *mut crate::UnityEngine::UIElements::EventBase,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+KeyboardNavigationManipulator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::KeyboardNavigationManipulator => "UnityEngine.UIElements"
    ."KeyboardNavigationManipulator"
);
#[cfg(feature = "UnityEngine+UIElements+KeyboardNavigationManipulator")]
impl std::ops::Deref for crate::UnityEngine::UIElements::KeyboardNavigationManipulator {
    type Target = crate::UnityEngine::UIElements::Manipulator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+KeyboardNavigationManipulator")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::KeyboardNavigationManipulator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+KeyboardNavigationManipulator")]
impl crate::UnityEngine::UIElements::KeyboardNavigationManipulator {
    #[cfg(
        feature = "UnityEngine+UIElements+KeyboardNavigationManipulator+__c__DisplayClass4_0"
    )]
    pub type __c__DisplayClass4_0 = crate::UnityEngine::UIElements::KeyboardNavigationManipulator___c__DisplayClass4_0;
    pub fn Invoke(
        &mut self,
        operation: crate::UnityEngine::UIElements::KeyboardNavigationOperation,
        evt: *mut crate::UnityEngine::UIElements::EventBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (operation, evt))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        action: *mut crate::System::Action_2<
            crate::UnityEngine::UIElements::KeyboardNavigationOperation,
            *mut crate::UnityEngine::UIElements::EventBase,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (action))?;
        Ok(__cordl_object)
    }
    pub fn OnKeyDown(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::KeyDownEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnKeyDown", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnNavigationCancel(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::NavigationCancelEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNavigationCancel", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnNavigationMove(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::NavigationMoveEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNavigationMove", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnNavigationSubmit(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::NavigationSubmitEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNavigationSubmit", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterCallbacksOnTarget(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallbacksOnTarget", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnregisterCallbacksFromTarget(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterCallbacksFromTarget", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        action: *mut crate::System::Action_2<
            crate::UnityEngine::UIElements::KeyboardNavigationOperation,
            *mut crate::UnityEngine::UIElements::EventBase,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (action))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+KeyboardNavigationManipulator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::KeyboardNavigationManipulator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
