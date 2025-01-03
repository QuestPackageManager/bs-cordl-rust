#[cfg(feature = "UnityEngine+UIElements+ContextualMenuPopulateEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct ContextualMenuPopulateEvent {
    __cordl_parent: crate::UnityEngine::UIElements::MouseEventBase_1<
        *mut crate::UnityEngine::UIElements::ContextualMenuPopulateEvent,
    >,
    pub _menu_k__BackingField: *mut crate::UnityEngine::UIElements::DropdownMenu,
    pub _triggerEvent_k__BackingField: *mut crate::UnityEngine::UIElements::EventBase,
    pub m_ContextualMenuManager: *mut crate::UnityEngine::UIElements::ContextualMenuManager,
}
#[cfg(feature = "UnityEngine+UIElements+ContextualMenuPopulateEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::ContextualMenuPopulateEvent => "UnityEngine.UIElements"
    ."ContextualMenuPopulateEvent"
);
#[cfg(feature = "UnityEngine+UIElements+ContextualMenuPopulateEvent")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ContextualMenuPopulateEvent {
    type Target = crate::UnityEngine::UIElements::MouseEventBase_1<
        *mut crate::UnityEngine::UIElements::ContextualMenuPopulateEvent,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ContextualMenuPopulateEvent")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ContextualMenuPopulateEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ContextualMenuPopulateEvent")]
impl crate::UnityEngine::UIElements::ContextualMenuPopulateEvent {
    #[cfg(feature = "UnityEngine+UIElements+ContextualMenuPopulateEvent+__c")]
    pub type __c = crate::UnityEngine::UIElements::ContextualMenuPopulateEvent___c;
    pub fn GetPooled(
        triggerEvent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventBase,
        >,
        menu: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DropdownMenu>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IEventHandler>,
        menuManager: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ContextualMenuManager,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ContextualMenuPopulateEvent,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ContextualMenuPopulateEvent,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPooled", (triggerEvent, menu, target, menuManager))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LocalInit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LocalInit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PostDispatch(
        &mut self,
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PostDispatch", (panel))?;
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
    pub fn get_menu(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DropdownMenu>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::DropdownMenu,
        > = __cordl_object.invoke("get_menu", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_triggerEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventBase,
        > = __cordl_object.invoke("get_triggerEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_menu(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DropdownMenu>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_menu", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_triggerEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_triggerEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+ContextualMenuPopulateEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ContextualMenuPopulateEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
