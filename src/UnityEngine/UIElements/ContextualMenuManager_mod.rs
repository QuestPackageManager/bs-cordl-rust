#[cfg(feature = "UnityEngine+UIElements+ContextualMenuManager")]
#[repr(C)]
#[derive(Debug)]
pub struct ContextualMenuManager {
    __cordl_parent: crate::System::Object,
    pub _displayMenuHandledOSX_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+UIElements+ContextualMenuManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ContextualMenuManager
    => "UnityEngine.UIElements"."ContextualMenuManager"
);
#[cfg(feature = "UnityEngine+UIElements+ContextualMenuManager")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ContextualMenuManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ContextualMenuManager")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ContextualMenuManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ContextualMenuManager")]
impl crate::UnityEngine::UIElements::ContextualMenuManager {
    pub fn get_displayMenuHandledOSX(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_displayMenuHandledOSX", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_displayMenuHandledOSX(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_displayMenuHandledOSX", (value))?;
        Ok(__cordl_ret)
    }
    pub fn DisplayMenu(
        &mut self,
        triggerEvent: *mut crate::UnityEngine::UIElements::EventBase,
        target: *mut crate::UnityEngine::UIElements::IEventHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisplayMenu", (triggerEvent, target))?;
        Ok(__cordl_ret)
    }
    pub fn DisplayMenuIfEventMatches(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::EventBase,
        eventHandler: *mut crate::UnityEngine::UIElements::IEventHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisplayMenuIfEventMatches", (evt, eventHandler))?;
        Ok(__cordl_ret)
    }
    pub fn DoDisplayMenu(
        &mut self,
        menu: *mut crate::UnityEngine::UIElements::DropdownMenu,
        triggerEvent: *mut crate::UnityEngine::UIElements::EventBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoDisplayMenu", (menu, triggerEvent))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+ContextualMenuManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ContextualMenuManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
